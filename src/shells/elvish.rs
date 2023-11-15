use crate::convert::parser::ValuePartKind;
use crate::shells::{Behavior, ElvishShell};
use log::{log_enabled, trace};

impl Behavior for ElvishShell {
    fn shell_format(&self, kind: &ValuePartKind, value: &str) -> String {
        match kind {
            ValuePartKind::Literal => self.single_quote(value),
            ValuePartKind::ShellVariable => format!("${}", value),
            ValuePartKind::ShellCommand => format!("(eval {})", self.single_quote(value)),
            ValuePartKind::Home => String::from("$home"),
        }
    }

    fn single_quote(&self, string: &str) -> String {
        // Elvish does not need escaping for single quotes within single-quoted strings,
        // as it uses a different quoting mechanism ('...' for literal strings and "..." for strings with expansion).
        format!("'{}'", string)
    }

    fn set_variable(&self, name: &str, value: &str, _is_path: bool) -> String {
        // In Elvish, environment variables are set with 'set-env' for external commands, not with an 'export' keyword.
        // The '_is_path' argument is ignored in this simple implementation, but you can use it as needed.
        if log_enabled!(log::Level::Trace) {
            trace!("[Set]: {} -> {}", name, value);
        }

        // Elvish uses 'set-env' for environment variables and 'set' for local variables; the following assumes an env var.
        format!("set-env {} {}", name, self.single_quote(value))
    }

    fn unset_variable(&self, name: &str) -> String {
        // In Elvish, environment variables are unset with 'del-env'.
        trace!("Unset: {}", name);
        format!("del-env {}", name)
    }
}
