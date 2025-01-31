use modules_crate::weapons::weapons::sword;
use modules_crate::potions::health::drink;
use modules_crate::armor::shield;

mod protections {
    pub mod armor {
        pub fn block() {
            println!("You block with the armor.");
        }
    }
}

fn main() {
    // module in main.rs file
    protections::armor::block();
    // module in lib.rs
    shield::block();
    // module in src/weapons.rs
    sword::swing();
    // module in src/potions/health.rs (folder base)
    drink();
}
