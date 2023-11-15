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

use crate::build::CommandLineInterface;
use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

// Adapted from https://rust-cli.github.io/book/in-depth/docs.html

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").ok_or(io::ErrorKind::NotFound)?);

    generate_man(&out_dir)?;
    generate_completion(&out_dir)?;

    Ok(())
}

fn generate_completion(out_dir: &Path) -> io::Result<()> {
    // We generate Elvish shell completion for anyone who wants to manually install it,
    // but Homebrew is unable to install Elvish shell completion.
    let shells = &[Shell::Bash, Shell::Elvish, Shell::Fish, Shell::Zsh];

    for shell in shells {
        let mut cmd = CommandLineInterface::command();
        generate_to(*shell, &mut cmd, "xshe", out_dir)?;
    }
    Ok(())
}

fn generate_man(out_dir: &Path) -> io::Result<()> {
    let cmd = CommandLineInterface::command();

    // Generate Man File
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    fs::write(out_dir.join("xshe.1"), buffer)?;
    Ok(())
}

mod build {
    use clap::{ArgGroup, Parser, ValueEnum, ValueHint};
    use clap_verbosity_flag::{Verbosity, WarnLevel};
    use std::path::PathBuf;

    #[derive(Parser, Debug)]
    #[command(about, long_about, author, version)]
    #[command(arg_required_else_help = true, group = ArgGroup::new("mode").multiple(false))]
    pub struct CommandLineInterface {
        /// The shell to generate a script for
        ///
        /// Outputs a runnable shell script for the specified shell.
        ///
        /// You can directly source these files in your shell.
        /// See <https://xshe.superatomic.dev/#/cli> for a detailed explanation.
        #[arg(value_enum, index = 1)]
        shell: ShellsList,

        #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
        #[arg(env = "XSHE_CONFIG", group = "mode")]
        /// Specifies a custom location to read from
        ///
        /// This defaults to $XDG_CONFIG_HOME, or ~/.config if not set.
        ///
        /// Use --pipe or --file=- to pipe from stdin.
        ///
        /// The file must be in TOML format (https://toml.io/en/).")
        file: Option<PathBuf>,

        #[arg(short, long, value_name = "TOML", value_hint = ValueHint::Other)]
        #[arg(visible_alias = "toml", group = "mode")]
        /// Directly specify TOML to parse
        ///
        /// The passed string must be in TOML format (https://toml.io/en/).
        text: Option<String>,

        #[arg(short, long, value_name = "PIPE", verbatim_doc_comment)]
        #[arg(alias = "stdin", group = "mode", action)]
        /// Get TOML-formatted data from the standard input
        ///
        /// This is normally used to pass a configuration in from a pipe, like so:
        ///
        ///     cat xshe.toml | xshe bash
        ///
        /// The passed string must be in TOML format (https://toml.io/en/).
        pipe: bool,

        #[clap(flatten)]
        verbose: Verbosity<WarnLevel>,
    }

    #[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
    enum ShellsList {
        Bash,
        Dash,
        Elvish,
        Fish,
        Nu,
        Xonsh,
        Zsh,
    }
}
