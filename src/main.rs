use std::iter;
use std::thread;
use std::time::Duration;

use slice_deque::SliceDeque;

fn time(mut f: impl FnMut() -> ()) -> usize {
    use time;

    let start = time::precise_time_ns() / 1_000;
    f();
    let end = time::precise_time_ns() / 1_000;
    (end - start) as usize
}


fn bench(
    thread: usize,
    initial_size: f64,
    initial_grow: f64,
    increment_grow: f64,
    initial_truncate: f64,
    increment_truncate: f64,
) {
    let mut queue = SliceDeque::new();
    queue.extend(iter::repeat(0).take(initial_size as usize));

    let mut grow = initial_grow;
    let mut truncate = initial_truncate;

    let start = time::precise_time_ns() / 1_000;

    loop {
        let time_grow = time(|| queue.extend(iter::repeat(0).take(grow as usize)));
        println!("{} GROW  {} {} {} {}", time::precise_time_ns() / 1_000 - start, thread, time_grow, grow as usize, queue.len());

        let time_truncate = time(|| queue.truncate_front(queue.len() - truncate as usize));
        println!("{} TRUNC {} {} {} {}", time::precise_time_ns() / 1_000 - start, thread, time_truncate, truncate as usize, queue.len());

        grow += increment_grow;
        truncate += increment_truncate;
    }
}

fn main() {
    let args: Vec<String> = std::env::args().into_iter().collect();
    if args.len() != 7 {
        println!("Found {} args, expected 6", args.len());
        println!("Usage: {} n_threads initial_size initial_grow increment_grow initial_truncate increment_truncate", args[0]);
        return;
    }

    let n_threads: usize = args[1].parse().expect("n_threads");
    let initial_size: f64 = args[2].parse().expect("initial_size");
    let initial_grow: f64 = args[3].parse().expect("initial_grow");
    let increment_grow: f64 = args[4].parse().expect("increment_grow");
    let initial_truncate: f64 = args[5].parse().expect("initial_truncate");
    let increment_truncate: f64 = args[6].parse().expect("increment_truncate");

    println!("# global_time action thread time[us] grow/truncate_size total_queue_size");

    for thread in 0..n_threads {
        thread::spawn(move || bench(thread, initial_size, initial_grow, increment_grow, initial_truncate, increment_truncate));
    }

    thread::sleep(Duration::from_secs(1000000))
}
