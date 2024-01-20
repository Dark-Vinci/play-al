pub fn pascal_triangle(n: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];

    for i in 1..=n {
        let mut ni: Vec<i32> = vec![1];

        let mut res = 1;

        for k in 1..i {
            res = res * (i - k);
            res = res / k;

            ni.push(res);
        }

        result.push(ni);
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn for_one() {
        assert_eq!(pascal_triangle(1), vec![vec![1]])
    }

    #[test]
    fn large_value() {
        assert_eq!(pascal_triangle(5), vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]);

        assert_eq!(
            pascal_triangle(4),
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]
        );
    }
}
