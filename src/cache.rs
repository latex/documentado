use std::path::PathBuf;

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::docs::DocItem;

pub struct Cache {
    conn: Connection,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let path = db_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating cache directory {:?}", parent))?;
        }
        let conn = Connection::open(&path)
            .with_context(|| format!("opening cache database {:?}", path))?;
        let cache = Self { conn };
        cache.create_tables()?;
        Ok(cache)
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS items (
                source_url TEXT NOT NULL,
                name TEXT NOT NULL,
                item_type TEXT NOT NULL,
                url TEXT NOT NULL,
                PRIMARY KEY (source_url, name)
            );
            CREATE TABLE IF NOT EXISTS content (
                url TEXT PRIMARY KEY,
                text TEXT NOT NULL
            );",
        )?;
        Ok(())
    }

    pub fn get_items(&self, source_url: &str) -> Result<Option<Vec<DocItem>>> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, item_type, url FROM items WHERE source_url = ?1 ORDER BY name")?;
        let rows = stmt.query_map([source_url], |row| {
            Ok(DocItem {
                name: row.get(0)?,
                item_type: row.get(1)?,
                url: row.get(2)?,
            })
        })?;
        let items: Vec<DocItem> = rows.collect::<std::result::Result<_, _>>()?;
        if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(items))
        }
    }

    pub fn put_items(&self, source_url: &str, items: &[DocItem]) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;
        {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO items (source_url, name, item_type, url) VALUES (?1, ?2, ?3, ?4)",
            )?;
            for item in items {
                stmt.execute([source_url, &item.name, &item.item_type, &item.url])?;
            }
        }
        tx.commit()?;
        Ok(())
    }

    pub fn get_content(&self, url: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT text FROM content WHERE url = ?1")?;
        let mut rows = stmt.query([url])?;
        match rows.next()? {
            Some(row) => Ok(Some(row.get(0)?)),
            None => Ok(None),
        }
    }

    pub fn put_content(&self, url: &str, text: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO content (url, text) VALUES (?1, ?2)",
            [url, text],
        )?;
        Ok(())
    }
}

fn db_path() -> Result<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("com", "documentado", "documentado")
        .context("could not determine project directories")?;
    let dir = proj_dirs.cache_dir();
    Ok(dir.join("cache.db"))
}
