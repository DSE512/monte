use std::thread;
use rand::prelude::*;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Approximate Pi with the Monte Carlo method
///
/// Draw random points, and count the number of points
/// that fall within the unit circle. This is an approximation of Pi.
#[pyfunction]
fn monte_carlo_pi(num_iterations: u32) -> f64 {
    let num_cpus = num_cpus::get();
    let mut threads = vec![];

    for _ in 0..num_cpus {
        threads.push(thread::spawn(move || {
            let mut rng = thread_rng();
            let mut inside: u64 = 0;
            // Compute over the quarter circle, we can multiply
            // this by four at the end
            for _ in 0..num_iterations {
                let x = rng.gen::<f64>();    
                let y = rng.gen::<f64>();
                let c = x.powf(2f64) + y.powf(2f64);
                if c <= 1f64 {
                    inside += 1;
                }
            }
            inside
        }));
    }

    let mut total_inside: u64 = 0;
    for thread in threads {
        total_inside += thread.join().unwrap();
    }

    let total_iterations = num_iterations as u64 * num_cpus as u64;
    let pi = total_inside as f64 / total_iterations as f64 * 4.0;
    pi
}

#[pymodule]
fn monte(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(monte_carlo_pi, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::monte_carlo_pi;
    #[test]
    fn it_works() {
        let pi = monte_carlo_pi(1_000_000);
        let rust_pi = std::f64::consts::PI;
        assert_approx_eq!(pi, rust_pi, 2f64);
    }
}
