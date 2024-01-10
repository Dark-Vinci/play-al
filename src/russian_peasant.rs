use std::ops::BitAnd;


pub fn russian_peasant(mut a: u32, mut b: u32) -> u32 {
    let mut res = 0;

    while b > 0 {
        if b.bitand(1).eq(&1u32) {
            res += a;
        }

        a = a << 1;
        b = b >> 1;
    }

    return res;
}

#[cfg(test)]
mod test {
    use super::*;

    // for simple value
    #[test]
    fn first() {
        assert_eq!(russian_peasant(100, 29), 100 * 29);
    }

    // for big value
    #[test]
    fn big_value() {
        assert_eq!(russian_peasant(101738, 29983), 101738 * 29983);
    }
}
