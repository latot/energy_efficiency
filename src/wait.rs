use std::time::{Duration, Instant};

fn solve(x: usize) {
    let mut x = x;
    while x != 1 {
        if (x % 2) == 0 {
            x = x / 2;
        } else {
            x = x * 3 + 1;
        }
    }
}

pub fn sample(x: usize) {
    for _ in 0..x {
        solve(87_654_321_012_345_678);
    }
}

pub fn optimize(t: Duration) -> usize {
    let mut s: usize = 0;
    let bits = std::mem::size_of::<usize>() * 8;
    let mut start_bit = 1;

    for max_bit in 1..bits {
        let now = Instant::now();
        sample(1 << max_bit);
        let elapsed = now.elapsed();
        if elapsed > t {
            start_bit = max_bit - 1;
            s = s + (1 << start_bit);
            break;
        }
    }

    for bit in (1..start_bit).rev() {
        let ss = s + (1 << bit);
        let now = Instant::now();
        sample(ss);
        let elapsed = now.elapsed();
        if elapsed <= t {
            s = ss;
        }
    }

    s
}
