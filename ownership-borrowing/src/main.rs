fn main() {

    // Ownership Rules
    // Every value in rust is Owner by Variable
    // Only one Owner at time : This Ensures memory is managed in a predictable way ;
    // Onwership ends when the variable goes out of scope : When the Owner (a variable) is longer needed , the data is automatically cleaned up

    // Borrowing Rules
    // You can Borrowing a value by Reference it "&" 
    // The Owner can lend immutable or mutable references

    let mut treasure : String = String::from("gold coins");

    let friend1 : &String = &treasure;
    let friend2 : &String = &treasure;

    println!("Friend 1 sees : {}", friend1);
    println!("Friend 2 sees : {}", friend2);

    let trusted_friend : &mut String = &mut treasure;

    trusted_friend.push_str(" and silver coins");

    println!("Trusted friend updates : {}", trusted_friend);
}
