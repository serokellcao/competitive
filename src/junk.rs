use crate::prelude::*;

pub fn demo() {
    // You can use escapes to write bytes by their hexadecimal values...
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    lh(
        ("junk", "demo"),
        format!("What are you doing\x3F (\\x3F means ?) {}", byte_escape),
    );

    // ...or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    lh(
        ("junk", "demo"),
        format!(
            "Unicode character {} (U+211D) is called {}",
            unicode_codepoint, character_name
        ),
    );

    let long_string = "String literals
                              can span multiple lines.
                              The linebreak and indentation here ->\
                              <- can be escaped too!";
    l(long_string);

    let byte_str = b"Hello world!";
    ll(byte_str);
}
