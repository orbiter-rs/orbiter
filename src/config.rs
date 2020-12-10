use std::io::Read;
use std::io::BufReader;
use std::fs::File;

use serde::Serialize;
use serde::Deserialize;

// mod paths;
use crate::paths::*;


pub fn parse(config: &str) -> Result<Vec<Payload>, Box<dyn std::error::Error>> {
    Ok(serde_yaml::from_str(config)?)
}

pub fn from_reader(reader: &mut dyn Read) -> Result<Vec<Payload>, Box<dyn std::error::Error>> {
    Ok(serde_yaml::from_reader(reader)?)
}


pub fn get_payloads() -> Result<Vec<Payload>, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let file = File::open(config_path)?;
    let mut reader = BufReader::new(file);
    from_reader(&mut reader) 
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Repo {
    repo: String,
    provider: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RepoRelease {
    repo: String,
    provider: Option<String>,
    binary_pattern:Option<String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Download {
    Location(String),
    Repo(Repo),
    RepoRelease(RepoRelease),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Executable {
    Run(String),
    Command { run: String, alias: Option<String> },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Menu {
    menu_name: String,
    name: Option<String>,
    run: Option<String>,
    icon: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    // The `string_or_struct` function delegates deserialization to a type's
    // `FromStr` impl if given a string, and to the type's `Deserialize` impl if
    // given a struct. The function is generic over the field type T (here T is
    // `Build`) so it can be reused for any field that implements both `FromStr`
    // and `Deserialize`.
    pub id: Option<String>,
    pub download: Download,
    pub post_download: Option<String>,
    pub exec: Executable,
    pub menu: Option<Menu>,
}


#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn it_should_parse_minimum() {
        let config = r#"
        - download: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
          exec: "**/firefox"
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = vec![
            Payload { 
                id: None, 
                download: Download::Location("https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US".to_string()), 
                post_download: None,
                exec: Executable::Run("**/firefox".to_string()), 
                menu: None
            }
        ];

        assert_eq!(actual, expected)
    }

    #[test]
    fn it_should_parse_menu() {
        let config = r#"
        - id: ff-dev
          download: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
          exec: "**/firefox"
          post_download: "./GitAhead*.sh --include-subdir"
          menu:
            name: firefox
            run: "env GDK_BACKEND=wayland $(readlink -f firefox/firefox)"
            icon: firefox
            menu_name: Firefox
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = vec![
            Payload {
                id: Some("ff-dev".to_string()),
                download: Download::Location("https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US".to_string()),
                post_download: Some("./GitAhead*.sh --include-subdir".to_string()),
                exec: Executable::Run("**/firefox".to_string()),
                menu: Some(Menu {
                    menu_name: "Firefox".to_string(),
                    name: Some("firefox".to_string()),
                    run: Some("env GDK_BACKEND=wayland $(readlink -f firefox/firefox)".to_string()),
                    icon: Some("firefox".to_string())
                })
            }
        ];

        assert_eq!(actual, expected)
    }

    #[test]
    fn it_should_parse_exec() {
        // string value cannot start with '*', need to have them in a string
        let config = r#"
        - id: gitahead
          download:
            repo: gitahead/gitahead
          post_download: ./GitAhead*.sh --include-subdir
          exec:
            run: '**/GitAhead'
            alias: gitahead
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = vec![
            Payload {
                id: Some("gitahead".to_string()),
                download: Download::Repo(Repo {
                    repo: "gitahead/gitahead".to_string(),
                    provider: None,
                }),
                post_download: Some("./GitAhead*.sh --include-subdir".to_string()),
                exec: Executable::Command {
                    run: "**/GitAhead".to_string(),
                    alias: Some("gitahead".to_string())
                },
                menu: None
            }
        ];

        assert_eq!(actual, expected)
    }
}

#[cfg(test)]
mod from_reader_tests {
    use super::*;
    use stringreader::StringReader;
    use std::io::BufReader;

    #[test]
    fn it_should_parse_from_reader() {
        // string value cannot start with '*', need to have them in a string
        let config = r#"
        - id: gitahead
          download:
            repo: gitahead/gitahead
          post_download: ./GitAhead*.sh --include-subdir
          exec:
            run: '**/GitAhead'
            alias: gitahead
        "#;

        let streader = StringReader::new(config);
        let mut bufreader = BufReader::new(streader);

        let actual: Vec<Payload> = from_reader(&mut bufreader).unwrap();
        let expected = vec![
            Payload {
                id: Some("gitahead".to_string()),
                download: Download::Repo(Repo {
                    repo: "gitahead/gitahead".to_string(),
                    provider: None
                }),
                post_download: Some("./GitAhead*.sh --include-subdir".to_string()),
                exec: Executable::Command {
                    run: "**/GitAhead".to_string(),
                    alias: Some("gitahead".to_string())
                },
                menu: None
            }
        ];

        assert_eq!(actual, expected)
    }
}
