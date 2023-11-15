use crate::convert::parser::ValuePartKind;
use crate::convert::tests::TestCases;
use crate::shells::{Behavior, FishShell};
use indoc::indoc;

impl Behavior for FishShell {
    fn shell_format(&self, kind: &ValuePartKind, value: &str) -> String {
        match kind {
            ValuePartKind::Literal => self.single_quote(value),
            ValuePartKind::ShellVariable => format!(r#""${}""#, value),
            ValuePartKind::ShellCommand => format!("(eval {})", self.single_quote(value)),
            ValuePartKind::Home => format!("(eval echo \"~{}\")", value),
        }
    }

    fn single_quote(&self, string: &str) -> String {
        format!("'{}'", string.replace('\\', "\\\\").replace('\'', "\\'"))
    }

    fn set_variable(&self, name: &str, value: &str, is_path: bool) -> String {
        if log_enabled!(log::Level::Trace) {
            let variable_log_header = match is_path {
                true => "[Set]",
                false => "'Set'",
            };
            trace!("{}: {} -> {}", variable_log_header, name, value);
        };
        let path_option = if is_path { " --path" } else { "" };
        format!("set -gx{} {} {}", path_option, name, value)
    }

    fn unset_variable(&self, name: &str) -> String {
        trace!("Unset: {}", name);

        // Select the correct form for the chosen shell.
        format!("set -ge {}", name)
    }
}

impl TestCases for FishShell {
    const CONVERT_STRING: &'static str = r#"set -gx FOO 'Bar'"#;
    const CONVERT_ARRAY: &'static str =
        r#"set -gx ARRAY 'array_item_1':'array_item_2':'array_item_3'"#;
    const CONVERT_PATH: &'static str =
        r#"set -gx --path PATH '/usr/local/bin':'/usr/bin':'/bin':'/usr/sbin':'/sbin'"#;
    const CONVERT_SET: &'static str = r#"set -gx HOMEBREW_NO_ANALYTICS 1"#;
    const CONVERT_UNSET: &'static str = r#"set -ge HOMEBREW_NO_ANALYTICS"#;
    const CONVERT_SPECIFIC: &'static str = "";
    const CONVERT_SPECIFIC_OTHER: &'static str = r"set -gx SOME_VARIABLE 'you\'re pretty'";
    const CONVERT_SPECIFIC_OTHER_ALT: &'static str = r"set -gx SOME_VARIABLE 'you\'re pretty'";
    const SHELL_VARIABLES_INLINE: &'static str = r#"set -gx WHERE_THE_HEART_IS "$HOME""#;
    const SHELL_VARIABLES_BLOCK: &'static str = r#"set -gx AN_EXAMPLE "$HOME"'less'"#;
    const SHELL_COMMANDS: &'static str = r#"set -gx EDITOR (eval 'which micro')"#;
    const SHELL_HOME_TILDE: &'static str = r#"set -gx HOME (eval echo "~superatomic")"#;
    const CONVERT_CHARACTER_ESCAPES: &'static str = indoc!(
        r"set -gx MESSAGE '$() is literal, and '(eval 'echo \')\'')' is escaped.'
        set -gx FAVORITE_CHARACTER '\\'"
    );
    const CONVERT_ESCAPING_QUOTES: &'static str = r"set -gx MESSAGE 'I \'love\' books'";
    const CONVERT_ALL: &'static str = indoc!(
        r#"
                    set -gx FOO 'bar'
                    set -gx --path BAZ 'gone':"$fishing"
                    set -gx ARRAY_TEST "$home":'alone'
                    set -gx --path NOTHING_CHANGES "$home":'alone'
                    set -gx TTY (eval 'tty')
                    set -gx THE_ECHO (eval 'echo ")"')
                    set -gx XSHE_IS_THE_BEST 1
                "#
    );
}
