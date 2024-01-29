use std::cmp::Ordering;

pub fn binary_search<T: Ord + Clone>(item: &T, pool: &[T]) -> Option<T> {
    let mut is_ascending = true;

    if pool.len() > 1 {
        is_ascending = pool[0] < pool[pool.len() - 1];
    }

    let mut left = 0;
    let mut right = pool.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match is_ascending {
            true => {
                match item.cmp(&pool[mid]) {
                    Ordering::Less => right = mid,
                    Ordering::Equal => return Some(pool[mid].clone()),
                    Ordering::Greater => left = mid + 1,
                }
            },

            false => {
                match item.cmp(&pool[mid]) {
                    Ordering::Less => left = mid + 1,
                    Ordering::Equal => return Some(pool[mid].clone()),
                    Ordering::Greater => right = mid,
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let el = binary_search(&"a", &[]);

        assert_eq!(el, None);
    }

    #[test]
    fn one() {
        let res = binary_search(&"a", &["a"]);

        assert_eq!(res.unwrap(), "a");
    }
}
