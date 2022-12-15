pub fn escape(value: &str) -> String {
    let mut result = String::new();
    for character in value.chars() {
        match character {
            '\\' => {
                result.push('\\');
                result.push('\\');
            }
            '\"' => {
                result.push('\\');
                result.push('\"');
            }
            '\'' => {
                result.push('\\');
                result.push('\'');
            }
            '\0' => {
                result.push('\\');
                result.push('0');
            }
            '\n' => {
                result.push('\\');
                result.push('n');
            }
            '\r' => {
                result.push('\\');
                result.push('r');
            }
            '\t' => {
                result.push('\\');
                result.push('t');
            }
            value => result.push(value),
        }
    }
    result
}

pub fn unescape(value: &str) -> String {
    let mut result = String::new();
    let mut escape = false;
    for character in value.chars() {
        if escape {
            match character {
                '\\' => result.push('\\'),
                '\"' => result.push('\"'),
                '\'' => result.push('\''),
                '0' => result.push('\0'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                value => result.push(value),
            }
            escape = false;
        } else {
            if character == '\\' {
                escape = true;
            } else {
                result.push(character);
            }
        }
    }
    result
}

// Tests ──────────────────────────────────────────────────────────────────── //

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_escape() {
        assert_eq!(escape("Hello, World!"), "Hello, World!");
        assert_eq!(escape("Hello, \"World\"!"), "Hello, \\\"World\\\"!");
        assert_eq!(escape("Hello, \'World\'!"), "Hello, \\\'World\\\'!");
        assert_eq!(escape("Hello, \0World\0!"), "Hello, \\0World\\0!");
        assert_eq!(escape("Hello, \rWorld\r!"), "Hello, \\rWorld\\r!");
        assert_eq!(escape("Hello, \tWorld\t!"), "Hello, \\tWorld\\t!");
        assert_eq!(escape("Hello, \\World\\!"), "Hello, \\\\World\\\\!");
    }

    #[test]
    fn test_unescape() {
        assert_eq!(unescape("Hello, World!"), "Hello, World!");
        assert_eq!(unescape("Hello, \\\"World\\\"!"), "Hello, \"World\"!");
        assert_eq!(unescape("Hello, \\\'World\\\'!"), "Hello, \'World\'!");
        assert_eq!(unescape("Hello, \\0World\\0!"), "Hello, \0World\0!");
        assert_eq!(unescape("Hello, \\rWorld\\r!"), "Hello, \rWorld\r!");
        assert_eq!(unescape("Hello, \\tWorld\\t!"), "Hello, \tWorld\t!");
        assert_eq!(unescape("Hello, \\\\World\\\\!"), "Hello, \\World\\!");
    }
}
