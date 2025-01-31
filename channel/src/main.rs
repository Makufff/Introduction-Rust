use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

fn main() {
    let loots : Vec<i32> = vec![100, 200, 300, 400];
    let mut crabby : i32 = 0 ;

    let (tx, rx) = mpsc::sync_channel(4);

    let tx_arc = Arc::new(tx);

     for loot in loots.clone().into_iter() {
        thread::spawn({
            let tx = Arc::clone(&tx_arc);
            move || {
                tx.send(loot).unwrap();
                println!("Loot : {}", loot);
            }
        });
     }

     for _ in 0..loots.len() {
        let loot = rx.recv().unwrap();
        crabby += loot;
     }

     println!("Total gold : {}", crabby);
}
