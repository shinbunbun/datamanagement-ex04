use rand::Rng;

fn main() {
    println!("Tossing a coin...");
    let mut tail = 0;
    let coin = vec!["Heads", "Tails"];
    for i in 0..3 {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0..2);
        tail += rand;
        println!("Round {}: {}", i + 1, coin[rand]);
    }
    println!("Heads: {}, Tails: {}", 3 - tail, tail);
}