fn main() {
    let crabby_pack : i32 = 2 ;
    let crabby_balance : f64 = 320.0 ;

    let getAllbalance : f64 = crabby_pack as f64 * crabby_balance;

    let crabby_message : String = String::from("Crabby is a crabby crab");

    let message_demo1: String = String::from("Hello World!");

    let message_demo2 : String = "Hello World!".to_string();

    let message_demo3 : &str = "Hello World!";

    let message_demo4 : String = format!("Pointless {}", "message");

    println!("{} have {} pack, {}$ {}$", crabby_message , crabby_pack , crabby_balance , getAllbalance);
}
