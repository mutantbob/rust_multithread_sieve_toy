use crate::{divisible_by_any, interleave};
use std::thread;

/// The strategy in this one is to pass a `Box<Vec<i32>>`
/// to the thread and receive it back with
/// the results and put it back in the `primes` array ready for the next iteration
pub fn sieve_multithreaded_2(max: i32) -> Vec<i32> {
    let mut primes: Vec<Option<Box<Vec<i32>>>> = (0..2).map(|_i| Some(Box::new(vec![]))).collect();
    {
        let mut carousel = 0;
        for candidate in 2..max {
            let mut join_handles = Vec::new();
            // create several threads and give each one a subset of known primes to check the candidate agains
            for chunk in &mut primes {
                //let (tx9, rx9) = mspc::channel;
                let p2 = chunk.take().unwrap();

                let handle = thread::spawn(move || {
                    let result = divisible_by_any(candidate, &p2);
                    (result, p2)
                });
                join_handles.push(handle);
            }

            // if any of the threads found a divisor, then `candidate` is not a prime
            let mut any = false;
            for (c_idx, handle) in join_handles.into_iter().enumerate() {
                let (result, p2) = handle.join().unwrap();
                primes[c_idx] = Some(p2);
                if result {
                    any = true
                }
            }
            if !any {
                let y = &mut primes[carousel];
                if let Some(ref mut x) = y {
                    x.push(candidate);
                } else {
                    panic!("wat")
                }

                carousel += 1;
                if carousel >= primes.len() {
                    carousel = 0;
                }
            }
        }
    }

    /*    for (idx,frag) in primes.iter().enumerate() {
            println!("frag[{}] = {:?}", idx, frag);
        }
    */
    let primes: Vec<Vec<i32>> = primes
        .iter_mut()
        .map(|ref mut box1| *box1.take().unwrap())
        .collect();

    /*    for (idx,frag) in primes.iter().enumerate() {
            println!("frag[{}] = {:?}", idx, frag);
        }
    */
    let interleaved = interleave(&primes);
    if false {
        println!("primes : {:?}", interleaved);
    }
    interleaved
}
