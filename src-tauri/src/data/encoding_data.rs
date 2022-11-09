use crate::encoding::types::Encoding;
use crate::encoding::types::Operation;

pub fn get_encodings() -> Vec<Encoding> {
    vec![
        Encoding::new(
            "base64",
            "Base64",
            "Converts text to and from Base64 encoding.",
            vec![
                Operation::new("encode", "decode", "Encode", "Encodes to Base64."),
                Operation::new("decode", "encode", "Decode", "Decodes from Base64."),
            ],
        ),
        Encoding::new(
            "html",
            "HTML",
            "Converts to and from escaped HTML.",
            vec![
                Operation::new("escape", "unescape", "Escape", "Escape the html."),
                Operation::new("unescape", "escape", "Unescape", "Decode the input."),
            ],
        ),
        Encoding::new(
            "url",
            "URL",
            "A Uniform Resource Locator (URL), colloquially termed a web address, is a reference to a web resource that specifies its location on a computer network and a mechanism for retrieving it. A URL is a specific type of Uniform Resource Identifier (URI), although many people use the two terms interchangeably.",
            vec![
                Operation::new("escape", "unescape", "Escape", "Escape the input."),
                Operation::new("unescape", "escape", "Unescape", "Decode the input."),
            ],
        ),
        Encoding::new(
            "smart-url",
            "URL - Smart",
            "A Uniform Resource Locator (URL), colloquially termed a web address, is a reference to a web resource that specifies its location on a computer network and a mechanism for retrieving it. A URL is a specific type of Uniform Resource Identifier (URI), although many people use the two terms interchangeably.",
            vec![
                Operation::new("escape", "unescape", "Escape", "Encode the input"),
                Operation::new("unescape", "escape", "Unescape", "Decode the input"),
            ],
        ),
    ]
}