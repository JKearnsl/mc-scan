// Strips Minecraft `§X` formatting codes (and a dangling trailing `§`). The
// scanner keeps them verbatim for CSV export; the GUI strips them for display.
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
        assert_eq!(strip_section_codes("abc§"), "abc");
    }
}
