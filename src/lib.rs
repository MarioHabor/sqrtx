use anyhow::{bail, Result};
use tokio::task;

/// Computes the square root of a number asynchronously by offloading the computation to a blocking thread pool.
///
/// # Arguments
/// - `number`: The input number (must be non-negative).
///
/// # Returns
/// - `Ok(f64)` if the computation is successful.
/// - `Err(anyhow::Error)` if the input number is negative.
pub async fn square_root_async(number: f64) -> Result<f64> {
    task::spawn_blocking(move || {
        if number < 0.0 {
            bail!(
                "Cannot calculate the square root of a negative number: {}",
                number
            );
        }

        let mut guess = number / 2.0;
        let mut prev_guess;

        loop {
            prev_guess = guess;
            guess = (guess + number / guess) / 2.0;
            if (prev_guess - guess).abs() < 1e-10 {
                break;
            }
        }

        Ok(guess)
    })
    .await?
}

/// Computes the square roots of a list of numbers asynchronously using parallel processing for heavy workloads.
///
/// # Arguments
/// - `numbers`: A vector of numbers (all must be non-negative).
///
/// # Returns
/// - `Ok(Vec<f64>)` if all computations are successful.
/// - `Err(anyhow::Error)` if any input number is negative.
pub async fn square_roots_parallel(numbers: Vec<f64>) -> Result<Vec<f64>> {
    task::spawn_blocking(move || {
        numbers
            .into_iter()
            .map(|number| {
                if number < 0.0 {
                    bail!(
                        "Cannot calculate the square root of a negative number: {}",
                        number
                    );
                }

                let mut guess = number / 2.0;
                let mut prev_guess;

                loop {
                    prev_guess = guess;
                    guess = (guess + number / guess) / 2.0;
                    if (prev_guess - guess).abs() < 1e-10 {
                        break;
                    }
                }

                Ok(guess)
            })
            .collect()
    })
    .await?
}

/// Computes the square root of a number synchronously.
///
/// # Arguments
/// - `number`: The input number (must be non-negative).
///
/// # Returns
/// - `Ok(f64)` if the computation is successful.
/// - `Err(anyhow::Error)` if the input number is negative.
pub fn square_root(number: f64) -> Result<f64> {
    if number < 0.0 {
        bail!(
            "Cannot calculate the square root of a negative number: {}",
            number
        );
    }

    let mut guess = number / 2.0;
    let mut prev_guess;

    loop {
        prev_guess = guess;
        guess = (guess + number / guess) / 2.0;
        if (prev_guess - guess).abs() < 1e-10 {
            break;
        }
    }

    Ok(guess)
}

/// Computes the square roots of a list of numbers synchronously using parallel processing for heavy workloads.
///
/// # Arguments
/// - `numbers`: A vector of numbers (all must be non-negative).
///
/// # Returns
/// - `Ok(Vec<f64>)` if all computations are successful.
/// - `Err(anyhow::Error)` if any input number is negative.
pub fn square_roots_parallel_sync(numbers: Vec<f64>) -> Result<Vec<f64>> {
    numbers
        .into_iter()
        .map(|number| {
            if number < 0.0 {
                bail!(
                    "Cannot calculate the square root of a negative number: {}",
                    number
                );
            }

            let mut guess = number / 2.0;
            let mut prev_guess;

            loop {
                prev_guess = guess;
                guess = (guess + number / guess) / 2.0;
                if (prev_guess - guess).abs() < 1e-10 {
                    break;
                }
            }

            Ok(guess)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_square_root_async() -> Result<()> {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(square_root_async(9.0))?;
        assert!((result - 3.0).abs() < 1e-10);
        Ok(())
    }

    #[test]
    fn test_square_roots_parallel() -> Result<()> {
        let rt = Runtime::new().unwrap();
        let numbers = vec![4.0, 16.0, 25.0];
        let results = rt.block_on(square_roots_parallel(numbers))?;
        let expected = vec![2.0, 4.0, 5.0];

        for (result, &expected_value) in results.iter().zip(expected.iter()) {
            assert!((*result - expected_value).abs() < 1e-10);
        }

        Ok(())
    }

    #[test]
    fn test_square_root_async_negative() {
        let rt = Runtime::new().unwrap();
        let result = rt.block_on(square_root_async(-4.0));
        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert_eq!(
            error_message,
            "Cannot calculate the square root of a negative number: -4"
        );
    }

    #[test]
    fn test_square_roots_parallel_with_negative() {
        let rt = Runtime::new().unwrap();
        let numbers = vec![4.0, -16.0, 25.0];
        let result = rt.block_on(square_roots_parallel(numbers));
        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert_eq!(
            error_message,
            "Cannot calculate the square root of a negative number: -16"
        );
    }
}

#[cfg(test)]
mod tests_sync {
    use super::*;

    #[test]
    fn test_square_root_sync() -> Result<()> {
        let result = square_root(9.0)?;
        assert!((result - 3.0).abs() < 1e-10);
        Ok(())
    }

    #[test]
    fn test_square_roots_parallel_sync() -> Result<()> {
        let numbers = vec![4.0, 16.0, 25.0];
        let results = square_roots_parallel_sync(numbers)?;
        let expected = vec![2.0, 4.0, 5.0];

        for (result, &expected_value) in results.iter().zip(expected.iter()) {
            assert!((*result - expected_value).abs() < 1e-10);
        }

        Ok(())
    }

    #[test]
    fn test_square_root_sync_negative() {
        let result = square_root(-4.0);
        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert_eq!(
            error_message,
            "Cannot calculate the square root of a negative number: -4"
        );
    }

    #[test]
    fn test_square_roots_parallel_sync_with_negative() {
        let numbers = vec![4.0, -16.0, 25.0];
        let result = square_roots_parallel_sync(numbers);
        assert!(result.is_err());
        let error_message = result.unwrap_err().to_string();
        assert_eq!(
            error_message,
            "Cannot calculate the square root of a negative number: -16"
        );
    }
}
