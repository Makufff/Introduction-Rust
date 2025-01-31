use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let crabby_gold : Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let loot_agents_1 = thread::spawn({
        let crabby_gold_arctifact : Arc<Mutex<i32>> = Arc::clone(&crabby_gold);
        move || {
            let mut gold = crabby_gold_arctifact.lock().unwrap();
            *gold += 100;
        }
    });

    let loot_agents_2 = thread::spawn({
        let crabby_gold_arctifact : Arc<Mutex<i32>> = Arc::clone(&crabby_gold);
        move || {
            let mut gold = crabby_gold_arctifact.lock().unwrap();
            *gold += 200;
        }
    });

    let loot_agents_3 = thread::spawn({
        let crabby_gold_arctifact : Arc<Mutex<i32>> = Arc::clone(&crabby_gold);
        move || {
            let mut gold = crabby_gold_arctifact.lock().unwrap();
            *gold += 200;
        }
    });

    loot_agents_1.join().unwrap();
    loot_agents_2.join().unwrap();
    loot_agents_3.join().unwrap();

    println!("Total gold : {}", crabby_gold.lock().unwrap());
}
