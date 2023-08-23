pub fn mean(numbers: &Vec<f64>) -> Option<f64> {
    if contains_nan(numbers) {
        return None;
    }
    let numbers_length = numbers.len();
    match numbers_length {
        0 => None,
        _ => Some(numbers.iter().sum::<f64>() / numbers_length as f64),
    }
}

#[cfg(test)]
mod mean_tests {
    use super::*;

    #[test]
    fn it_takes_in_a_list_of_numbers_and_doesnt_panic() {
        let long_vec: Vec<f64> = vec![1.0];
        mean(&long_vec);
    }

    #[test]
    fn it_prints_the_mean() {
        let short_vec: Vec<f64> = vec![2.0, 2.0, 2.0];
        assert_eq!(mean(&short_vec), Some(2.0));
    }

    #[test]
    fn it_prints_the_mean_with_some_recurring_numbers() {
        let short_vec: Vec<f64> = vec![3.0, 3.0, 4.0];
        assert_eq!(mean(&short_vec), Some(3.3333333333333335));
    }

    #[test]
    fn it_handles_dividing_zero() {
        // It's built in! Thanks Rust!
        let zero_mean = vec![-0.1, 0.0, 0.1];
        assert_eq!(mean(&zero_mean), Some(0.0));
    }

    #[test]
    fn it_handles_an_empty_vector() {
        let empty_vec: Vec<f64> = Vec::new();
        assert_eq!(mean(&empty_vec), None);
    }

    #[test]
    fn it_handles_mixed_positive_and_negative_numbers() {
        let mixed_numbers: Vec<f64> = vec![-3.0, 0.0, 3.0];
        assert_eq!(mean(&mixed_numbers), Some(0.0));
    }

    #[test]
    fn it_handles_large_dataset() {
        let large_vec: Vec<f64> = vec![1.0; 1000000];
        assert_eq!(mean(&large_vec), Some(1.0));
    }
}

fn contains_nan(numbers: &[f64]) -> bool {
    numbers.iter().any(|&x| x.is_nan())
}

pub fn median(numbers: &Vec<f64>) -> Option<f64> {
    if contains_nan(numbers) {
        return None;
    }

    let mut numbers_owned = numbers.to_owned();
    let numbers_length = numbers.len();
    match numbers_length {
        0 => None,
        _ => {
            numbers_owned.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            match numbers_length % 2 {
                0 => {
                    // Even Number
                    let big_index = numbers_length / 2;
                    let small_index = (numbers_length / 2) - 1;
                    Some((numbers_owned[small_index] + numbers_owned[big_index]) / 2.0)
                }
                _ => {
                    // Odd Number:
                    let index = numbers_length / 2;
                    Some(numbers_owned[index])
                }
            }
        }
    }
}

#[cfg(test)]
mod median_tests {

    use super::*;

    #[test]
    fn it_tests_we_have_a_median_function() {
        let result = median(&vec![0.0]);
        assert_eq!(result, Some(0.0));
    }

    #[test]
    fn returns_none_for_empty_vector() {
        let result = median(&vec![]);
        assert_eq!(result, None);
    }

    #[test]
    fn returns_correct_median_small_vector() {
        let result = median(&vec![1.0, 2.0, 3.0]);
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn returns_correct_median_even_num_elements() {
        let result = median(&vec![2.0, 3.0]);
        assert_eq!(result, Some(2.5));

        let result2 = median(&vec![2.0, 3.0, 4.0, 5.0]);
        assert_eq!(result2, Some(3.5));

        let result3 = median(&vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        assert_eq!(result3, Some(4.5));

        let big_vec = vec![1.0; 1000];
        let result4 = median(&big_vec);
        assert_eq!(result4, Some(1.0));
    }

    #[test]
    fn returns_correct_median_unordered_elements() {
        let result = median(&vec![3.0, 1.0, 2.0]);
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn returns_correct_median_negative_numbers() {
        let result = median(&vec![-3.0, -1.0, -2.0]);
        assert_eq!(result, Some(-2.0));
    }

    #[test]
    fn returns_correct_median_mixed_numbers() {
        let result = median(&vec![-2.0, 0.0, 2.0]);
        assert_eq!(result, Some(0.0));
    }

    #[test]
    fn returns_correct_median_repeating_numbers() {
        let result = median(&vec![1.0, 1.0, 1.0]);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn returns_correct_median_large_vector() {
        let result = median(&vec![1.0; 1000]);
        assert_eq!(result, Some(1.0));
    }

    #[test]
    fn returns_correct_median_single_element() {
        let result = median(&vec![42.0]);
        assert_eq!(result, Some(42.0));
    }

    #[test]
    fn it_doesnt_crash_with_nan_input() {
        let result = median(&vec![42.0, f64::NAN]);
        assert_eq!(result, None)
    }
}

// #[cfg(test)]
// mod mode_tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
