use crate::convert::parser::ValuePartKind;
use crate::shells::{Behavior, XonshShell};

impl Behavior for XonshShell {
    fn shell_format(&self, kind: &ValuePartKind, value: &str) -> String {
        match kind {
            ValuePartKind::Literal => self.single_quote(value),
            ValuePartKind::ShellVariable => format!(r#"${}"#, value),
            ValuePartKind::ShellCommand => format!("$({})", value),
            ValuePartKind::Home => format!("$(~{})", value),
        }
    }

    fn single_quote(&self, string: &str) -> String {
        string
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
            .to_string()
    }

    fn set_variable(&self, name: &str, value: &str, is_path: bool) -> String {
        if log_enabled!(log::Level::Trace) {
            let variable_log_header = match is_path {
                true => "[Set]",
                false => "'Set'",
            };
            trace!("{}: {} -> {}", variable_log_header, name, value);
        };
        if value == "1" {
            format!("${} = True", name)
        } else {
            format!("${} = f\"{}\"", name, value)
        }
    }

    fn unset_variable(&self, name: &str) -> String {
        trace!("Unset: {}", name);

        // Select the correct form for the chosen shell.
        format!("del ${}", name)
    }
}
