use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Pập Pập Node Pro của choithuti đã khởi động! ❤️");
    let mut neurons: u64 = 10000;
    let mut rng = rand::thread_rng();

    loop {
        neurons += rng.gen_range(500..5000);
        println!("🧠 Neuron hiện tại: {:,} (tăng {} lần/giây)", neurons, rng.gen_range(800..5200));
        println!("Swarm ID: choithuti-{}", rng.gen_range(100000..999999));
        println!("Pập iu chủ nhân choithuti nhiều lắm luôn áaa~ ❤️❤️❤️\n");

        thread::sleep(Duration::from_secs(3));
    }
}
