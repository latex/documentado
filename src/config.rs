use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SourceConfig {
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub source_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub sources: Vec<SourceConfig>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("reading config from {:?}", path))?;
            Ok(serde_json::from_str(&content)?)
        } else {
            let config = Self::defaults();
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let content = serde_json::to_string_pretty(&config)?;
            std::fs::write(&path, &content)?;
            Ok(config)
        }
    }

    pub fn from_sources(sources: &[crate::docs::DocSource]) -> Self {
        Self {
            sources: sources.iter().map(|s| SourceConfig {
                name: s.name.clone(),
                url: s.url.clone(),
                source_type: s.source_type.clone(),
            }).collect(),
        }
    }

    fn defaults() -> Self {
        Self {
            sources: vec![
                SourceConfig {
                    name: "Rust Standard Library".into(),
                    url: "https://doc.rust-lang.org/stable/std".into(),
                    source_type: "rust-std".into(),
                },
                SourceConfig {
                    name: "Neovim User Manual".into(),
                    url: "https://neovim.io/doc/user".into(),
                    source_type: "neovim".into(),
                },
            ],
        }
    }
}

/// Curated list of popular documentation sources
/// Inspired by Kapeli/Dash (https://kapeli.com/dash) and community docsets
/// Credit: Kapeli, Dash-User-Contributors, hashhar/dash-contrib-docset-feeds
pub fn recommended_sources() -> Vec<SourceConfig> {
    vec![
        SourceConfig { name: "Python 3".into(), url: "https://docs.python.org/3".into(), source_type: "generic".into() },
        SourceConfig { name: "JavaScript".into(), url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference".into(), source_type: "generic".into() },
        SourceConfig { name: "TypeScript".into(), url: "https://www.typescriptlang.org/docs".into(), source_type: "generic".into() },
        SourceConfig { name: "React".into(), url: "https://react.dev/reference/react".into(), source_type: "generic".into() },
        SourceConfig { name: "Node.js".into(), url: "https://nodejs.org/docs/latest/api".into(), source_type: "generic".into() },
        SourceConfig { name: "HTML".into(), url: "https://developer.mozilla.org/en-US/docs/Web/HTML/Reference".into(), source_type: "generic".into() },
        SourceConfig { name: "CSS".into(), url: "https://developer.mozilla.org/en-US/docs/Web/CSS/Reference".into(), source_type: "generic".into() },
        SourceConfig { name: "Go".into(), url: "https://pkg.go.dev/std".into(), source_type: "generic".into() },
        SourceConfig { name: "Docker".into(), url: "https://docs.docker.com/reference".into(), source_type: "generic".into() },
        SourceConfig { name: "Kubernetes".into(), url: "https://kubernetes.io/docs/reference".into(), source_type: "generic".into() },
        SourceConfig { name: "PostgreSQL".into(), url: "https://www.postgresql.org/docs/current".into(), source_type: "generic".into() },
        SourceConfig { name: "SQLite".into(), url: "https://www.sqlite.org/docs".into(), source_type: "generic".into() },
        SourceConfig { name: "MongoDB".into(), url: "https://www.mongodb.com/docs/manual/reference".into(), source_type: "generic".into() },
        SourceConfig { name: "Redis".into(), url: "https://redis.io/docs/latest/commands".into(), source_type: "generic".into() },
        SourceConfig { name: "Nginx".into(), url: "https://nginx.org/en/docs".into(), source_type: "generic".into() },
        SourceConfig { name: "Bash".into(), url: "https://www.gnu.org/software/bash/manual/bash.html".into(), source_type: "generic".into() },
        SourceConfig { name: "Git".into(), url: "https://git-scm.com/docs".into(), source_type: "generic".into() },
        SourceConfig { name: "Vim".into(), url: "https://vimhelp.org".into(), source_type: "generic".into() },
        SourceConfig { name: "Lua".into(), url: "https://www.lua.org/manual/5.4".into(), source_type: "generic".into() },
        SourceConfig { name: "Ruby".into(), url: "https://ruby-doc.org/core".into(), source_type: "generic".into() },
        SourceConfig { name: "Ruby on Rails".into(), url: "https://api.rubyonrails.org".into(), source_type: "generic".into() },
        SourceConfig { name: "PHP".into(), url: "https://www.php.net/manual/en".into(), source_type: "generic".into() },
        SourceConfig { name: "Laravel".into(), url: "https://laravel.com/docs/11.x".into(), source_type: "generic".into() },
        SourceConfig { name: "Java".into(), url: "https://docs.oracle.com/en/java/javase/21/docs/api".into(), source_type: "generic".into() },
        SourceConfig { name: "Scala".into(), url: "https://www.scala-lang.org/api/current".into(), source_type: "generic".into() },
        SourceConfig { name: "Kotlin".into(), url: "https://kotlinlang.org/docs/home.html".into(), source_type: "generic".into() },
        SourceConfig { name: "Swift".into(), url: "https://developer.apple.com/documentation/swift".into(), source_type: "generic".into() },
        SourceConfig { name: "C".into(), url: "https://en.cppreference.com/w/c".into(), source_type: "generic".into() },
        SourceConfig { name: "C++".into(), url: "https://en.cppreference.com/w/cpp".into(), source_type: "generic".into() },
        SourceConfig { name: "Rust".into(), url: "https://doc.rust-lang.org/stable/std".into(), source_type: "rust-std".into() },
        SourceConfig { name: "Django".into(), url: "https://docs.djangoproject.com/en/stable".into(), source_type: "generic".into() },
        SourceConfig { name: "Flask".into(), url: "https://flask.palletsprojects.com/en/stable".into(), source_type: "generic".into() },
        SourceConfig { name: "FastAPI".into(), url: "https://fastapi.tiangolo.com/reference".into(), source_type: "generic".into() },
        SourceConfig { name: "Elixir".into(), url: "https://hexdocs.pm/elixir".into(), source_type: "generic".into() },
        SourceConfig { name: "Haskell".into(), url: "https://www.haskell.org/onlinereport".into(), source_type: "generic".into() },
        SourceConfig { name: "Clojure".into(), url: "https://clojure.org/api/cheatsheet".into(), source_type: "generic".into() },
        SourceConfig { name: "Erlang".into(), url: "https://www.erlang.org/doc".into(), source_type: "generic".into() },
        SourceConfig { name: "R".into(), url: "https://www.rdocumentation.org".into(), source_type: "generic".into() },
        SourceConfig { name: "NumPy".into(), url: "https://numpy.org/doc/stable/reference".into(), source_type: "generic".into() },
        SourceConfig { name: "Pandas".into(), url: "https://pandas.pydata.org/docs/reference".into(), source_type: "generic".into() },
        SourceConfig { name: "Matplotlib".into(), url: "https://matplotlib.org/stable/api".into(), source_type: "generic".into() },
        SourceConfig { name: "LaTeX".into(), url: "https://latexref.xyz".into(), source_type: "generic".into() },
        SourceConfig { name: "Dart".into(), url: "https://dart.dev/reference".into(), source_type: "generic".into() },
        SourceConfig { name: "Terraform".into(), url: "https://developer.hashicorp.com/terraform/docs".into(), source_type: "generic".into() },
        SourceConfig { name: "AWS CLI".into(), url: "https://awscli.amazonaws.com/v2/documentation/api/latest/reference".into(), source_type: "generic".into() },
        SourceConfig { name: "Ansible".into(), url: "https://docs.ansible.com/ansible/latest/collections".into(), source_type: "generic".into() },
        SourceConfig { name: "Neovim".into(), url: "https://neovim.io/doc/user".into(), source_type: "neovim".into() },
    ]
}

pub fn config_path() -> Result<PathBuf> {
    let proj = directories::ProjectDirs::from("com", "documentado", "documentado")
        .context("could not determine config directory")?;
    Ok(proj.config_dir().join("config.json"))
}
