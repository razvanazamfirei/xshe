pub trait TestCases {
    const CONVERT_STRING: &'static str;
    const CONVERT_ARRAY: &'static str;
    const CONVERT_PATH: &'static str;
    const CONVERT_SET: &'static str;
    const CONVERT_UNSET: &'static str;
    const CONVERT_SPECIFIC: &'static str;
    const CONVERT_SPECIFIC_OTHER: &'static str;
    const CONVERT_SPECIFIC_OTHER_ALT: &'static str;
    const SHELL_VARIABLES_INLINE: &'static str;
    const SHELL_VARIABLES_BLOCK: &'static str;
    const SHELL_COMMANDS: &'static str;
    const SHELL_HOME_TILDE: &'static str;
    const CONVERT_CHARACTER_ESCAPES: &'static str;
    const CONVERT_ESCAPING_QUOTES: &'static str;
    const CONVERT_ALL: &'static str;
}

#[cfg(test)]
mod test_conversion {
    use crate::{
        shells::{BashShell, FishShell, ZshShell},
        convert::{tests::TestCases, to_shell_source},
        structure::*,
        structure::{ConfigFile, EnvironmentVariables},
        Shells::{self, *}
    };
    use indexmap::indexmap;
    use indoc::indoc;
    use maplit::hashmap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use std::collections::HashMap;
    use EnvVariableOption::*;

    // Used to test whether a file in toml format can be converted ito a correct `IndexMap`,
    // and then whether it can be converted from an `IndexMap`
    // into an output string to be sourced, for each shell.
    // This checks both the functionality of `convert::to_shell_source` (in this file)
    // and whether `config_file::ConfigFile` parses correctly.
    fn assert_convert(
        toml_str: &str,
        map: EnvironmentVariables,
        shell_sources: HashMap<Shells, &str>,
    ) {
        // Verify that the toml converts to the correct representation.
        assert_eq!(
            toml::from_str::<ConfigFile>(toml_str)
                .expect("Invalid toml formatting")
                .vars,
            map,
            "Compare toml data to its `EnvironmentVariables` representation",
        );

        // Verify that the representation translates into the correct shell-script, for each shell.
        for (shell, shell_source) in shell_sources {
            assert_str_eq!(
                // Trim the trailing newline(s), if they exist.
                to_shell_source(&map, &shell).trim_end_matches('\n'),
                shell_source.trim_end_matches('\n'),
                "Check for correct {:?} syntax",
                &shell,
            );
        }
    }

    #[test]
    fn test_convert_string() {
        assert_convert(
            // language=TOML
            r#"FOO = "Bar""#,
            indexmap! {
                "FOO".into() => General(EnvVariableValue::String("Bar".into())),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_STRING,
                Zsh => ZshShell::CONVERT_STRING,
                Fish => FishShell::CONVERT_STRING,
            },
        )
    }

    #[test]
    fn test_convert_path() {
        assert_convert(
            // language=TOML
            r#"PATH = ["/usr/local/bin", "/usr/bin", "/bin", "/usr/sbin", "/sbin"]"#,
            indexmap! {
                "PATH".into() => General(EnvVariableValue::Path(vec![
                    "/usr/local/bin".into(),
                    "/usr/bin".into(),
                    "/bin".into(),
                    "/usr/sbin".into(),
                    "/sbin".into(),
                ])),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_PATH,
                Zsh => ZshShell::CONVERT_PATH,
                Fish => FishShell::CONVERT_PATH,
            },
        )
    }

    #[test]
    fn test_convert_array() {
        assert_convert(
            // language=TOML
            r#"ARRAY = [["array_item_1", "array_item_2", "array_item_3"]]"#,
            indexmap! {
                "ARRAY".into() => General(EnvVariableValue::Array(vec![vec![
                    "array_item_1".into(),
                    "array_item_2".into(),
                    "array_item_3".into(),
                ]])),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_ARRAY,
                Zsh => ZshShell::CONVERT_ARRAY,
                Fish => FishShell::CONVERT_ARRAY,
            },
        )
    }

    #[test]
    fn test_convert_set() {
        assert_convert(
            // language=TOML
            "HOMEBREW_NO_ANALYTICS = true",
            indexmap! {
                "HOMEBREW_NO_ANALYTICS".into() => General(EnvVariableValue::Set(true)),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_SET,
                Zsh => ZshShell::CONVERT_SET,
                Fish => FishShell::CONVERT_SET,
            },
        )
    }

    #[test]
    fn test_convert_unset() {
        assert_convert(
            // language=TOML
            "HOMEBREW_NO_ANALYTICS = false",
            indexmap! {
                "HOMEBREW_NO_ANALYTICS".into() => General(EnvVariableValue::Set(false)),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_UNSET,
                Zsh => ZshShell::CONVERT_UNSET,
                Fish => FishShell::CONVERT_UNSET,
            },
        )
    }

    #[test]
    fn test_convert_specific() {
        assert_convert(
            // language=TOML
            r#"ONLY_FOR_BASH.bash = "Do people read test cases?""#,
            indexmap! {
                "ONLY_FOR_BASH".into() => Specific(indexmap! {
                    "bash".into() => EnvVariableValue::String("Do people read test cases?".into()),
                }),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_SPECIFIC,
                Zsh => ZshShell::CONVERT_SPECIFIC,
                Fish => FishShell::CONVERT_SPECIFIC,
            },
        )
    }

    #[test]
    fn test_convert_specific_other() {
        assert_convert(
            // language=TOML
            indoc! {r#"
                SOME_VARIABLE.fish = "you're pretty"
                SOME_VARIABLE._ = '[ACCESS DENIED]'
            "#},
            indexmap! {
                "SOME_VARIABLE".into() => Specific(indexmap! {
                    "fish".into() => EnvVariableValue::String("you're pretty".into()),
                    "_".into() => EnvVariableValue::String("[ACCESS DENIED]".into()),
                })
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_SPECIFIC_OTHER,
                Zsh => ZshShell::CONVERT_SPECIFIC_OTHER,
                Fish => FishShell::CONVERT_SPECIFIC_OTHER,
            },
        )
    }

    #[test]
    fn test_convert_specific_other_alt() {
        assert_convert(
            // language=TOML
            indoc! {r#"
                [SOME_VARIABLE]
                fish = "you're pretty"
                _ = '[ACCESS DENIED]'

                [ANOTHER_VARIABLE]
                zsh = 'Zzz'
            "#},
            indexmap! {
                "SOME_VARIABLE".into() => Specific(indexmap! {
                    "fish".into() => EnvVariableValue::String("you're pretty".into()),
                    "_".into() => EnvVariableValue::String("[ACCESS DENIED]".into()),
                }),
                "ANOTHER_VARIABLE".into() => Specific(indexmap! {
                    "zsh".into() => EnvVariableValue::String("Zzz".into()),
                }),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_SPECIFIC_OTHER_ALT,
                Zsh =>  ZshShell::CONVERT_SPECIFIC_OTHER_ALT,
                Fish => FishShell::CONVERT_SPECIFIC_OTHER_ALT,
            },
        )
    }

    #[test]
    fn test_shell_variables_inline() {
        assert_convert(
            // language=TOML
            r#"WHERE_THE_HEART_IS = "$HOME""#,
            indexmap! {
                "WHERE_THE_HEART_IS".into() => General(EnvVariableValue::String("$HOME".into())),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::SHELL_VARIABLES_INLINE,
                Zsh => ZshShell::SHELL_VARIABLES_INLINE,
                Fish => FishShell::SHELL_VARIABLES_INLINE,
            },
        )
    }

    #[test]
    fn test_shell_variables_block() {
        assert_convert(
            // language=TOML
            r#"AN_EXAMPLE = "${HOME}less""#,
            indexmap! {
                "AN_EXAMPLE".into() => General(EnvVariableValue::String("${HOME}less".into())),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::SHELL_VARIABLES_BLOCK,
                Zsh => ZshShell::SHELL_VARIABLES_BLOCK,
                Fish => FishShell::SHELL_VARIABLES_BLOCK,
            },
        )
    }

    #[test]
    fn test_shell_commands() {
        assert_convert(
            // language=TOML
            r#"EDITOR = "$(which micro)""#,
            indexmap! {
                "EDITOR".into() => General(EnvVariableValue::String("$(which micro)".into())),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::SHELL_COMMANDS,
                Zsh => ZshShell::SHELL_COMMANDS,
                Fish => FishShell::SHELL_COMMANDS,
            },
        )
    }

    #[test]
    fn test_shell_home_tilde() {
        assert_convert(
            // language=TOML
            r#"HOME = "~superatomic""#,
            indexmap! {
                "HOME".into() => General(EnvVariableValue::String("~superatomic".into())),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::SHELL_HOME_TILDE,
                Zsh => ZshShell::SHELL_HOME_TILDE,
                Fish => FishShell::SHELL_HOME_TILDE,
            },
        )
    }

    #[test]
    fn test_convert_character_escapes() {
        assert_convert(
            // language=TOML
            indoc! {r#"
                MESSAGE = '''\$() is literal, and $(echo '\)') is escaped.'''
                FAVORITE_CHARACTER = '\\'
            "#},
            indexmap! {
                "MESSAGE".into() => General(EnvVariableValue::String(
                    r"\$() is literal, and $(echo '\)') is escaped.".into()
                )),
                "FAVORITE_CHARACTER".into() => General(EnvVariableValue::String(r"\\".into())),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_CHARACTER_ESCAPES,
                Zsh => ZshShell::CONVERT_CHARACTER_ESCAPES,
                Fish => FishShell::CONVERT_CHARACTER_ESCAPES,
            },
        )
    }

    #[test]
    fn test_convert_escaping_quotes() {
        assert_convert(
            // language=TOML
            "MESSAGE = '''I 'love' books'''",
            indexmap! {
                "MESSAGE".into() => General(EnvVariableValue::String(r"I 'love' books".into())),
            },
            // language=sh
            hashmap! {
                Bash =>BashShell::CONVERT_ESCAPING_QUOTES,
                Zsh => ZshShell::CONVERT_ESCAPING_QUOTES,
                Fish => FishShell::CONVERT_ESCAPING_QUOTES,
            },
        )
    }

    #[test]
    fn test_convert_everything() {
        assert_convert(
            // language=TOML
            indoc! {r#"
                # A collection of random things for testing.
                FOO = 'bar'
                BAZ.zsh = 'zž'
                BAZ.fish = ['gone', '$fishing']
                BAZ._ = '~other'
                ARRAY_TEST = [['$home', 'alone']]
                NOTHING_CHANGES = ['$home', 'alone']
                TTY = '$(tty)'
                THE_ECHO = '$(echo "\)")'
                XSHE_IS_THE_BEST = true # look, idk.
                # Return to poluting the home directory in bash
                XDG_CONFIG_HOME.bash = false
            "#},
            indexmap! {
                "FOO".into() => General(EnvVariableValue::String("bar".into())),
                "BAZ".into() => Specific(indexmap! {
                    "zsh".into() => EnvVariableValue::String("zž".into()),
                    "fish".into() => EnvVariableValue::Path(vec!["gone".into(), "$fishing".into()]),
                    "_".into() => EnvVariableValue::String("~other".into()),
                }),
                "ARRAY_TEST".into() => General(EnvVariableValue::Array(vec![vec![
                    "$home".into(),
                    "alone".into(),
                ]])),
                "NOTHING_CHANGES".into() => General(EnvVariableValue::Path(vec![
                    "$home".into(),
                    "alone".into(),
                ])),
                "TTY".into() => General(EnvVariableValue::String("$(tty)".into())),
                "THE_ECHO".into() => General(EnvVariableValue::String(r#"$(echo "\)")"#.into())),
                "XSHE_IS_THE_BEST".into() => General(EnvVariableValue::Set(true)),
                "XDG_CONFIG_HOME".into() => Specific(indexmap! {
                    "bash".into() => EnvVariableValue::Set(false),
                }),
            },
            // language=sh
            hashmap! {
                Bash => BashShell::CONVERT_ALL,
                Zsh => ZshShell::CONVERT_ALL,
                Fish => FishShell::CONVERT_ALL,
            },
        )
    }
}
