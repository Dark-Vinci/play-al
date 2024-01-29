// use num_traits::{Num, FromPrimitive};
// use std::{
//     cmp::PartialOrd, 
//     ops::BitAnd, 
//     collections::{
//         HashMap, 
//         HashSet
//     }, 
//     hash::Hash
// };

// // mean of a sequence
// pub fn mean<T: Num + Copy + FromPrimitive>(val: Vec<T>) -> Option<T> {
//     let l = val.len();

//     if l == 0 {
//         return None;
//     }

//     let sum = sum(val);

//     let a = T::from_usize(l).unwrap();

//     Some(sum / a)
// }

// // finding the median of a sequence
// pub fn median<T: Num + Copy + FromPrimitive + PartialOrd>(mut seq: Vec<T>) -> Option<T> {
//     let l = seq.len();

//     if l == 0 {
//         return None;
//     }

//     seq.sort_by(|a, b| a.partial_cmp(&b).unwrap());

//     if l.bitand(1) == 0 {
//         let mid_mid = l / 2;

//         Some(mean_of_two(seq[mid_mid - 1], seq[mid_mid]))
//     } else {
//         let mid = (l / 2) -1;

//         Some(seq[mid])
//     }
// }

// pub fn histogram<T: Eq + Hash>(val: Vec<T>) -> HashMap<T, usize> {
//     val
//         .into_iter()
//         .fold(HashMap::new(), |mut acc, val| {
//             *acc.entry(val).or_insert(0) += 1;

//             acc
//         })
// }

// pub fn mode<T: Eq + Hash>(val: Vec<T>) -> Option<HashSet<T>> {
//     if val.is_empty() {
//         return None;
//     }

//     let hist = histogram(val);
//     let max = *hist.values().max().unwrap();

//     return Some(
//         hist
//             .into_iter()
//             .filter(|(_, v)| *v == max)
//             .map(|(k, _)| k)
//             .collect()
//     )
// }

// pub fn mean_of_two<T: Num + Copy>(a: T, b: T) -> T {
//     (a + b) / (T::one() + T::one())
// }

// pub fn sum<T: Num + Copy>(values: Vec<T>) -> T {
//     values.iter().fold(T::zero(), |acc, x| acc + *x)
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn median_test() {
//         assert_eq!(median(vec![2, 3]).unwrap(), 2);
//         assert_eq!(median(vec![-9, -8, 0, 1, 2, 2, 3, 4, 6, 9, 53]).unwrap(), 2);
//         assert!(median(Vec::<i32>::new()).is_none());
//     }

//     #[test]
//     fn mean_test() {
//         assert_eq!(mean(vec![2, 3]).unwrap(), 2);
//         assert_eq!(mean(vec![-7.0, 4.0, 53.0, 2.0, 1.0, -9.0, 0.0, 2.0, 3.0, -6.0]).unwrap(), 4.3);
//     }

//     #[test]
//     fn mode_test() {
//         assert_eq!(
//             mode(vec![4, 53, 2, 1, 9, 0, 2, 3, 6]).unwrap(),
//             HashSet::from([2])
//         );

//         assert!(mode(Vec::<i32>::new()).is_none());
//     }
// }
