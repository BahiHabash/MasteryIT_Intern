use std::{sync::{Arc, Mutex}, thread, time::{Instant}, vec};

fn main () {
    array_batch_proccessing();
    array_bulk_proccessing();
    // single_val_mutiple_access();
}

fn array_batch_proccessing() {
    let time = Instant::now();
    const ARR_SIZE: usize = 1_00_000;
    let mut arr = [0; ARR_SIZE];
    let batch_size = (ARR_SIZE / 100) + 1;

    thread::scope(|s| {
        for (batch_idx, batch) in arr.chunks_mut(batch_size).enumerate() {
            s.spawn(move || {
                for (i, val) in batch.iter_mut().enumerate() {
                    *val += batch_idx + i;
                }
            });
        }
    });

    println!("Array Batch Processing: {:?} Item.", arr.len());
    println!("Batch Proccessing took: {:?}", time.elapsed());
}

fn array_bulk_proccessing() {
    let time = Instant::now();

    const ARR_SIZE: usize = 1_00_000;
    let mut arr = [0; ARR_SIZE];

    for (i, val) in arr.iter_mut().enumerate() {
        *val += i;
    }

    println!("Bulk Proccessing took: {:?}", time.elapsed());
}

fn single_val_mutiple_access() {
    let val = Arc::new(Mutex::new(0));
    let mut thrds = vec![];

    
    for _ in 0..10 {
        let val = Arc::clone(&val);
        let thrd = thread::spawn(move || {
            let mut val = val.lock().unwrap();
            *val += 1;
        });
        
        thrds.push(thrd);
    }
    

    for thrd in thrds {
        thrd.join().unwrap();
    }

    println!("val: {:?}", *val.lock().unwrap());
}

/* ========================== 1 ========================
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 1..=10 {
        let tx = tx.clone();
        tx.send(i).unwrap();
    }

    drop(tx);

    let mut values = vec![];

    for val in rx {
        values.push(val);
    }

    println!("values: {:?}", values);
    println!("End..");
}
*/

/* ========================== 2 ========================
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let main = Instant::now();
    const TOTAL_THREADS_NUMBER: usize = 1000;

    let mut thrds = vec![];
    let mut times = vec![];

    for _ in 0..TOTAL_THREADS_NUMBER {
        times.push(Instant::now());
        let thrd = heavy_async_task();
        thrds.push(thrd);
    }

    let mut total_time = Duration::from_millis(0);
    for (thrd, time) in thrds.into_iter().zip(times) {
        thrd.join().unwrap();
        total_time += time.elapsed();
    }

    println!("total num of threads: {:?}", TOTAL_THREADS_NUMBER);
    println!("program overall time: {:?}", main.elapsed());
    println!("sum of individual threads' time: {:?}", total_time);
}

fn heavy_sync_task() {
    for _ in 0..1_000_000 {}
}

fn heavy_async_task() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        heavy_sync_task();
    })
}
*/