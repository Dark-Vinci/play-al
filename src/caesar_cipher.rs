
pub fn caesar_cipher(s: &str, shift: u8) -> String {
    s
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a'} else { b'A' };

                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
        }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty()  {
        assert_eq!(caesar_cipher("", 13), "");
    }


    #[test]
    fn caesar_root_13() {
        assert_eq!(caesar_cipher("rust", 13), "ehfg");
    }

}