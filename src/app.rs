use std::fs;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use nucleo::Matcher;
use nucleo::Utf32String;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio::sync::mpsc;

use crate::docs::{self, DocContent, DocItem, DocSource};
use crate::ui;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    Sources,
    Items,
    Content,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    Searching,
    Help,
}

enum BackgroundEvent {
    ModuleItemsLoaded(usize, Vec<DocItem>),
    ModuleItemsError(usize, String),
    DocContentLoaded(String),
    DocContentError(String),
}

pub struct App {
    pub sources: Vec<DocSource>,
    pub selected_source: usize,
    pub filtered_items: Vec<DocItem>,
    pub selected_item: usize,
    pub content_scroll: usize,
    pub doc_content: Option<DocContent>,
    pub search_input: String,
    pub focus: Focus,
    pub mode: Mode,

    event_tx: mpsc::Sender<BackgroundEvent>,
    event_rx: mpsc::Receiver<BackgroundEvent>,
    matcher: Matcher,
    source_items_loaded: Vec<bool>,
}

impl App {
    pub fn new(query: String) -> Result<Self> {
        let (event_tx, event_rx) = mpsc::channel(32);
        let sources = docs::default_sources();
        let source_count = sources.len();

        Ok(Self {
            sources,
            selected_source: 0,
            filtered_items: Vec::new(),
            selected_item: 0,
            content_scroll: 0,
            doc_content: None,
            search_input: query,
            focus: Focus::Items,
            mode: Mode::Normal,
            event_rx,
            event_tx,
            matcher: Matcher::new(nucleo::Config::DEFAULT),
            source_items_loaded: vec![false; source_count],
        })
    }

    fn apply_filter(&mut self) {
        if self.search_input.is_empty() {
            if let Some(source) = self.sources.get(self.selected_source) {
                self.filtered_items = source.items.clone();
            }
        } else if let Some(source) = self.sources.get(self.selected_source) {
            let query = Utf32String::from(self.search_input.as_str());
            let mut scored: Vec<(u16, &DocItem)> = source
                .items
                .iter()
                .filter_map(|item| {
                    let name = Utf32String::from(item.name.to_lowercase().as_str());
                    let typ = Utf32String::from(item.item_type.as_str());
                    self.matcher
                        .fuzzy_match(name.slice(..), query.slice(..))
                        .or_else(|| self.matcher.fuzzy_match(typ.slice(..), query.slice(..)))
                        .map(|s| (s, item))
                })
                .collect();
            scored.sort_by(|a, b| b.0.cmp(&a.0));
            self.filtered_items = scored.into_iter().map(|(_, item)| item.clone()).collect();
        }
        self.selected_item = 0;
        self.content_scroll = 0;
    }

    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>,
    ) -> Result<()> {
        let (input_tx, mut input_rx) = mpsc::channel::<Event>(32);

        // Spawn blocking thread for terminal I/O
        tokio::task::spawn_blocking(move || loop {
            if let Ok(event) = event::read() {
                if input_tx.blocking_send(event).is_err() {
                    break;
                }
            }
        });

        // Initial load: fetch items for the first source
        self.request_module_items(self.selected_source);

        loop {
            tokio::select! {
                Some(event) = input_rx.recv() => {
                    if let Event::Key(key) = event {
                        if key.kind == KeyEventKind::Press {
                            match self.mode {
                                Mode::Searching => self.handle_search_key(key),
                                Mode::Help => {
                                    if key.code == KeyCode::Char('?') {
                                        self.mode = Mode::Normal;
                                    }
                                }
                                Mode::Normal => {
                                    if key.code == KeyCode::Char('o')
                                        && self.focus == Focus::Content
                                        && self.doc_content.is_some()
                                    {
                                        self.open_in_vim(terminal).await?;
                                    } else {
                                        self.handle_normal_key(key);
                                    }
                                }
                            }
                        }
                    }
                }
                Some(bg) = self.event_rx.recv() => {
                    match bg {
                        BackgroundEvent::ModuleItemsLoaded(idx, items) => {
                            if let Some(source) = self.sources.get_mut(idx) {
                                source.items = items;
                                self.source_items_loaded[idx] = true;
                                if idx == self.selected_source {
                                    self.apply_filter();
                                }
                            }
                        }
                        BackgroundEvent::ModuleItemsError(idx, error) => {
                            eprintln!("error loading source {idx}: {error}");
                        }
                        BackgroundEvent::DocContentLoaded(text) => {
                            self.doc_content = Some(DocContent::Loaded(text));
                        }
                        BackgroundEvent::DocContentError(error) => {
                            self.doc_content = Some(DocContent::Error(error));
                        }
                    }
                }
            }

            terminal.draw(|f| ui::draw(f, self))?;
        }
    }

    fn handle_normal_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                std::process::exit(0);
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                std::process::exit(0);
            }
            KeyCode::Char('?') => {
                self.mode = Mode::Help;
            }
            KeyCode::Char('/') => {
                self.mode = Mode::Searching;
            }
            KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.mode = Mode::Searching;
            }
            KeyCode::Tab | KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.focus = match self.focus {
                    Focus::Sources => Focus::Items,
                    Focus::Items => Focus::Content,
                    Focus::Content => Focus::Sources,
                };
            }
            KeyCode::Char('1') => self.focus = Focus::Sources,
            KeyCode::Char('2') => self.focus = Focus::Items,
            KeyCode::Char('3') => self.focus = Focus::Content,
            KeyCode::Char('g') => match self.focus {
                Focus::Sources => {
                    self.selected_source = 0;
                    self.request_module_items(0);
                    if self.source_items_loaded[0] {
                        self.apply_filter();
                    } else {
                        self.filtered_items.clear();
                        self.doc_content = None;
                    }
                }
                Focus::Items => self.selected_item = 0,
                Focus::Content => self.content_scroll = 0,
            },
            KeyCode::Char('G') => match self.focus {
                Focus::Sources => {
                    let last = self.sources.len().saturating_sub(1);
                    self.selected_source = last;
                    self.request_module_items(last);
                    if self.source_items_loaded[last] {
                        self.apply_filter();
                    } else {
                        self.filtered_items.clear();
                        self.doc_content = None;
                    }
                }
                Focus::Items => self.selected_item = self.filtered_items.len().saturating_sub(1),
                Focus::Content => self.content_scroll = usize::MAX,
            },
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.focus == Focus::Content {
                    self.content_scroll = self.content_scroll.saturating_sub(10);
                }
            }
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if self.focus == Focus::Content {
                    self.content_scroll = self.content_scroll.saturating_add(10);
                }
            }
            KeyCode::Char('h') | KeyCode::Left => {
                self.focus = match self.focus {
                    Focus::Sources => Focus::Content,
                    Focus::Items => Focus::Sources,
                    Focus::Content => Focus::Items,
                };
            }
            KeyCode::Char('l') | KeyCode::Right => {
                self.focus = match self.focus {
                    Focus::Sources => Focus::Items,
                    Focus::Items => Focus::Content,
                    Focus::Content => Focus::Sources,
                };
            }
            KeyCode::Char('j') | KeyCode::Down => match self.focus {
                Focus::Sources => {
                    if self.selected_source + 1 < self.sources.len() {
                        self.selected_source += 1;
                        self.request_module_items(self.selected_source);
                        if self.source_items_loaded[self.selected_source] {
                            self.apply_filter();
                        } else {
                            self.filtered_items.clear();
                            self.doc_content = None;
                        }
                    }
                }
                Focus::Items => {
                    if self.selected_item + 1 < self.filtered_items.len() {
                        self.selected_item += 1;
                    }
                }
                Focus::Content => {
                    self.content_scroll = self.content_scroll.saturating_add(3);
                }
            },
            KeyCode::Char('k') | KeyCode::Up => match self.focus {
                Focus::Sources => {
                    if self.selected_source > 0 {
                        self.selected_source -= 1;
                        self.request_module_items(self.selected_source);
                        if self.source_items_loaded[self.selected_source] {
                            self.apply_filter();
                        } else {
                            self.filtered_items.clear();
                            self.doc_content = None;
                        }
                    }
                }
                Focus::Items => {
                    self.selected_item = self.selected_item.saturating_sub(1);
                }
                Focus::Content => {
                    self.content_scroll = self.content_scroll.saturating_sub(3);
                }
            },
            KeyCode::Enter => {
                if self.focus == Focus::Items {
                    if let Some(item) = self.filtered_items.get(self.selected_item) {
                        self.doc_content = Some(DocContent::Loading);
                        self.content_scroll = 0;
                        self.request_doc_content(item.url.clone());
                    }
                } else if self.focus == Focus::Sources {
                    if self.source_items_loaded[self.selected_source] {
                        self.apply_filter();
                    }
                    self.focus = Focus::Items;
                }
            }
            KeyCode::Esc => {
                if !self.search_input.is_empty() {
                    self.search_input.clear();
                    self.apply_filter();
                }
            }
            _ => {}
        }
    }

    fn handle_search_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char(c) => {
                self.search_input.push(c);
                self.apply_filter();
            }
            KeyCode::Backspace => {
                self.search_input.pop();
                self.apply_filter();
            }
            KeyCode::Esc | KeyCode::Enter => {
                self.mode = Mode::Normal;
            }
            _ => {}
        }
    }

    fn request_module_items(&self, source_idx: usize) {
        if source_idx >= self.sources.len() || self.source_items_loaded[source_idx] {
            return;
        }
        let source_url = self.sources[source_idx].url.clone();
        let tx = self.event_tx.clone();
        tokio::spawn(async move {
            match docs::fetch_module_items(&source_url).await {
                Ok(items) => {
                    let _ = tx
                        .send(BackgroundEvent::ModuleItemsLoaded(source_idx, items))
                        .await;
                }
                Err(e) => {
                    let _ = tx
                        .send(BackgroundEvent::ModuleItemsError(
                            source_idx,
                            e.to_string(),
                        ))
                        .await;
                }
            }
        });
    }

    fn request_doc_content(&self, url: String) {
        let tx = self.event_tx.clone();
        tokio::spawn(async move {
            match docs::fetch_item_content(&url).await {
                Ok(text) => {
                    let _ = tx.send(BackgroundEvent::DocContentLoaded(text)).await;
                }
                Err(e) => {
                    let _ = tx
                        .send(BackgroundEvent::DocContentError(e.to_string()))
                        .await;
                }
            }
        });
    }

    async fn open_in_vim(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>,
    ) -> Result<()> {
        let text = match &self.doc_content {
            Some(DocContent::Loaded(t)) => t.clone(),
            _ => return Ok(()),
        };

        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("documentado_preview.md");
        fs::write(&file_path, &text)?;

        let editor = std::env::var("EDITOR")
            .or_else(|_| std::env::var("VISUAL"))
            .unwrap_or_else(|_| "vim".to_string());

        terminal.clear()?;
        crossterm::execute!(
            std::io::stderr(),
            crossterm::terminal::LeaveAlternateScreen
        )?;
        crossterm::terminal::disable_raw_mode()?;

        tokio::process::Command::new(&editor)
            .arg(&file_path)
            .status()
            .await?;

        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            std::io::stderr(),
            crossterm::terminal::EnterAlternateScreen
        )?;
        terminal.clear()?;

        Ok(())
    }
}
