use crate::domain::encode::types::Encoding;
use crate::domain::encode::types::Operation;

use encode::constants::*;

pub fn get_encodings() -> Vec<Encoding> {
    vec![
        Encoding::new(
            BASE_64,
            "Base64",
            "Converts text to and from Base64 encoding.",
            vec![
                Operation::new(ENCODE, DECODE, "Encode", "Encodes text to Base64"),
                Operation::new(DECODE, ENCODE, "Decode", "Decodes Base64 to text"),
            ],
        ),
        Encoding::new(
            HTML,
            "HTML",
            "Converts to and from escaped HTML.",
            vec![
                Operation::new(ESCAPE, UNESCAPE, "Escape", "Escapes text to HTML"),
                Operation::new(UNESCAPE, ESCAPE, "Unescape", "Convert escaped HTML to text"),
            ],
        ),
        Encoding::new(
            TEXT,
            "Text",
            "Converts to and from escaped text.",
            vec![
                Operation::new(ESCAPE, UNESCAPE, "Escape", "Escapes text special characters."),
                Operation::new(UNESCAPE, ESCAPE, "Unescape", "Converts escaped text to text."),
            ],
        ),
        Encoding::new(
            TEXT_CHARACTER,
            "Text Character",
            "Converts text to and from hex or decimal characters codes.",
            vec![
                Operation::new(TO_HEX, FROM_HEX, "To Hex", "Convert text to hexadecimal"),
                Operation::new(FROM_HEX, TO_HEX, "From Hex", "Convert hexadecimal to text"),
                Operation::new(TO_DECIMAL, FROM_DECIMAL, "To Decimal", "Convert text to decimal"),
                Operation::new(FROM_DECIMAL, TO_DECIMAL, "From Decimal", "Convert decimal to text"),
            ],
        ),
        Encoding::new(
            TEXT_UTF8,
            "Text UTF-8",
            "Converts text to and from UTF-8 hex or decimal bytes.",
            vec![
                Operation::new(TO_HEX, FROM_HEX, "To Hex", "Convert UTF-8 to hexadecimal"),
                Operation::new(FROM_HEX, TO_HEX, "From Hex", "Convert hexadecimal to UTF-8"),
                Operation::new(TO_DECIMAL, FROM_DECIMAL, "To Decimal", "Convert UTF-8 to decimal"),
                Operation::new(FROM_DECIMAL, TO_DECIMAL, "From Decimal", "Convert decimal to UTF-8"),
            ],
        ),
        Encoding::new(
            URL,
            "URL",
            "Converts all characters to and from escaped URL.",
            vec![
                Operation::new(ESCAPE, UNESCAPE, "Escape", "Encode text to URL, leaving query and fragment characters unescaped"),
                Operation::new(UNESCAPE, ESCAPE, "Unescape", "Convert escaped URL to text"),
            ],
        ),
        Encoding::new(
            URL_ADDRESS,
            "URL Address",
            "Converts to and from an URL address (eg web address).\nLeaves the query '?' and fragment '#' characters unescaped.",
            vec![
                Operation::new(ESCAPE, UNESCAPE, "Escape", "Escapes text to URL"),
                Operation::new(UNESCAPE, ESCAPE, "Unescape", "Convert escaped URL to text"),
            ],
        ),
    ]
}
