fn main() {
    let map_1 : String = String::from("Map Dungeon");
    let borrowing_map : &str = map_1.as_str();

    let mut crabby_map : String = borrowing_map.to_string();

    crabby_map.push_str(" Crabby Dungeon");

    println!("crabby_map : {}", crabby_map);

}
