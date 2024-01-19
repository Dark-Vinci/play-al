

pub fn factors(numb: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    // this condition because we are attcking from right and left
    for i in 1..((numb as f64).sqrt() as u64 + 1) {
        if numb % i == 0 {
            factors.push(i);

            // example when numb = 36 and i = 3, the division of 36/3 would also be a factor
            if i != numb / i {
                factors.push(numb/i);
            }
        }
    }

    factors.sort();
    
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial() {
        assert_eq!(factors(22), vec![1, 2, 11, 22])
    }
}