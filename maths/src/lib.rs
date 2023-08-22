pub fn mean(numbers: &Vec<f64>) -> f64 {
    let mut mean: f64 = 0.0;
    for number in numbers {
        mean = mean + number
    }
    mean / numbers.len() as f64
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
        assert_eq!(mean(&short_vec), 2.0);
    }

    #[test]
    fn it_prints_the_mean_with_some_recurring_numbers() {
        let short_vec: Vec<f64> = vec![3.0, 3.0, 4.0];
        assert_eq!(mean(&short_vec), 3.3333333333333335);
    }

    #[test]
    fn it_handles_dividing_zero() {
        // It's built in! Thanks Rust!
        let zero_mean = vec![-0.1, 0.0, 0.1];
        assert_eq!(mean(&zero_mean), 0.0);
    }
}

// #[cfg(test)]
// mod median_tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// #[cfg(test)]
// mod mode_tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
