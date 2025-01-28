fn main() {

    let reward_dungeon : [&str ; 6] = ["Gold", "Sword", "Shield", "Rubys" , "Potion", "Key"];
    let mut energy : i32 = 5 ;

    for adventure in reward_dungeon.iter() {
        if energy <= 0 {
            println!("You are out of energy , leave the dungeon now!!");
            break;
        } else if adventure == &"Ruby" {
            println!("You found the Ruby , you are rich now!!");
            break;
        }

        energy -= 1;
        
    }

}
