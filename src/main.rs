extern crate time;

use std::thread;

fn fib(n: i64) -> i64 {
	match n {
		0 | 1 => n,
		x => fib(x-1) + fib(x-2)
	}
}

fn main() {
	let start = time::precise_time_s();
	let mut threads = Vec::new();
	for n in 0..40 {
		threads.push(
			thread::spawn(move|| {
	    		println!("fib({}) = {}", n, fib(n));
	    	}));
	}

	println!("Spawned all the threads.");

	for t in threads {
		let _ = t.join();
	}
	let end = time::precise_time_s();
	let elapsed = end - start;
	println!("Elapsed {:?} ns", elapsed);
}