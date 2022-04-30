// Copyright 2022 Ethan Kinnear
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Convert a mapping representation of toml-formatted data into an `eval`able shell script.

// Ignore ptr_arg Clippy warnings, as they are false positives.
// This will be fixed in 1.61.
// https://github.com/rust-lang/rust-clippy/pull/8271
#![allow(clippy::ptr_arg)]

use clap::ArgEnum;
use indexmap::IndexMap;
use std::string::String;

use crate::cli::Shell;
use crate::config_file::{EnvVariableOption, EnvVariableValue, EnvironmentVariables};

pub(crate) fn to_shell_source(vars: &EnvironmentVariables, shell: &Shell) -> String {
    //! Converts the hash table of `vars` into a script for the given `shell`.

    let mut output = String::new();
    for (name, variable_option) in vars {
        // Check whether the current item is a single environment var or a table of specific shells.
        let raw_value = match variable_option {
            EnvVariableOption::General(v) => v,

            // If it is a shell specific choice, get the correct value for `shell`, and then...
            EnvVariableOption::Specific(map) => match value_for_specific(shell, map) {
                Some(v) => v,     // ...extract the `EnvVariableValue` if it exists
                None => continue, // ...and skip the value if it does not.
            },
        };

        // Convert an array to a string, but log if it was an array.
        // Any arrays are treated as a path.
        let (value, is_path) = &match raw_value {
            // If the value of the environment variable is `false`,
            // then add the "unset" script line to the String and skip the rest of this function.
            EnvVariableValue::Set(false) => {
                add_script_line::unset_variable(&mut output, shell, name);
                continue;
            }

            EnvVariableValue::String(string) => (expand_value(string.as_str()), false),
            EnvVariableValue::Set(true) => ("1".to_string(), false),
            EnvVariableValue::Array(array) => {
                let v_expanded: Vec<String> =
                    array.iter().map(|value| expand_value(value)).collect();
                (v_expanded.join(":"), true)
            }
        };

        add_script_line::set_variable(&mut output, shell, name, value, is_path);
    }
    output
}

// Module for adding a line to the script that will be sourced by the shell.
// Defines methods for adding the different types of lines.
mod add_script_line {
    use super::Shell;

    pub fn set_variable(
        output: &mut String,
        shell: &Shell,
        name: &str,
        value: &str,
        is_path: &bool,
    ) {
        // Log each processed variable
        if log_enabled!(log::Level::Trace) {
            let variable_log_header = match is_path {
                true => "[Set]",
                false => "'Set'",
            };
            trace!("{}: {} -> {}", variable_log_header, name, value);
        };

        // Select the correct form for the chosen shell.
        *output += &match shell {
            Shell::Bash | Shell::Zsh => {
                format!("export {}=\"{}\";\n", name, value)
            }
            Shell::Fish => {
                // Add `--path` to the variable if the variable is represented as a list.
                let path = match is_path {
                    true => " --path",
                    false => "",
                };
                format!("set -gx{path} {} \"{}\";\n", name, value, path = path)
            }
        };
    }

    pub fn unset_variable(output: &mut String, shell: &Shell, name: &str) {
        // Log each processed variable
        trace!("Unset: {}", name);

        // Select the correct form for the chosen shell.
        *output += &match shell {
            Shell::Bash | Shell::Zsh => {
                format!("unset {};\n", name)
            }
            Shell::Fish => {
                format!("set -e {};\n", name)
            }
        };
    }
}

fn value_for_specific<'a>(
    shell: &Shell,
    map: &'a IndexMap<String, EnvVariableValue>,
) -> Option<&'a EnvVariableValue> {
    //! Given a `shell` and a `map` of all specific shell options, get the correct shell `EnvVariableValue`.
    //! Used by `to_shell_source` to filter the right `EnvVariableOption::Specific` for the current shell.
    let shell_name = shell.to_possible_value()?.get_name();
    map.get(shell_name).or_else(|| map.get("_"))
}

fn expand_value(value: &str) -> String {
    //! Expand the literal representation of a string in the toml
    //! to a value with escape characters escaped and shell variables expanded.

    // Replace TOML escape codes with the literal representation so that they are correctly used.
    // We need to do this for every code listed here: https://toml.io/en/v1.0.0#string
    let value = value
        .replace('\\', r"\\") // Backslash - Must be first!
        .replace('\x08', r"\b") // Backspace
        .replace('\t', r"\t") // Tab
        .replace('\n', r"\n") // Newline
        .replace('\x0C', r"\f") // Form Feed
        .replace('\r', r"\r") // Carriage Return
        .replace('\"', "\\\""); // Double Quote

    // Expand tildes
    shellexpand::tilde(&value).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config_file::*;

    use EnvVariableOption::*;

    use indexmap::indexmap;
    use indoc::indoc;
    use maplit::hashmap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use std::collections::HashMap;

    // Used to test whether a file in toml format can be converted ito a correct `IndexMap`,
    // and then whether it can be converted from an `IndexMap`
    // into an output string to be sourced, for each shell.
    // This checks both the functionality of `convert::to_shell_source` (in this file)
    // and whether `config_file::ConfigFile` parses correctly.
    fn assert_converts(
        toml_str: &str,
        map: EnvironmentVariables,
        shell_sources: HashMap<Shell, &str>,
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
    fn test_config_file_load_string() {
        assert_converts(
            indoc! {r#"
                FOO = "Bar"
            "#},
            indexmap! {
                "FOO".into() => General(EnvVariableValue::String("Bar".into())),
            },
            hashmap! {
                Shell::Bash => indoc! (r#"export FOO="Bar";"#),
                Shell::Zsh => indoc! (r#"export FOO="Bar";"#),
                Shell::Fish => indoc! (r#"set -gx FOO "Bar";"#),
            },
        )
    }

    #[test]
    fn test_config_file_load_path() {
        assert_converts(
            indoc! {r#"
                PATH = ["/usr/local/bin", "/usr/bin", "/bin", "/usr/sbin", "/sbin"]
            "#},
            indexmap! {
                "PATH".into() => General(EnvVariableValue::Array(vec![
                    "/usr/local/bin".into(),
                    "/usr/bin".into(),
                    "/bin".into(),
                    "/usr/sbin".into(),
                    "/sbin".into(),
                ])),
            },
            hashmap! {
                Shell::Bash => indoc! (r#"export PATH="/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin";"#),
                Shell::Zsh => indoc! (r#"export PATH="/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin";"#),
                Shell::Fish => indoc! (r#"set -gx --path PATH "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin";"#),
            },
        )
    }

    #[test]
    fn test_config_file_load_specific() {
        assert_converts(
            indoc! {r#"
                ONLY_FOR_BASH.bash = "Do people read test cases?"
            "#},
            indexmap! {
                "ONLY_FOR_BASH".into() => Specific(indexmap! {
                    "bash".into() => EnvVariableValue::String("Do people read test cases?".into()),
                }),
            },
            hashmap! {
                Shell::Bash => indoc! (r#"export ONLY_FOR_BASH="Do people read test cases?";"#),
                Shell::Zsh => "",
                Shell::Fish => "",
            },
        )
    }

    #[test]
    fn test_config_file_load_specific_other() {
        assert_converts(
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
            hashmap! {
                Shell::Bash => indoc! (r#"export SOME_VARIABLE="[ACCESS DENIED]";"#),
                Shell::Zsh => indoc! (r#"export SOME_VARIABLE="[ACCESS DENIED]";"#),
                Shell::Fish => indoc! (r#"set -gx SOME_VARIABLE "you're pretty";"#),
            },
        )
    }

    #[test]
    fn test_config_file_load_specific_other_alt() {
        assert_converts(
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
            hashmap! {
                Shell::Bash => indoc! (r#"export SOME_VARIABLE="[ACCESS DENIED]";"#),
                Shell::Zsh => indoc! (r#"
                    export SOME_VARIABLE="[ACCESS DENIED]";
                    export ANOTHER_VARIABLE="Zzz";
                "#),
                Shell::Fish => indoc! (r#"set -gx SOME_VARIABLE "you're pretty";"#),
            },
        )
    }

    #[test]
    fn test_config_file_everything() {
        assert_converts(
            indoc! {r#"
                FOO = 'bar'
                BAZ.zsh = 'zž'
                BAZ.fish = ['gone', 'fishing']
                BAZ._ = 'other'
            "#},
            indexmap! {
                "FOO".into() => General(EnvVariableValue::String("bar".into())),
                "BAZ".into() => Specific(indexmap! {
                    "zsh".into() => EnvVariableValue::String("zž".into()),
                    "fish".into() => EnvVariableValue::Array(vec!["gone".into(), "fishing".into()]),
                    "_".into() => EnvVariableValue::String("other".into()),
                })
            },
            hashmap! {
                Shell::Bash => indoc! (r#"
                    export FOO="bar";
                    export BAZ="other";
                "#),
                Shell::Zsh => indoc! (r#"
                    export FOO="bar";
                    export BAZ="zž";
                "#),
                Shell::Fish => indoc! (r#"
                    set -gx FOO "bar";
                    set -gx --path BAZ "gone:fishing";
                "#),
            },
        )
    }
}
