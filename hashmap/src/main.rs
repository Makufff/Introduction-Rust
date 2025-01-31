use std::collections::HashMap;

fn main() {
    let mut backpack : HashMap<&str , i32> = HashMap::new();

    backpack.insert("Sword", 2);
    backpack.insert("Shield", 1);
    backpack.insert("Potion", 1);
    backpack.insert("Key", 1);

    println!("{:?}" , backpack);

    if let Some(x) = backpack.get("Sword") {
        println!("Sword : {}", x);
    }

    if let Some(x) = backpack.get_mut("Sword") {
        *x *= 3;
        println!("Update : {:?}", backpack);
    }

    for (k , v) in backpack.iter() {
        println!("{} : {}", k , v);
    }

}
