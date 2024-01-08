

pub fn xor_bytes(s: &[u8], key: u8) -> Vec<u8> {
    s.iter().map(|a| a ^ key).collect()
}

pub fn xor(text: &str, key: u8) -> Vec<u8> {
    xor_bytes(text.as_bytes(), key)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let key = 10;

        let text = "tomiwa";
        let ciphered_text = xor(text, key);

        assert_eq!(text.as_bytes(), xor_bytes(&ciphered_text, key));
    }

    #[test]
    fn test_every_alphabet_with_space() {
        let key = 32;

        let text = "This is a string with plenty spaces";
        let ciphered_text = xor(text, key);

        assert_eq!(text.as_bytes(), xor_bytes(&ciphered_text, key));
    }

    #[test]
    fn with_unicode() {
        let key = 64;

        let text = "長個個的";

        let ciphered_text = xor(text, key);

        assert_eq!(text.as_bytes(), xor_bytes(&ciphered_text, key));
    }
}