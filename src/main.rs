/// Based on 'Go concurrency is not parallelism'
/// www.soroushjp.com/2015/02/07/go-concurrency-is-not-parallelism-real-world-lessons-with-monte-carlo-simulations/

use std::rand::distributions::{IndependentSample, Range};
use std::rand;
use std::num::Float;

fn pi(samples: u32) -> f64
{
    let mut inside_circle = 0;
    let mut rng = rand::thread_rng();
    let range = Range::new(-1.0, 1.0);

    for _ in 0..(samples as u32)
    {
        let x: f64 = range.ind_sample(&mut rng);
        let y: f64 = range.ind_sample(&mut rng);

        if x*x + y*y <= 1.0
        {
            inside_circle += 1;
        }
    }
    4.0 * (inside_circle as f64) / (samples as f64)
}

fn main()
{
    let iterations = 10_000_000;
    let pi_estimate = pi(iterations);
    println!("Pi after {} iterations: {}", iterations, pi_estimate);
}

#[test]
fn calculate_pi()
{
    let expected_pi = 3.1415;
    let delta = 0.01;
    let estimate = pi(1_000_000);
    assert!((estimate - expected_pi).abs() <= delta);
}
