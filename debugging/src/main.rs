fn main() {
    let x : i32 = 5;
    let y : i32 = 10;
    let r : i32 = call_level_1(x, y);
    println!("Result: {}", r);
}

fn call_level_1(x: i32, y: i32) -> i32 {
    let z : i32 = 15;
    let r : i32 = call_level_2(x, y, z);
    r
}

fn call_level_2(x: i32, y: i32, z: i32) -> i32 {
    let r : i32 = x + y + z;
    r
}
