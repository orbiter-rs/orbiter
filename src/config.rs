use std::io::{Read, BufRead, BufReader};
use stringreader::StringReader;

use serde::Deserialize;

pub fn parse(config: &str) -> Vec<InitItem> {
    serde_yaml::from_str(config).unwrap()
}

pub fn from_reader(reader: &mut dyn Read)-> Vec<InitItem> {
    serde_yaml::from_reader(reader).unwrap()
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Release {
    repo: String,
    provider: Option<String>,
    binary_pattern: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Download {
    Location(String),
    Release(Release),
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Executable {
    Run(String),
    Command { run: String, alias: Option<String> },
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Launcher {
    menu_name: String,
    name: Option<String>,
    run: Option<String>,
    icon: Option<String>,
}


#[derive(Debug,PartialEq, Deserialize)]
pub struct InitItem {
    // The `string_or_struct` function delegates deserialization to a type's
    // `FromStr` impl if given a string, and to the type's `Deserialize` impl if
    // given a struct. The function is generic over the field type T (here T is
    // `Build`) so it can be reused for any field that implements both `FromStr`
    // and `Deserialize`.
    pub id: Option<String>,
    pub download: Download,
    pub post_download: Option<String>,
    pub exec: Executable,
    pub launcher: Option<Launcher>,
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

        let actual: Vec<InitItem> = parse(config);
        let expected = vec![
            InitItem { 
                id: None, 
                download: Download::Location("https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US".to_string()), 
                post_download: None,
                exec: Executable::Run("**/firefox".to_string()), 
                launcher: None
            }
        ];

        assert_eq!(actual, expected)
    }

    #[test]
    fn it_should_parse_launcher() {
        let config = r#"
        - id: ff-dev
          download: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
          exec: "**/firefox"
          post_download: "./GitAhead*.sh --include-subdir"
          launcher:
            name: firefox
            run: "env GDK_BACKEND=wayland $(readlink -f firefox/firefox)"
            icon: firefox
            menu_name: Firefox
        "#;

        let actual: Vec<InitItem> = parse(config);
        let expected = vec![
            InitItem {
                id: Some("ff-dev".to_string()),
                download: Download::Location("https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US".to_string()),
                post_download: Some("./GitAhead*.sh --include-subdir".to_string()),
                exec: Executable::Run("**/firefox".to_string()),
                launcher: Some(Launcher {
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

        let actual: Vec<InitItem> = parse(config);
        let expected = vec![
            InitItem {
                id: Some("gitahead".to_string()),
                download: Download::Release(Release {
                    repo: "gitahead/gitahead".to_string(),
                    provider: None,
                    binary_pattern: None,
                }),
                post_download: Some("./GitAhead*.sh --include-subdir".to_string()),
                exec: Executable::Command {
                    run: "**/GitAhead".to_string(),
                    alias: Some("gitahead".to_string())
                },
                launcher: None
            }
        ];

        assert_eq!(actual, expected)
    }
}

#[cfg(test)]
mod from_reader_tests {
    use super::*;

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

        let actual: Vec<InitItem> = from_reader(&mut bufreader);
        let expected = vec![
            InitItem {
                id: Some("gitahead".to_string()),
                download: Download::Release(Release {
                    repo: "gitahead/gitahead".to_string(),
                    provider: None,
                    binary_pattern: None,
                }),
                post_download: Some("./GitAhead*.sh --include-subdir".to_string()),
                exec: Executable::Command {
                    run: "**/GitAhead".to_string(),
                    alias: Some("gitahead".to_string())
                },
                launcher: None
            }
        ];

        assert_eq!(actual, expected)
    }
}
