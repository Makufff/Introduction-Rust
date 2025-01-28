fn main() {

    // if else statements 

    // let weather : String = String::from("sunny");
    // let weather : String = String::from("rainy");
    // let weather : String = String::from("stormy");

    // if weather == "sunny" {println!("Crabby will cross the river by swimming!");}
    // else if weather == "rainy" {println!("Crabby will build a bridge to stay dry.");}
    // else if weather == "stormy" {println!("Crabby will wait for better weather.") ;}

// ================================================================================================

    // match statements

    // let enemy : &str = "troll" ;
 
    // match enemy {
    //     "goblin" => println!("Crabby will fight the goblin!"),
    //     "troll" => println!("Crabby will run away from the troll!"),
    //     "dragon" => println!("Crabby will hide from the dragon!"),
    //     _ => println!("Crabby will run away from the enemy!"),
    // };

// ================================================================================================

    // loop statements

    let mut wood : i32 = 0 ;

    loop {
        
        if wood < 10 {
            wood = wood + 1 ;
            println!("Crabby pick-up wood, Crabby have wood now : {}", wood);
        }
        else {
            println!("Crabby Aventure!");
            break;
        }
    }
}
