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

pub(crate) mod parser;
pub(crate) mod tests;

use crate::shells::Shells;
use crate::structure::{EnvVariableOption, EnvVariableValue, EnvironmentVariables};
use clap::ValueEnum;
use indexmap::IndexMap;
use std::string::String;

/// Converts the hash table of `vars` into a script for the given `shell`.
pub(crate) fn to_shell_source(vars: &EnvironmentVariables, shell: &Shells) -> String {
    // Generate the script lines based on the trait object
    let outputs: Vec<String> = vars
        .iter()
        .filter_map(|(name, variable_option)| {
            match variable_option {
                EnvVariableOption::General(v) => Some(v),
                // If it is a shell specific choice, get the correct value for `shell`,
                // and then extract the `EnvVariableValue` if it exists and skip the value
                // if it does not
                EnvVariableOption::Specific(map) => value_for_specific(shell, map),
            }
            .map(|raw_value| process_variable(shell, name, raw_value))
        })
        .collect();
    outputs.join("\n") + "\n"
}

fn process_variable(shell: &Shells, name: &str, raw_value: &EnvVariableValue) -> String {
    // If the value of the environment variable is `false`,
    // then add the "unset" script line to the String and skip the rest of this function.
    let shell_instance = shell.create_behavior();
    let script_line = match raw_value {
        EnvVariableValue::Set(false) => shell_instance.unset_variable(name),
        EnvVariableValue::Set(true) => shell_instance.set_variable(name, "1", false),
        EnvVariableValue::String(string) => {
            let expanded_value = shell_instance.expand_value(string);
            shell_instance.set_variable(name, &expanded_value, false)
        }
        EnvVariableValue::Array(array_of_arrays) => {
            let flattened_array = array_of_arrays
                .iter()
                .flat_map(|array| array.iter())
                .map(|value| shell_instance.expand_value(value))
                .collect::<Vec<String>>()
                .join(":");
            shell_instance.set_variable(name, &flattened_array, false)
        }
        EnvVariableValue::Path(path) => {
            let path_string = path
                .iter()
                .map(|value| shell_instance.expand_value(value))
                .collect::<Vec<String>>()
                .join(":");
            shell_instance.set_variable(name, &path_string, true)
        }
    };
    script_line
}

/// Given a `shell` and a `map` of all specific shell options, get the correct shell `EnvVariableValue`.
/// Used by `to_shell_source` to filter the right `EnvVariableOption::Specific` for the current shell.
fn value_for_specific<'a>(
    shell: &Shells,
    map: &'a IndexMap<String, EnvVariableValue>,
) -> Option<&'a EnvVariableValue> {
    let binding = shell.to_possible_value()?;
    let shell_name = binding.get_name();
    map.get(shell_name).or_else(|| map.get("_"))
}
