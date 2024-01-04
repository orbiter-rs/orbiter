use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use serde::Deserialize;
use serde::Serialize;

use crate::providers::Providers;

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
    pub provider: Option<Providers>,
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
    Standard(Resource),
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
    ArchSpecific(SupportedArchSpecificResource),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportedArchSpecificResource {
    pub x86_64: Option<Resource>,
    pub aarch64: Option<Resource>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportedOSSpecificCommand {
    pub linux: Option<String>,
    pub macos: Option<String>,
    pub windows: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OSSpecificCommand {
    Generic(String),
    OSSpecific(SupportedOSSpecificCommand),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportedShellSpecificCommand {
    pub sh: Option<OSSpecificCommand>,
    pub zsh: Option<OSSpecificCommand>,
    pub bash: Option<OSSpecificCommand>,
    pub fish: Option<OSSpecificCommand>,
    pub powershell: Option<OSSpecificCommand>,
    pub wincmd: Option<OSSpecificCommand>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShellSpecificCommand {
    Generic(String),
    ShellSpecific(SupportedShellSpecificCommand),
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
pub struct SupportedShellSpecificSourceTarget {
    pub sh: Option<SourceTarget>,
    pub zsh: Option<SourceTarget>,
    pub bash: Option<SourceTarget>,
    pub fish: Option<SourceTarget>,
    pub powershell: Option<SourceTarget>,
    pub wincmd: Option<SourceTarget>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShellSpecificSourceTarget {
    Generic(SourceTarget),
    ShellSpecific(SupportedShellSpecificSourceTarget),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SupportedShellSpecificEvaluatable {
    pub sh: Option<String>,
    pub zsh: Option<String>,
    pub bash: Option<String>,
    pub fish: Option<String>,
    pub powershell: Option<String>,
    pub wincmd: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShellSpecificEvaluatable {
    Generic(String),
    ShellSpecific(SupportedShellSpecificEvaluatable),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Payload {
    // The `string_or_struct` function delegates deserialization to a type's
    // `FromStr` impl if given a string, and to the type's `Deserialize` impl if
    // given a struct. The function is generic over the field type T (here T is
    // `Build`) so it can be reused for any field that implements both `FromStr`
    // and `Deserialize`.
    pub id: String,
    pub init: Option<ShellSpecificCommand>,
    pub resource: AdaptiveResource,
    pub extract: Option<String>, // path to the file to be extracted
    pub install: Option<ShellSpecificCommand>,
    pub update: Option<ShellSpecificCommand>,
    pub src: Option<ShellSpecificSourceTarget>,
    pub load: Option<ShellSpecificEvaluatable>,
    pub exec: Option<Executable>,
    pub menu: Option<Menu>,
}

#[cfg(test)]
mod parse_tests {
    use crate::providers::Providers;

    use super::*;

    pub fn parse(config: &str) -> Result<Vec<Payload>, Box<dyn std::error::Error>> {
        Ok(serde_yaml::from_str(config)?)
    }

    #[test]
    fn it_should_parse_minimum() {
        let config = r#"
        - id: minimum
          resource: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
          exec: "**/firefox"
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = vec![
            Payload {
                id: "minimum".to_string(),
                init: None,
                resource: AdaptiveResource::Standard(Resource::Location("https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US".to_string())),
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
          install: 
            sh: 
                macos: "./GitAhead*.sh --include-subdir"
          menu:
            name: firefox
            run: "env GDK_BACKEND=wayland $(readlink -f firefox/firefox)"
            icon: firefox
            menu_name: Firefox
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = vec![
            Payload {
                id: "ff-dev".to_string(),
                init: None,
                resource: AdaptiveResource::Standard(Resource::Location("https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US".to_string())),
                install: Some(ShellSpecificCommand::ShellSpecific(SupportedShellSpecificCommand{
                        sh:Some( OSSpecificCommand::OSSpecific( SupportedOSSpecificCommand{
                            macos: Some("./GitAhead*.sh --include-subdir".to_string()),
                            linux: None, windows: None}
                        )),
                        zsh: None,
                        bash: None,
                        fish: None,
                        powershell: None,
                        wincmd: None
                    }),
                ),
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
          install: 
            sh: 
              macos: './GitAhead*.sh --include-subdir'
          exec:
            run: '**/GitAhead'
            alias: gitahead
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = vec![Payload {
            id: "gitahead".to_string(),
            init: None,
            resource: AdaptiveResource::Standard(Resource::Repo(Repo {
                repo: "gitahead/gitahead".to_string(),
                provider: None,
                ver: None,
                from_release: None,
                binary_pattern: None,
            })),
            install: Some(ShellSpecificCommand::ShellSpecific(
                SupportedShellSpecificCommand {
                    sh: Some(OSSpecificCommand::OSSpecific(SupportedOSSpecificCommand {
                        macos: Some("./GitAhead*.sh --include-subdir".to_string()),
                        linux: None,
                        windows: None,
                    })),
                    zsh: None,
                    bash: None,
                    fish: None,
                    powershell: None,
                    wincmd: None,
                },
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
    fn it_should_parse_resource_repo() {
        let config = r#"
        - id: neovim
          resource:
            repo: neovim/neovim
            provider: github
            from_release: true
            binary_pattern: "*.tar.gz"
          extract: "*.tar.*"
          exec: "**/bin/nvim"
        "#;

        let actual = parse(config).unwrap();
        let actual_resource = &actual.first().unwrap().resource;
        if let AdaptiveResource::Standard(res) = actual_resource {
            if let Resource::Repo(rel) = res {
                assert_eq!(rel.provider.as_ref().unwrap(), &Providers::GitHub);
                assert_eq!(rel.from_release.unwrap(), true);
                assert_eq!(rel.binary_pattern.as_ref().unwrap(), "*.tar.gz");
                return;
            }
        }

        panic!("Invalid resource repo config")
    }

    // #[test]
    // fn it_should_parse_repo() {
    //     let config = r#"
    //     - id: neovim
    //       resource:
    //         repo: neovim/neovim
    //         provider: github
    //         binary_pattern: "*.tar.gz"
    //       extract: "*.tar.*"
    //       exec: "**/bin/nvim"
    //     "#;

    //     let actual: Vec<Payload> = parse(config).unwrap();
    //     let expected = "*.tar.gz";

    //     let actual_resource = &actual.first().unwrap().resource;
    //     if let AdaptiveResource::Standard(res) = actual_resource {
    //         if let Resource::Repo(rel) = res {
    //             assert_eq!(rel.binary_pattern.as_ref().unwrap(), expected);
    //             return;
    //         }
    //     }

    //     panic!("No binary_pattern")
    // }

    #[test]
    fn it_should_parse_extract() {
        let config = r#"
        - id: ff-dev
          resource: https://download.mozilla.org/?product=firefox-devedition-latest-ssl&os=linux64&lang=en-US
          extract: "*.tar.gz"
          exec: "**/firefox"
          install: 
            sh: "./GitAhead*.sh --include-subdir"
          menu:
            name: firefox
            run: "env GDK_BACKEND=wayland $(readlink -f firefox/firefox)"
            icon: firefox
            menu_name: Firefox
        "#;

        let actual: Vec<Payload> = parse(config).unwrap();
        let expected = "*.tar.gz";

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
          install: 
            sh: 'chmod +x ./minikube; ./minikube completion zsh > zsh_completion.zsh'
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
          install: 
            sh: 
              macos: './GitAhead*.sh --include-subdir'
          exec:
            run: '**/GitAhead'
            alias: gitahead
        "#;

        let streader = config.as_bytes();
        let mut bufreader = BufReader::new(streader);

        let actual: Vec<Payload> = from_reader(&mut bufreader).unwrap();
        let expected = vec![Payload {
            id: "gitahead".to_string(),
            init: None,
            resource: AdaptiveResource::Standard(Resource::Repo(Repo {
                repo: "gitahead/gitahead".to_string(),
                provider: None,
                ver: None,
                from_release: None,
                binary_pattern: None,
            })),
            install: Some(ShellSpecificCommand::ShellSpecific(
                SupportedShellSpecificCommand {
                    sh: Some(OSSpecificCommand::OSSpecific(SupportedOSSpecificCommand {
                        macos: Some("./GitAhead*.sh --include-subdir".to_string()),
                        linux: None,
                        windows: None,
                    })),
                    zsh: None,
                    bash: None,
                    fish: None,
                    powershell: None,
                    wincmd: None,
                },
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
