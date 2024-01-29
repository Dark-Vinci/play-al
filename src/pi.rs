use rand::Rng;

pub fn monte_carlo_pi(iter_count: u64, width: u64) -> f64 {
    let mut rn = rand::thread_rng();
    let radius: f64 = (width / 2) as f64;

    let mut circle_count = 0;

    for _a in 0..iter_count {
        let px = rn.gen_range(0.0..1.0) * width as f64;
        let py = rn.gen_range(0.0..1.0) * width as f64;

        let distance_from_circ_center = ((radius - px).powf(2.0) + (radius - py).powf(2.0)).sqrt();

        if distance_from_circ_center <= radius {
            circle_count += 1;
        }
    }

    return 4.0 * (circle_count as f64/iter_count as f64);
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn test_against_pi() {
        let pi = monte_carlo_pi(100000, 100);

        assert!(PI > pi);
        assert!(pi > 3.0);
    }
}