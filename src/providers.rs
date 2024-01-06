use serde::{Deserialize, Serialize};

pub mod github;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Providers {
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "gitlab")]
    GitLab,
    #[serde(rename = "gitee")]
    Gitee,
}

impl From<&str> for Providers {
    fn from(s: &str) -> Self {
        match s {
            "github" => Providers::GitHub,
            "gitlab" => Providers::GitLab,
            "gitee" => Providers::Gitee,
            _ => Providers::GitHub,
        }
    }
}

impl From<&Option<String>> for Providers {
    fn from(s: &Option<String>) -> Self {
        if let Some(provider_type) = s {
            Providers::from(provider_type.as_str())
        } else {
            Providers::GitHub
        }
    }
}

impl From<&Providers> for Providers {
    fn from(s: &Providers) -> Self {
        s.into()
    }
}

impl From<&Option<Providers>> for Providers {
    fn from(s: &Option<Providers>) -> Self {
        match s {
            Some(ss) => ss.into(),
            None => Providers::GitHub,
        }
    }
}
