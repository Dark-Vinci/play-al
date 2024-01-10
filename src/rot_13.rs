pub fn rot_13(text: &str) -> String {
    let text = text.to_uppercase();

    text
        .chars()
        .map(|c| {
            return match c {
                'A'..='M' =>  ((c as u8) + 13) as char,
                'N'..='Z' => ((c as u8) - 13) as char,
                _ => c
            }
        }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inittial() {
        assert_eq!(rot_13("hello"), "URYYB")
    }

    #[test]
    fn with_unicode() {
        assert_eq!(rot_13("hello🚎"), "URYYB🚎")
    }

    #[test]
    fn long_text() {
        assert_eq!(rot_13("a very long text that is too long"), "N IREL YBAT GRKG GUNG VF GBB YBAT")
    }
}