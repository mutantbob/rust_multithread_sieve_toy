//#[macro_use]
//extern crate interleave;

use crate::arc_mutex_vec::sieve_multithreaded_1;
use crate::arc_mutex_vec_interruptible::sieve_multithreaded_arc_interruptible;
use crate::move_box_vec::sieve_multithreaded_2;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod arc_mutex_vec;
mod arc_mutex_vec_interruptible;
mod move_box_vec;

fn divisible_by_any(candidate: i32, divisors: &[i32]) -> bool {
    for num in divisors {
        if 0 == candidate % num {
            return true;
        }
    }
    false
}

#[allow(clippy::verbose_bit_mask)]
fn divisible_by_any_interruptible(
    candidate: i32,
    divisors: &[i32],
    quit_flag: &AtomicBool,
) -> bool {
    for (idx, num) in divisors.iter().enumerate() {
        if 0 == candidate % num {
            quit_flag.store(true, Ordering::Relaxed);
            return true;
        }
        if idx & 0x1f == 0 && quit_flag.load(Ordering::Relaxed) {
            return false;
        }
    }
    false
}

fn test_primes_sieve<F>(calc: F)
where
    F: Fn(i32) -> Vec<i32>,
{
    let (max_to_check, biggest_prime, prime_count) = (1000, 997, 168 as usize);
    let interleaved: Vec<i32> = calc(max_to_check);

    println!("primes : {:?}", interleaved);

    let big_prime = *interleaved.last().unwrap();
    assert_equals(biggest_prime, big_prime, "last prime mismatch");

    assert_equals(prime_count, interleaved.len(), "prime count mismatch");
}

fn main() {
    test_primes_sieve(sieve_single_threaded);
    test_primes_sieve(sieve_multithreaded_1);
    println!("check 2");
    test_primes_sieve(sieve_multithreaded_2);
    println!("check 3");
    test_primes_sieve(sieve_multithreaded_arc_interruptible);

    if true {
        let end = 100_000;
        {
            let start = Instant::now();

            let primes = sieve_multithreaded_1(end);
            let elapsed = start.elapsed();
            println!("{} primes; last={}", primes.len(), primes.last().unwrap());
            println!("MT1 elapsed = {:?}", elapsed);
        }
        {
            let start = Instant::now();

            let primes = sieve_multithreaded_2(end);
            let elapsed = start.elapsed();
            println!("{} primes; last={}", primes.len(), primes.last().unwrap());
            println!("MT2 elapsed = {:?}", elapsed);
        }
        {
            let start = Instant::now();
            let primes = sieve_multithreaded_arc_interruptible(end);
            let elapsed = start.elapsed();
            println!("{} primes; last={}", primes.len(), primes.last().unwrap());
            println!("MTAI elapsed = {:?}", elapsed);
        }
    }
}

fn assert_equals<T>(expected: T, actual: T, msg_prefix: &str)
where
    T: PartialEq,
    T: std::fmt::Display,
{
    if !(actual == expected) {
        println!("{} {} != {}", msg_prefix, expected, actual);
    }
}

pub fn interleave<T>(primes: &[Vec<T>]) -> Vec<T>
where
    T: Clone,
{
    let mut interleaved: Vec<T> = Vec::new();
    let mut i = 0;
    loop {
        let mut any = false;
        for chunk in primes {
            if let Some(val) = chunk.get(i) {
                any = true;
                interleaved.push(val.clone());
            }
        }
        if !any {
            break;
        }
        i += 1;
    }
    interleaved
}

fn sieve_single_threaded(max_to_check: i32) -> Vec<i32> {
    let mut primes = vec![2];
    for i in 3..max_to_check {
        if !divisible_by_any(i, &primes) {
            primes.push(i);
        }
    }

    primes
}

pub fn unwrap_vec_arc_mutex(primes: Vec<Arc<Mutex<Vec<i32>>>>) -> Vec<Vec<i32>> {
    primes
        .into_iter()
        .map(|p| Arc::try_unwrap(p).unwrap().into_inner().unwrap())
        .collect()
}
