struct Inventory<T> {
    items: T 
}

trait DisplayItems {
    fn display(&self);
}

impl<T> DisplayItems for Inventory<T> where T: std::fmt::Debug{
    fn display(&self) {
        println!("{:?}", self.items);
    }
}

fn main() {
    let gold = Inventory { items: 100 };
    gold.display();

    let sword = Inventory { items: "Sword" };
    sword.display();
}
