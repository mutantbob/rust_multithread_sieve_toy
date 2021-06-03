use crate::divisible_by_any;
use crate::interleave;
use std::sync::{Arc, Mutex};
use std::thread;

///The strategy for this one is to pass an Arc::clone to the thread for it to lock and use.
pub fn seive_multithreaded_1(max: i32) -> Vec<i32> {
    let thread_count = 4;
    let primes: Vec<Arc<Mutex<Vec<i32>>>> = (0..thread_count)
        .map(|_i| Arc::new(Mutex::new(vec![])))
        .collect();
    let mut carousel = 0;
    for i in 2..max {
        let mut join_handles = Vec::new();
        for chunk in &primes {
            let p2 = Arc::clone(chunk);
            let handle = thread::spawn(move || divisible_by_any(i, &p2.lock().unwrap()));
            join_handles.push(handle);
        }

        let any = join_handles
            .into_iter()
            //.any(|h| h.join().unwrap()) // this doesn't consume all the JoinHandles and causes try_unwrap() to fail later
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
