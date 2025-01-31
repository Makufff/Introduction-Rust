trait Gear {
    fn use_gear(&self);
}

struct Sword ;
struct Bow ;
struct Potion ;

// fn use_gear<T : Gear >(item : T) {
//     item.use_gear();
// }

fn use_gear(item : Box<dyn Gear>) {
    item.use_gear();
}

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Swing the sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("Shoot the bow");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("Drink the potion");
    }
}

fn main() {

    // let crabby_sword = Sword;
    // let crabby_bow = Bow;
    // let crabby_potion = Potion;

    let crabby_sword : Box<Sword> = Box::new(Sword);
    let crabby_bow : Box<Bow> = Box::new(Bow);
    let crabby_potion : Box<Potion> = Box::new(Potion);
    use_gear(crabby_sword);
    use_gear(crabby_bow);
    use_gear(crabby_potion);
}
