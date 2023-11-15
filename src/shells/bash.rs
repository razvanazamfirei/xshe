use crate::convert::tests::TestCases;
use crate::shells::{Bash, BashShell, Behavior};
use indoc::indoc;

impl Behavior for BashShell {
    // This uses the default implementation
}
impl TestCases for BashShell {
    const CONVERT_STRING: &'static str = r#"export FOO='Bar'"#;
    const CONVERT_ARRAY: &'static str =
        r#"export ARRAY='array_item_1':'array_item_2':'array_item_3'"#;
    const CONVERT_PATH: &'static str =
        r#"export PATH='/usr/local/bin':'/usr/bin':'/bin':'/usr/sbin':'/sbin'"#;
    const CONVERT_SET: &'static str = r#"export HOMEBREW_NO_ANALYTICS=1"#;
    const CONVERT_UNSET: &'static str = r#"unset HOMEBREW_NO_ANALYTICS"#;
    const CONVERT_SPECIFIC: &'static str = r#"export ONLY_FOR_BASH='Do people read test cases?'"#;
    const CONVERT_SPECIFIC_OTHER: &'static str = r#"export SOME_VARIABLE='[ACCESS DENIED]'"#;
    const CONVERT_SPECIFIC_OTHER_ALT: &'static str = r#"export SOME_VARIABLE='[ACCESS DENIED]'"#;
    const SHELL_VARIABLES_INLINE: &'static str = r#"export WHERE_THE_HEART_IS="$HOME""#;
    const SHELL_VARIABLES_BLOCK: &'static str = r#"export AN_EXAMPLE="$HOME"'less'"#;
    const SHELL_COMMANDS: &'static str = r#"export EDITOR=$(eval 'which micro')"#;
    const SHELL_HOME_TILDE: &'static str = r#"export HOME=$(eval echo "~superatomic")"#;
    const CONVERT_CHARACTER_ESCAPES: &'static str = indoc!(
        r#"
                    export MESSAGE='$() is literal, and '$(eval 'echo '"'"')'"'")' is escaped.'
                    export FAVORITE_CHARACTER='\'
                "#
    );
    const CONVERT_ESCAPING_QUOTES: &'static str = r#"export MESSAGE='I '"'"'love'"'"' books'"#;
    const CONVERT_ALL: &'static str = indoc!(
        r#"
                    export FOO='bar'
                    export BAZ=$(eval echo "~other")
                    export ARRAY_TEST="$home":'alone'
                    export NOTHING_CHANGES="$home":'alone'
                    export TTY=$(eval 'tty')
                    export THE_ECHO=$(eval 'echo ")"')
                    export XSHE_IS_THE_BEST=1
                    unset XDG_CONFIG_HOME
                "#
    );
}
