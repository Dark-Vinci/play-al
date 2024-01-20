
// step_by
// find out why the condition

pub fn prime_check(val: usize) -> bool {
    if (val > 1) && (val < 4) {
        return true;
    }

    if (val < 2) || (val % 2 == 0) {
        return false;
    }

    let limit = (val as f64).sqrt() as usize;

    for i in (3..=limit).step_by(2) {
        if val % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn minimal() {
        assert!(!prime_check(20));
        assert!(prime_check(11));
        assert!(!prime_check(199913));
    }
}