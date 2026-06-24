use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Focus, Mode};
use crate::docs::DocContent;

// Nerd Font / Font Awesome glyphs (use with a Nerd Font patched font)
const ICON_SEARCH: &str = "\u{F002}";    // 
const ICON_CHECK: &str = "\u{F00C}";     // 
const ICON_CLOSE: &str = "\u{F00D}";     // 
const ICON_PLUS: &str = "\u{F067}";      // 
const ICON_DOWNLOAD: &str = "\u{F01A}";  // 
const ICON_BOOK: &str = "\u{F02D}";      // 
const ICON_FILE: &str = "\u{F15B}";      // 
const ICON_FOLDER: &str = "\u{F07B}";    // 
const ICON_REFRESH: &str = "\u{F021}";   // 
const ICON_HELP: &str = "\u{F059}";      // 
const ICON_CUBES: &str = "\u{F1B3}";     // 
const ICON_KEYBOARD: &str = "\u{F11C}";  // 

pub fn draw(frame: &mut Frame, app: &App) {
    let [main, bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(2),
    ])
    .areas(frame.area());

    let [sources_area, items_area, content_area] = Layout::horizontal([
        Constraint::Length(28),
        Constraint::Length(34),
        Constraint::Fill(1),
    ])
    .areas(main);

    draw_sources(frame, app, sources_area);
    draw_items(frame, app, items_area);
    draw_content(frame, app, content_area);

    let [search_line, status_line] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(bottom);

    draw_search_line(frame, app, search_line);
    draw_status_line(frame, app, status_line);

    if matches!(app.mode, Mode::Help) {
        draw_help(frame, frame.area());
    } else if matches!(app.mode, Mode::Discover) {
        draw_discover(frame, frame.area(), app);
    }
}

fn draw_sources(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .sources
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let (prefix, prefix_color) = if app.source_items_loading[i] {
                (ICON_REFRESH, Color::Yellow)
            } else if app.source_items_loaded[i] {
                (ICON_CHECK, Color::Green)
            } else {
                (ICON_FOLDER, Color::DarkGray)
            };
            let style = if i == app.selected_source {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(prefix, Style::default().fg(prefix_color)),
                Span::raw(" "),
                Span::styled(&s.name, style),
            ]))
        })
        .collect();

    let border_style = if matches!(app.focus, Focus::Sources) {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(format!(" {} Sources ", ICON_BOOK))
        .borders(Borders::ALL)
        .style(border_style);

    frame.render_widget(
        List::new(items)
            .block(block)
            .highlight_style(Style::default()),
        area,
    );
}

fn draw_items(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let name_style = if i == app.selected_item {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let type_color = match item.item_type.as_str() {
                "struct" => Color::LightRed,
                "enum" => Color::LightYellow,
                "trait" => Color::LightGreen,
                "fn" => Color::LightCyan,
                "mod" => Color::LightBlue,
                "macro" => Color::LightMagenta,
                "type" => Color::LightRed,
                _ => Color::DarkGray,
            };
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("[{:7}]", item.item_type),
                    Style::default().fg(type_color),
                ),
                Span::styled(&item.name, name_style),
            ]))
        })
        .collect();

    let border_style = if matches!(app.focus, Focus::Items) {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(format!(" {} Items ", ICON_CUBES))
        .borders(Borders::ALL)
        .style(border_style);

    frame.render_widget(
        List::new(items)
            .block(block)
            .highlight_style(Style::default()),
        area,
    );
}

fn draw_content(frame: &mut Frame, app: &App, area: Rect) {
    let border_style = if matches!(app.focus, Focus::Content) {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(format!(" {} Documentation ", ICON_FILE))
        .borders(Borders::ALL)
        .style(border_style);

    let text = match &app.doc_content {
        Some(DocContent::Loading) => {
            Text::from(Line::from(Span::styled(
                format!(" {} Loading...", ICON_REFRESH),
                Style::default().fg(Color::Gray),
            )))
        }
        Some(DocContent::Error(e)) => Text::from(Line::from(Span::styled(
            format!(" {} Error: {}", ICON_CLOSE, e),
            Style::default().fg(Color::Red),
        ))),
        Some(DocContent::Loaded(s)) => {
            let lines: Vec<Line> = s
                .lines()
                .skip(app.content_scroll)
                .map(|line| {
                    let trimmed = line.trim();
                    if trimmed.starts_with("```") {
                        Line::from(Span::styled(
                            trimmed,
                            Style::default().fg(Color::Green),
                        ))
                    } else if trimmed.starts_with('#') {
                        Line::from(Span::styled(
                            trimmed,
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                    } else if trimmed.starts_with("  •") {
                        Line::from(Span::styled(
                            trimmed,
                            Style::default().fg(Color::White),
                        ))
                    } else {
                        Line::from(Span::raw(trimmed))
                    }
                })
                .collect();
            Text::from(lines)
        }
        None => Text::from(Line::from(Span::styled(
            format!(" {} {} Select an item", ICON_BOOK, ICON_KEYBOARD),
            Style::default().fg(Color::Gray),
        ))),
    };

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn draw_search_line(frame: &mut Frame, app: &App, area: Rect) {
    if matches!(app.mode, Mode::Searching) {
        let text = format!(" {} {}█", ICON_SEARCH, app.search_input);
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                text,
                Style::default().fg(Color::Yellow),
            ))),
            area,
        );
    } else {
        let text = if app.search_input.is_empty() {
            format!(" {} / or Ctrl+f", ICON_SEARCH)
        } else {
            format!(" {} {}", ICON_SEARCH, app.search_input)
        };
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                text,
                Style::default().fg(Color::DarkGray),
            ))),
            area,
        );
    }
}

fn draw_status_line(frame: &mut Frame, app: &App, area: Rect) {
    let mode_label = match app.mode {
        Mode::Normal => format!(" {} NORMAL ", ICON_KEYBOARD),
        Mode::Searching => format!(" {} SEARCH ", ICON_SEARCH),
        Mode::Help => format!(" {} HELP ", ICON_HELP),
        Mode::Discover => format!(" {} DISCOVER ", ICON_PLUS),
    };
    let mode_color = match app.mode {
        Mode::Normal => Color::Blue,
        Mode::Searching => Color::Yellow,
        Mode::Help => Color::Cyan,
        Mode::Discover => Color::Magenta,
    };

    let focus_icon = match app.focus {
        Focus::Sources => ICON_BOOK,
        Focus::Items => ICON_CUBES,
        Focus::Content => ICON_FILE,
    };
    let focus_label = match app.focus {
        Focus::Sources => "Sources",
        Focus::Items => "Items",
        Focus::Content => "Content",
    };

    let source_name = app
        .sources
        .get(app.selected_source)
        .map(|s| s.name.as_str())
        .unwrap_or("-");

    let hints = if app.focus == Focus::Sources && !app.source_items_loading.iter().any(|&x| x) {
        format!(" {}dl {}add {}help ", ICON_DOWNLOAD, ICON_PLUS, ICON_HELP)
    } else {
        format!(" {}search {}help ", ICON_SEARCH, ICON_HELP)
    };

    let item_count = format!(" {} {} ", ICON_CUBES, app.filtered_items.len());

    // Lualine-style: colored mode block, sections with separators
    let mode_span = Span::styled(
        mode_label,
        Style::default()
            .fg(Color::White)
            .bg(mode_color)
            .add_modifier(Modifier::BOLD),
    );
    let sep = Span::styled(" │ ", Style::default().fg(Color::DarkGray));
    let focus_span = Span::styled(
        format!("{} {}", focus_icon, focus_label),
        Style::default().fg(match app.focus {
            Focus::Sources => Color::Cyan,
            Focus::Items => Color::Yellow,
            Focus::Content => Color::Green,
        }),
    );
    let source_span = Span::styled(source_name, Style::default().fg(Color::White));
    let hints_span = Span::styled(&hints, Style::default().fg(Color::DarkGray));
    let count_span = Span::styled(&item_count, Style::default().fg(Color::Cyan));
    let credit_span = Span::styled(
        format!(" {} ", ICON_HELP),
        Style::default().fg(Color::DarkGray),
    );

    let line = Line::from(vec![
        mode_span,
        sep.clone(),
        focus_span,
        Span::raw(" "),
        source_span,
        sep.clone(),
        hints_span,
        sep.clone(),
        count_span,
        credit_span,
    ]);

    frame.render_widget(
        Paragraph::new(line).block(
            Block::default()
                .style(Style::default().bg(Color::Rgb(30, 30, 40))),
        ),
        area,
    );
}

fn draw_help(frame: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(Span::styled(
            format!(" {} Keyboard Shortcuts ", ICON_KEYBOARD),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::raw(format!(" {} / {} h/l/←→   Switch panel", ICON_KEYBOARD, ICON_SEARCH))),
        Line::from(Span::raw(" j/k/↓/↑        Move down / up")),
        Line::from(Span::raw(" Enter          Open item / load doc")),
        Line::from(Span::raw(format!(" {} {}dl        Download source", ICON_DOWNLOAD, ICON_KEYBOARD))),
        Line::from(Span::raw(format!(" {} {}add        Add source from catalog", ICON_PLUS, ICON_KEYBOARD))),
        Line::from(Span::raw(format!(" {} / Ctrl+f    Start search", ICON_SEARCH))),
        Line::from(Span::raw(" Esc            Exit search / go back")),
        Line::from(Span::raw(" o              Open doc in vim")),
        Line::from(Span::raw(format!(" {} / Ctrl+c    Quit", ICON_CLOSE))),
        Line::from(Span::raw(format!(" {}             Toggle this help", ICON_HELP))),
        Line::from(""),
        Line::from(Span::styled(
            format!(" Press {} again to close ", ICON_HELP),
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(Text::from(help_text))
        .block(
            Block::default()
                .title(format!(" {} Help ", ICON_HELP))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    let popup_area = centered_rect(area, 55, 55);
    frame.render_widget(paragraph, popup_area);
}

fn draw_discover(frame: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .discover_sources
        .iter()
        .enumerate()
        .map(|(i, sc)| {
            let style = if i == app.discover_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let exists = app.sources.iter().any(|s| s.url == sc.url);
            let prefix = if exists { ICON_CHECK } else { ICON_PLUS };
            ListItem::new(Line::from(vec![
                Span::styled(prefix, Style::default().fg(if exists { Color::Green } else { Color::DarkGray })),
                Span::raw(" "),
                Span::styled(&sc.name, style),
            ]))
        })
        .collect();

    let popup_area = centered_rect(area, 60, 70);

    let list = List::new(items).highlight_style(Style::default());
    let count = app.discover_sources.len();

    let para = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            " Catalog of documentation sources ",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " Inspired by Kapeli/Dash & community docsets ",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
    ]));

    let outer_block = Block::default()
        .title(format!(" {} Add Source ({} available, {} added) ", ICON_PLUS, count, app.sources.len()))
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let inner = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(popup_area);

    frame.render_widget(outer_block, popup_area);
    frame.render_widget(para, inner[0]);
    frame.render_widget(list, inner[1]);

    let hint = Line::from(Span::styled(
        format!(" {} add  |  Esc/q/a: close ", ICON_CHECK),
        Style::default().fg(Color::DarkGray),
    ));
    frame.render_widget(Paragraph::new(hint), inner[2]);
}

fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length((area.height.saturating_mul(percent_y) / 100).max(12)),
        Constraint::Fill(1),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length((area.width.saturating_mul(percent_x) / 100).max(40)),
        Constraint::Fill(1),
    ])
    .split(popup_layout[1])[1]
}
