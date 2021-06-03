use crate::divisible_by_any_interruptible;
use crate::interleave;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;

///The strategy for this one is to pass an Arc::clone to the thread.
pub fn sieve_multithreaded_arc_interruptible(max: i32) -> Vec<i32> {
    let thread_count = 4;
    let primes: Vec<Arc<Mutex<Vec<i32>>>> = (0..thread_count)
        .map(|_i| Arc::new(Mutex::new(vec![])))
        .collect();
    let mut carousel = 0;
    for i in 2..max {
        let mut join_handles = Vec::new();
        let quit_flag = Arc::new(AtomicBool::new(false));
        for chunk in &primes {
            let p2 = Arc::clone(chunk);
            let quit2 = Arc::clone(&quit_flag);
            let handle = thread::spawn(move || {
                let qf = quit2.as_ref();
                divisible_by_any_interruptible(i, &p2.lock().unwrap(), qf)
            });
            join_handles.push(handle);
        }

        let any = join_handles
            .into_iter()
            //.any(|h| h.join().unwrap()) // fails to consume all the JoinHandles leading to a failure during try_unwrap()
            .fold(false, |accum, h| accum | h.join().unwrap());

        if !any {
            primes[carousel].lock().unwrap().push(i);

            carousel += 1;
            if carousel >= primes.len() {
                carousel = 0;
            }
        }
    }
    let tmp: Vec<Vec<i32>> = crate::unwrap_vec_arc_mutex(primes);

    interleave(&tmp)
}
