use anyhow::Result;
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct DocSource {
    pub name: String,
    pub url: String,
    pub source_type: String,
    pub items: Vec<DocItem>,
}

#[derive(Debug, Clone)]
pub struct DocItem {
    pub name: String,
    pub item_type: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub enum DocContent {
    Loading,
    Error(String),
    Loaded(String),
}

pub async fn fetch_module_items(source_url: &str, source_type: &str) -> Result<Vec<DocItem>> {
    match source_type {
        "neovim" => fetch_nvim_items().await,
        "rust-std" => fetch_rust_items(source_url).await,
        _ => fetch_generic_items(source_url).await,
    }
}

async fn fetch_rust_items(source_url: &str) -> Result<Vec<DocItem>> {
    let url = format!("{}/index.html", source_url);
    let html = reqwest::get(&url).await?.text().await?;
    let document = Html::parse_document(&html);
    let mut items = Vec::new();

    let link_sel = Selector::parse("a.mod, a.struct, a.fn, a.enum, a.trait, a.type, a.macro, a.union, a.constant, a.primitive, a.keyword").unwrap();

    for link in document.select(&link_sel) {
        if let Some(href) = link.value().attr("href") {
            if href.starts_with("http") || href.contains('#') {
                continue;
            }
            let name = link.text().collect::<String>().trim().to_string();
            if name.is_empty() {
                continue;
            }
            let class_str = link.value().classes().collect::<Vec<_>>().join(" ");
            let item_type = if class_str.contains("struct") { "struct" }
            else if class_str.contains("enum") { "enum" }
            else if class_str.contains("trait") { "trait" }
            else if class_str.contains("fn") { "fn" }
            else if class_str.contains("mod") { "mod" }
            else if class_str.contains("type") { "type" }
            else if class_str.contains("macro") { "macro" }
            else if class_str.contains("union") { "union" }
            else if class_str.contains("constant") { "constant" }
            else if class_str.contains("primitive") { "primitive" }
            else if class_str.contains("keyword") { "keyword" }
            else { "other" }
            .to_string();

            let full_url = if href.starts_with('/') {
                format!("https://doc.rust-lang.org{}", href)
            } else {
                format!("{}/{}", source_url.trim_end_matches('/'), href)
            };

            items.push(DocItem { name, item_type, url: full_url });
        }
    }

    Ok(items)
}

async fn fetch_nvim_items() -> Result<Vec<DocItem>> {
    let html = reqwest::get("https://neovim.io/doc/user").await?.text().await?;
    let document = Html::parse_document(&html);
    let mut items = Vec::new();

    let help_sel = Selector::parse(".help-li a").unwrap();

    for link in document.select(&help_sel) {
        if let Some(href) = link.value().attr("href") {
            let name = link.text().collect::<String>().trim().to_string();
            if name.is_empty() {
                continue;
            }
            let full_url = format!("https://neovim.io{}", href);
            items.push(DocItem {
                name,
                item_type: "help".into(),
                url: full_url,
            });
        }
    }

    Ok(items)
}

async fn fetch_generic_items(source_url: &str) -> Result<Vec<DocItem>> {
    let url = format!("{}/index.html", source_url);
    let html = reqwest::get(&url).await?.text().await?;
    let document = Html::parse_document(&html);
    let mut items = Vec::new();

    let link_sel = Selector::parse("a[href]").unwrap();

    for link in document.select(&link_sel) {
        if let Some(href) = link.value().attr("href") {
            if href.starts_with("http") || href.starts_with('#') || href.starts_with("javascript:") {
                continue;
            }
            let name = link.text().collect::<String>().trim().to_string();
            if name.is_empty() || name.len() < 2 {
                continue;
            }
            let full_url = resolve_url(source_url, href);
            items.push(DocItem {
                name,
                item_type: "page".into(),
                url: full_url,
            });
        }
    }

    Ok(items)
}

fn resolve_url(base: &str, href: &str) -> String {
    let base = base.trim_end_matches('/');
    if href.starts_with('/') {
        let domain_end = base[8..].find('/').map(|i| i + 8).unwrap_or(base.len());
        format!("{}{}", &base[..domain_end], href)
    } else {
        format!("{}/{}", base, href)
    }
}

pub async fn fetch_item_content(url: &str) -> Result<String> {
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);
    let mut text = String::new();

    let content_sel = Selector::parse(".docblock, #main-content, main, #content, .col-wide").unwrap();
    if let Some(content) = document.select(&content_sel).next() {
        extract_text(&content, &mut text, 0);
    }

    if text.trim().is_empty() {
        let body_sel = Selector::parse("body").unwrap();
        if let Some(body) = document.select(&body_sel).next() {
            extract_text(&body, &mut text, 0);
        }
    }

    Ok(text.trim().to_string())
}

fn extract_text(element: &scraper::ElementRef, out: &mut String, depth: usize) {
    for child in element.children() {
        match child.value() {
            scraper::node::Node::Text(t) => {
                let s = t.text.trim();
                if !s.is_empty() {
                    if out.ends_with('\n') || out.is_empty() {
                        out.push_str(s);
                    } else {
                        out.push(' ');
                        out.push_str(s);
                    }
                }
            }
            scraper::node::Node::Element(e) => {
                let tag: &str = e.name.local.as_ref();
                let is_block = matches!(tag, "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "div" | "section" | "pre" | "blockquote" | "ul" | "ol" | "li" | "table" | "tr" | "td" | "th" | "dl" | "dt" | "dd" | "br");

                if is_block && !out.ends_with('\n') && !out.is_empty() {
                    out.push('\n');
                }

                if let Some(el) = scraper::ElementRef::wrap(child) {
                    match tag {
                        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                            let level = tag.as_bytes().last().unwrap_or(&b'1') - b'0';
                            let prefix = "#".repeat(level as usize);
                            out.push_str(&format!("\n{} ", prefix));
                            extract_text(&el, out, depth + 1);
                            out.push('\n');
                        }
                        "li" => {
                            out.push_str("  • ");
                            extract_text(&el, out, depth + 1);
                        }
                        "pre" | "code" => {
                            let code: String = el.text().collect();
                            let code = code.trim();
                            if !code.is_empty() {
                                out.push_str(&format!("\n```\n{}\n```\n", code));
                            }
                        }
                        "a" => {
                            extract_text(&el, out, depth + 1);
                        }
                        "br" => {
                            out.push('\n');
                        }
                        _ => {
                            extract_text(&el, out, depth + 1);
                        }
                    }
                }

                if is_block && !out.ends_with('\n') {
                    out.push('\n');
                }
            }
            _ => {}
        }
    }
}
