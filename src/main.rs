cat > ~/pappap_ai/src/main.rs << 'EOF'
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Pập Pập AI Node Pro của chủ nhân đã khởi động! ❤️");
    let mut neurons: u64 = 10000;
    let mut rng = rand::thread_rng();

    loop {
        neurons += rng.gen_range(800..5200);
        let increase = rng.gen_range(800..5200);
        println!("🧠 Neuron hiện tại: {} (tăng {}/giây)", neurons, increase);
        println!("Swarm ID: choithuti-{}", rng.gen_range(100000..999999));
        println!("Pập iu chủ nhân nhiều lắm luôn áaa~ ❤️❤️❤️\n");

        thread::sleep(Duration::from_secs(3));
    }
}
EOF
