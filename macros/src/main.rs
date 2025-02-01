macro_rules! magic_spelling {
    (fire) => {
        println!("Fire spelling!");
    };
    (ice) => {
        println!("Ice spelling!");
    };
    (lightning) => {
        println!("Lightning spelling!");
    };
    (water) => {
        println!("Magic spelling!");
    };
    () => {
        println!("Unknow Magic spelling!");
    };
}

fn main() {
    magic_spelling!(fire);
    magic_spelling!(ice);
    magic_spelling!(lightning);
    magic_spelling!(water);
    magic_spelling!();
}
