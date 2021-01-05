use crate::divisible_by_any_interruptible;
use crate::interleave;
use std::sync::atomic::AtomicBool;
use std::sync::{mpsc, Arc, Mutex};
use std::{mem, thread};

/*
The strategy for this one is to pass an Arc::clone to the thread.
*/
pub fn seive_multithreaded_arc_interruptible(max: i32) -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let thread_count = 4;
    let primes: Vec<Arc<Mutex<Vec<i32>>>> = (0..thread_count)
        .map(|_i| Arc::new(Mutex::new(vec![])))
        .collect();
    let mut carousel = 0;
    for i in 2..max {
        let quit_flag = Arc::new(AtomicBool::new(false));
        for chunk in &primes {
            let tx2 = mpsc::Sender::clone(&tx);
            let p2 = Arc::clone(chunk);
            let quit2 = Arc::clone(&quit_flag);
            thread::spawn(move || {
                let qf = quit2.as_ref();
                let result = divisible_by_any_interruptible(i, &p2.lock().unwrap(), qf);
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
        tmp.push(mem::replace(chunk.lock().unwrap().as_mut(), Vec::new()))
        // すみません
    }

    interleave(&tmp)
}
