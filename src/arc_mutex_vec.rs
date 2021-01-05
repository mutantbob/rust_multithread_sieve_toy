use crate::divisible_by_any;
use crate::interleave;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/*
The strategy for this one is to pass an Arc::clone to the thread.
*/
pub fn seive_multithreaded_1(max: i32) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let thread_count = 4;
    let primes: Vec<Arc<Mutex<Vec<i32>>>> = (0..thread_count)
        .map(|_i| Arc::new(Mutex::new(vec![])))
        .collect();
    let mut carousel = 0;
    for i in 2..max {
        for chunk in &primes {
            let tx2 = mpsc::Sender::clone(&tx);
            let p2 = Arc::clone(chunk);
            thread::spawn(move || {
                let result = divisible_by_any(i, &p2.lock().unwrap());
                tx2.send(result).unwrap();
            });
        }

        let mut any = false;
        for _i in 0..primes.len() {
            let result = rx.recv().unwrap();
            if result {
                any = true
            }
        }
        if !any {
            primes[carousel].lock().unwrap().push(i);

            carousel += 1;
            if carousel >= primes.len() {
                carousel = 0;
            }
        }
    }
    let mut tmp: Vec<Vec<i32>> = Vec::new();
    for chunk in primes {
        tmp.push(chunk.lock().unwrap().clone())
    }

    interleave(&tmp)
}
