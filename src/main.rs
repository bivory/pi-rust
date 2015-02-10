/// Based on 'Go concurrency is not parallelism'
/// www.soroushjp.com/2015/02/07/go-concurrency-is-not-parallelism-real-world-lessons-with-monte-carlo-simulations/

extern crate test;
use test::Bencher;

use std::rand::distributions::{IndependentSample, Range};
use std::rand;
use std::num::Float;

// The number of samples to take to estimate Pi.
static DEFAULT_NUMBER_OF_SAMPLES: usize = 1_000_000;

// Estimate Pi by taking random samples.
fn pi(samples: usize) -> f64
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
    let iterations = DEFAULT_NUMBER_OF_SAMPLES;
    let pi_estimate = pi(iterations);
    println!("Pi after {} iterations: {}", iterations, pi_estimate);
}

#[test]
fn test_calculate_pi()
{
    let expected_pi = 3.1415;
    let delta = 0.01;
    let estimate = pi(DEFAULT_NUMBER_OF_SAMPLES);
    assert!((estimate - expected_pi).abs() <= delta);
}

#[bench]
fn bench_calculate_pi(b: &mut Bencher)
{
    b.iter(|| {
        pi(DEFAULT_NUMBER_OF_SAMPLES)
    })
}
