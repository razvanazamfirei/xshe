mod bash;
mod dash;
mod elvish;
mod fish;
mod nu;
mod xonsh;
mod zsh;

use crate::convert::parser::{ValuePart, ValuePartKind};
use clap::ValueEnum;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Shells {
    Bash,
    Dash,
    Elvish,
    Fish,
    Nu,
    Xonsh,
    Zsh,
}

impl Shells {
    pub fn create_behavior(&self) -> Box<dyn Behavior> {
        match self {
            Shells::Bash => Box::new(BashShell {}),
            Shells::Dash => Box::new(DashShell {}),
            Shells::Elvish => Box::new(ElvishShell {}),
            Shells::Fish => Box::new(FishShell {}),
            Shells::Nu => Box::new(NuShell {}),
            Shells::Xonsh => Box::new(XonshShell {}),
            Shells::Zsh => Box::new(ZshShell {}),
        }
    }
}

pub struct Shell<T> {
    marker: PhantomData<T>,
}

impl<T> Shell<T> {
    pub fn new() -> Self {
        Shell {
            marker: PhantomData,
        }
    }
}

impl<T> Default for Shell<T> {
    fn default() -> Self {
        Self::new()
    }
}
pub trait Bash {}
pub trait Dash {}
pub trait Elvish {}
pub trait Fish {}
pub trait Nu {}
pub trait Xonsh {}
pub trait Zsh {}

#[derive(Eq, PartialEq)]
pub struct BashShell;
#[derive(Eq, PartialEq)]
pub struct DashShell;
#[derive(Eq, PartialEq)]
pub struct ElvishShell;
#[derive(Eq, PartialEq)]
pub struct FishShell;
#[derive(Eq, PartialEq)]
pub struct NuShell;
#[derive(Eq, PartialEq)]
pub struct XonshShell;
#[derive(Eq, PartialEq)]
pub struct ZshShell;

impl Display for Shells {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Shells::Bash => write!(f, "bash"),
            Shells::Dash => write!(f, "dash"),
            Shells::Elvish => write!(f, "elvish"),
            Shells::Fish => write!(f, "fish"),
            Shells::Nu => write!(f, "nu"),
            Shells::Xonsh => write!(f, "xonsh"),
            Shells::Zsh => write!(f, "zsh"),
        }
    }
}

pub trait Behavior {
    fn shell_format(&self, kind: &ValuePartKind, value: &str) -> String {
        match kind {
            ValuePartKind::Literal => self.single_quote(value),
            ValuePartKind::ShellVariable => format!(r#""${}""#, value),
            ValuePartKind::ShellCommand => format!("$(eval {})", self.single_quote(value)),
            ValuePartKind::Home => format!("$(eval echo \"~{}\")", value),
        }
    }
    fn single_quote(&self, string: &str) -> String {
        string
            .split('\'')
            .map(|v| {
                if v.is_empty() {
                    String::new()
                } else {
                    format!("'{}'", v)
                }
            })
            .collect::<Vec<String>>()
            .join(r#""'""#)
    }
    fn set_variable(&self, name: &str, value: &str, is_path: bool) -> String {
        if log_enabled!(log::Level::Trace) {
            let variable_log_header = match is_path {
                true => "[Set]",
                false => "'Set'",
            };
            trace!("{}: {} -> {}", variable_log_header, name, value);
        };

        format!("export {}={}", name, value)
    }
    fn unset_variable(&self, name: &str) -> String {
        trace!("Unset: {}", name);

        // Select the correct form for the chosen shell.
        format!("unset {}", name)
    }
    fn expand_value(&self, value: &str) -> String {
        let value_parts = crate::convert::parser::parse_value(value);
        let mut expanded_value = String::with_capacity(value.len() * 2);
        for ValuePart { value, kind } in value_parts {
            expanded_value.push_str(&self.shell_format(&kind, &value));
        }
        expanded_value
    }
}
