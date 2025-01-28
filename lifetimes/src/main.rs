fn longest_map<'a>(map1 : &'a str, map2 : &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else if map1.len() < map2.len() {
        map2
    } else {
        "Crabby can't decide"
    }
}

fn main() { 
    let map1 : &str = "DND";
    let map2 : &str = "DXD";

    let choose_map = longest_map(map1, map2);
    println!("Crabby's longest map : {}", choose_map);
}
