use crate::{divisible_by_any, interleave};
use std::sync::mpsc;
use std::thread;

/* The strategy in this one is to `Send` a `Box<Vec<i32>>` over the channel
to the thread and receive it back with
the results and put it back in the `primes` array ready for the next iteration
*/
pub fn seive_multithreaded_2(max: i32) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();

    let mut primes: Vec<Option<Box<Vec<i32>>>> = (0..2).map(|_i| Some(Box::new(vec![]))).collect();
    {
        let mut carousel = 0;
        for candidate in 2..max {
            // create several threads and give each one a subset of known primes to check the candidate agains
            for (c_idx, chunk) in &mut primes.iter_mut().enumerate() {
                //let (tx9, rx9) = mspc::channel;
                let tx2 = mpsc::Sender::clone(&tx);
                let p2 = chunk.take().unwrap();

                thread::spawn(move || {
                    let result = divisible_by_any(candidate, &p2);
                    tx2.send((result, c_idx, p2)).unwrap();
                });
            }

            // if any of the threads found a divisor, then `candidate` is not a prime
            let mut any = false;
            for _ in 0..primes.len() {
                let (result, c_idx, p2) = rx.recv().unwrap();
                // have to put it back in the correct slot, or things get out of order
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
