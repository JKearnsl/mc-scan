//! Display-time text cleanup.
//!
//! Servers embed Minecraft `§`-formatting codes (colours, bold, …) in their
//! MOTD, version, player and plugin strings. The scanner stores those strings
//! verbatim so the CSV export keeps them; the GUI strips them here so the codes
//! never leak into the rendered text.

/// Removes every `§X` formatting code (the `§` and the character after it).
/// A dangling trailing `§` is dropped too.
pub fn strip_section_codes(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\u{00A7}' {
            chars.next();
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::strip_section_codes;

    #[test]
    fn strips_color_and_format_codes() {
        assert_eq!(strip_section_codes("§aHello §lWorld"), "Hello World");
    }

    #[test]
    fn passes_through_text_without_codes() {
        assert_eq!(strip_section_codes("plain text"), "plain text");
    }

    #[test]
    fn drops_a_dangling_section_sign() {
        // A trailing `§` with no following char is consumed, not emitted.
        assert_eq!(strip_section_codes("abc§"), "abc");
    }
}
