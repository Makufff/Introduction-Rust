struct Crabby {
    name : String ,
    health : u8 ,
}

impl Crabby {

    fn take_damage(&mut self, damage : u8) {
        self.health = self.health.saturating_sub(damage);
        return ;
    }

    fn healing (&mut self, heal : u8) {
        if self.health >= 100 {
            self.health = 100 ;
            return ;
        }
        self.health = self.health.saturating_add(heal);
        return ;
    }

}

fn main() {
    let mut crabby = Crabby {
        name : String::from("Crabby"),
        health : 0,
    };

    crabby.take_damage(10);
    println!("Crabby's health : {}", crabby.health);

    crabby.healing(10);
    println!("Crabby's health : {}", crabby.health);
}
