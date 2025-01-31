fn open_chest (is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some("You found a key".to_string())
    }
}

fn open_door(is_danger : bool) -> Result<String, String> {
    if is_danger {
        Err("You found a monster".to_string())
    } else {
        Ok("You found a treasure".to_string())
    }
}

fn main() {
    let _chest_result : bool = match open_chest(true){
        Some(x) => {
            println!("{}", x);
            true
        },
        None => false
    };

    println!("{}", _chest_result);

    let _door_result = match open_door(false){
        Ok(x) => x , 
        Err(x) => panic!("{}", x) ,
    };

    println!("{}", _door_result);

}
