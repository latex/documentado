use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Focus, Mode};
use crate::docs::DocContent;

pub fn draw(frame: &mut Frame, app: &App) {
    let [main, bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
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
    draw_search_bar(frame, app, bottom);

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
                ("\u{21BB} ", Color::Yellow)
            } else if app.source_items_loaded[i] {
                ("\u{2713} ", Color::Green)
            } else {
                ("  ", Color::DarkGray)
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
        .title(" Sources ")
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
        .title(" Items ")
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
        .title(" Documentation ")
        .borders(Borders::ALL)
        .style(border_style);

    let text = match &app.doc_content {
        Some(DocContent::Loading) => {
            Text::from(Line::from(Span::styled(
                " Loading...",
                Style::default().fg(Color::Gray),
            )))
        }
        Some(DocContent::Error(e)) => Text::from(Line::from(Span::styled(
            format!(" Error: {}", e),
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
            " Select an item to view documentation",
            Style::default().fg(Color::Gray),
        ))),
    };

    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn draw_search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let [search_area, status_area] =
        Layout::horizontal([Constraint::Fill(1), Constraint::Length(50)]).areas(area);

    let search_text = if matches!(app.mode, Mode::Searching) {
        format!(" Search: {}█", app.search_input)
    } else {
        format!(" Search: {}", app.search_input)
    };

    let search_style = if matches!(app.mode, Mode::Searching) {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let search_block = Block::default()
        .borders(Borders::ALL)
        .style(search_style);

    frame.render_widget(
        Paragraph::new(Line::from(Span::raw(search_text)))
            .block(search_block)
            .wrap(Wrap { trim: false }),
        search_area,
    );

    let status_text = format!(
        " {} items | thanks to Kapeli, Dash & GitHub ",
        app.filtered_items.len()
    );
    let status_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(
        Paragraph::new(Line::from(Span::raw(status_text))).block(status_block),
        status_area,
    );
}

fn draw_help(frame: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(Span::styled(
            " Help ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::raw(" Tab / Ctrl+w / ← →  - Switch panel")),
        Line::from(Span::raw(" j / k / ↓ / ↑        - Move down / up")),
        Line::from(Span::raw(" Enter              - Open item / load doc")),
        Line::from(Span::raw(" d                  - Download selected source")),
        Line::from(Span::raw(" a                  - Add source from catalog")),
        Line::from(Span::raw(" / or Ctrl+f        - Start search")),
        Line::from(Span::raw(" Esc                - Exit search / go back")),
        Line::from(Span::raw(" o                  - Open doc in vim")),
        Line::from(Span::raw(" q / Ctrl+c         - Quit")),
        Line::from(Span::raw(" ?                  - Toggle this help")),
        Line::from(""),
        Line::from(Span::styled(
            " Press ? again to close ",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(Text::from(help_text))
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    let popup_area = centered_rect(area, 50, 50);
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
            let prefix = if exists { "✓ " } else { "  " };
            ListItem::new(Line::from(vec![
                Span::styled(prefix, Style::default().fg(if exists { Color::Green } else { Color::DarkGray })),
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
        .title(format!(" Add Source ({} available, {} added) ", count, app.sources.len()))
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
        " Enter: add  |  Esc/q/a: close ",
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
