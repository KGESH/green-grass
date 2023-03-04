mod app;

use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Start commit ...");

    let loop_count = rand::thread_rng().gen_range(8..=21);

    for _ in 0..loop_count {
        app::run();
        sleep(Duration::from_secs(1));
    }

    println!("Commit done.");
}
