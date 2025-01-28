fn main() {
    let mut carbby_backpack : Vec<&str> = vec!["Sword", "Shield", "Potion", "Key"];

    println!("{:?}" , carbby_backpack);
    println!("length :{}" , carbby_backpack.len());
    println!("size :{}" , carbby_backpack.capacity());

    carbby_backpack.push("Gold");
    carbby_backpack.push("Ruby");

    println!("{:?}" , carbby_backpack);
    println!("length :{}" , carbby_backpack.len());
    println!("size :{}" , carbby_backpack.capacity());
    
    carbby_backpack.pop();

    println!("{:?}" , carbby_backpack);
    println!("length :{}" , carbby_backpack.len());
    println!("size :{}" , carbby_backpack.capacity());

    carbby_backpack.remove(0);

    println!("{:?}" , carbby_backpack);
    println!("length :{}" , carbby_backpack.len());
    println!("size :{}" , carbby_backpack.capacity());

    carbby_backpack.push("dummy");

    println!("{:?}" , carbby_backpack);
    println!("length :{}" , carbby_backpack.len());
    println!("size :{}" , carbby_backpack.capacity());

    carbby_backpack.push("dummy");
    carbby_backpack.push("dummy");
    carbby_backpack.push("dummy");
    carbby_backpack.push("dummy");
    carbby_backpack.push("dummy");

    println!("{:?}" , carbby_backpack);
    println!("length :{}" , carbby_backpack.len());
    println!("size :{}" , carbby_backpack.capacity());
}
