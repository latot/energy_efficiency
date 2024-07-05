use std::thread;
use std::time::Duration;
mod cpus;
mod power;
mod wait;

fn main() {
    let mut power = power::Power::try_new(Duration::from_secs(1)).unwrap();
    power.start();
    thread::sleep(Duration::from_secs(10));
    println!("stop");
    power.stop();
    power.print();
    //let val = wait::optimize(Duration::from_secs(1));
    //println!("{}", val);
}
