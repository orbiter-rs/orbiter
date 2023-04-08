use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use serde::Deserialize;
use serde::Serialize;

use super::paths::*;

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
    pub repo: String,
    pub provider: Option<String>,
    pub from_release: Option<bool>,
    pub ver: Option<String>,
    pub binary_pattern: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Resource {
    Location(String),
    Repo(Repo),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdaptiveResource {
    Location(String),
    Repo(Repo),
    OSSpecific(SupportedOSSpecificResource),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportedOSSpecificResource {
    pub linux: Option<OSSpecificResource>,
    pub macos: Option<OSSpecificResource>,
    pub windows: Option<OSSpecificResource>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OSSpecificResource {
    Standard(Resource),
    ArchSpecific(ArchSpecificResource),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ArchSpecificResource {
    pub x86_64: Option<Resource>,
    pub aarch64: Option<Resource>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Executable {
    Run(String),
    Command {
        run: String,
        alias: Option<String>,
        use_symlink: Option<bool>,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Menu {
    pub menu_name: String,
    pub name: Option<String>,
    pub run: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceTarget {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdaptiveInstall {
    Run(String),
    OSSpecific {
        linux: Option<String>,
        macos: Option<String>,
        windows: Option<String>,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdaptiveInit {
    Run(String),
    OSSpecific {
        linux: Option<String>,
        macos: Option<String>,
        windows: Option<String>,
    },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    // The `string_or_struct` function delegates deserialization to a type's
    // `FromStr` impl if given a string, and to the type's `Deserialize` impl if
    // given a struct. The function is generic over the field type T (here T is
    // `Build`) so it can be reused for any field that implements both `FromStr`
    // and `Deserialize`.
    pub id: Option<String>,
    pub init: Option<AdaptiveInit>,
    pub resource: AdaptiveResource,
    pub extract: Option<String>,
    pub install: Option<AdaptiveInstall>,
    pub update: Option<String>,
    pub src: Option<SourceTarget>,
    pub load: Option<String>,
    pub exec: Option<Executable>,
    pub menu: Option<Menu>,
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    pub fn parse(config: &str) -> Result<Vec<Payload>, Box<dyn std::error::Error>> {
        Ok(serde_yaml::from_str(config)?)
    }

    #[test]
    fn it_should_parse_minimum() {
        let config = r#"
        - resource: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
          exec: "**/firefox"
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = vec![
            Payload {
                id: None,
                init: None,
                resource: AdaptiveResource::Location("https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US".to_string()),
                install: None,
                update: None,
                src: None,
                extract: None,
                load: None,
                exec: Some( Executable::Run("**/firefox".to_string())),
                menu: None
            }
        ];

        assert_eq!(actual, expected)
    }

    #[test]
    fn it_should_parse_menu() {
        let config = r#"
        - id: ff-dev
          resource: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
          exec: "**/firefox"
          install: "./GitAhead*.sh --include-subdir"
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
                init: None,
                resource: AdaptiveResource::Location("https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US".to_string()),
                install: Some(AdaptiveInstall::Run("./GitAhead*.sh --include-subdir".to_string()) ),
                update: None,
                src: None,
                extract: None,
                load: None,
                exec: Some(Executable::Run("**/firefox".to_string())),
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
          resource:
            repo: gitahead/gitahead
          install: ./GitAhead*.sh --include-subdir
          exec:
            run: '**/GitAhead'
            alias: gitahead
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = vec![Payload {
            id: Some("gitahead".to_string()),
            init: None,
            resource: AdaptiveResource::Repo(Repo {
                repo: "gitahead/gitahead".to_string(),
                provider: None,
                ver: None,
                from_release: None,
                binary_pattern: None,
            }),
            install: Some(AdaptiveInstall::Run(
                "./GitAhead*.sh --include-subdir".to_string(),
            )),
            update: None,
            src: None,
            extract: None,
            load: None,
            exec: Some(Executable::Command {
                run: "**/GitAhead".to_string(),
                alias: Some("gitahead".to_string()),
                use_symlink: None,
            }),
            menu: None,
        }];

        assert_eq!(actual, expected)
    }

    #[test]
    fn it_should_parse_repo_release() {
        let config = r#"
        - id: neovim
          resource:
            repo: neovim/neovim
            binary_pattern: "*.tar.gz"
          extract: "tar xvf *.tar.*"
          exec: "**/bin/nvim"
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = "*.tar.gz";

        let actual_resource = &actual.first().unwrap().resource;
        if let AdaptiveResource::Repo(rel) = actual_resource {
            assert_eq!(rel.binary_pattern.as_ref().unwrap(), expected)
        } else {
            panic!("No binary_pattern")
        }
    }

    #[test]
    fn it_should_parse_extract() {
        let config = r#"
        - id: ff-dev
          resource: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
          extract: "tar xzf *.tar.gz"
          exec: "**/firefox"
          install: "./GitAhead*.sh --include-subdir"
          menu:
            name: firefox
            run: "env GDK_BACKEND=wayland $(readlink -f firefox/firefox)"
            icon: firefox
            menu_name: Firefox
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = "tar xzf *.tar.gz";

        assert_eq!(actual.first().unwrap().extract.as_ref().unwrap(), expected)
    }

    #[test]
    fn it_should_parse_arch() {
        let config = r#"
        - id: minikube
          resource:
            macos: 
                aarch64: https://storage.googleapis.com/minikube/releases/latest/minikube-darwin-arm64
            linux: https://storage.googleapis.com/minikube/releases/latest/minikube-linux-amd64
          install: 'chmod +x ./minikube; ./minikube completion zsh > zsh_completion.zsh'
          src: zsh_completion.zsh
          exec: minikube
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let actual_resource = &actual.first().unwrap().resource;
        let expected =
            "https://storage.googleapis.com/minikube/releases/latest/minikube-darwin-arm64";

        if let AdaptiveResource::OSSpecific(os_specific) = actual_resource {
            if let Some(macos_os_specific_resource) = &os_specific.macos {
                if let OSSpecificResource::ArchSpecific(arch_specific_resource) =
                    macos_os_specific_resource
                {
                    if let Some(aarch64_arch_specific_resource) = &arch_specific_resource.aarch64 {
                        if let Resource::Location(location) = aarch64_arch_specific_resource {
                            assert_eq!(location, expected)
                        } else {
                            panic!("No location")
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod from_reader_tests {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn it_should_parse_from_reader() {
        // string value cannot start with '*', need to have them in a string
        let config = r#"
        - id: gitahead
          resource:
            repo: gitahead/gitahead
          install: ./GitAhead*.sh --include-subdir
          exec:
            run: '**/GitAhead'
            alias: gitahead
        "#;

        let streader = config.as_bytes();
        let mut bufreader = BufReader::new(streader);

        let actual: Vec<Payload> = from_reader(&mut bufreader).unwrap();
        let expected = vec![Payload {
            id: Some("gitahead".to_string()),
            init: None,
            resource: AdaptiveResource::Repo(Repo {
                repo: "gitahead/gitahead".to_string(),
                provider: None,
                ver: None,
                from_release: None,
                binary_pattern: None,
            }),
            install: Some(AdaptiveInstall::Run(
                "./GitAhead*.sh --include-subdir".to_string(),
            )),
            update: None,
            src: None,
            extract: None,
            load: None,
            exec: Some(Executable::Command {
                run: "**/GitAhead".to_string(),
                alias: Some("gitahead".to_string()),
                use_symlink: None,
            }),
            menu: None,
        }];

        assert_eq!(actual, expected)
    }
}
