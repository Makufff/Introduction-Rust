trait weapon {
    fn attack(&self);
}

struct sword;

impl weapon for sword {
    fn attack(&self) {
        println!("Attacking with sword");
    }
}

struct staff;

impl weapon for staff {
    fn attack(&self) {
        println!("Attacking with staff");
    }
}

struct warrior {
    health : i32,
    strangth : i32,
    intelligence : u8,
    weapon: Box<dyn weapon>
}

impl warrior {
    fn new() -> Self {
        Self {
            health: 100,
            strangth: 10,
            intelligence: 5,
            weapon: Box::new(sword)
        }
    }

    fn health_increase(&mut self , val : i32) {
        self.health += val;
    }

    fn health_decrease(&mut self , val : i32) {
        self.health = self.health.saturating_sub(val);
    }
}

struct mage {
    health : i32,
    strangth : i32,
    intelligence : u8,
    weapon: Box<dyn weapon>

}

impl mage {
    fn new() -> Self {
        Self {
            health: 50,
            strangth: 5,
            intelligence: 10,
            weapon: Box::new(staff)
        }
    }

    fn health_increase(&mut self , val : i32) {
        self.health += val;
    }

    fn health_decrease(&mut self , val : i32) {
        self.health = self.health.saturating_sub(val);
    }
}

struct healer {
    health : i32,
    strangth : i32,
    intelligence : u8,
    weapon: Box<dyn weapon>
}

impl healer {
    fn new() -> Self {
        Self {
            health: 75,
            strangth: 5,
            intelligence: 10,
            weapon: Box::new(staff)
        }
    }

    fn health_increase(&mut self , val : i32) {
        self.health += val;
    }

    fn health_decrease(&mut self , val : i32) {
        self.health = self.health.saturating_sub(val);
    }
}

fn spacial_attack(weapon: Box<dyn weapon>) {
    weapon.attack();
}

fn main() {
    let mut player1 = warrior::new();
    let mut player2 = mage::new();
    let mut player3 = healer::new();

    player1.weapon.attack();
    player2.weapon.attack();
    player3.weapon.attack();

    println!("Player 1 health: {}", player1.health);
    println!("Player 2 health: {}", player2.health);
    println!("Player 3 health: {}", player3.health);

    println!("=============================");

    player1.health_decrease(10);
    player2.health_decrease(10);
    player3.health_decrease(10);

    println!("Player 1 health: {}", player1.health);
    println!("Player 2 health: {}", player2.health);
    println!("Player 3 health: {}", player3.health);

    println!("=============================");

    player1.health_increase(10);
    player2.health_increase(10);
    player3.health_increase(10);

    println!("Player 1 health: {}", player1.health);
    println!("Player 2 health: {}", player2.health);
    println!("Player 3 health: {}", player3.health);

    println!("=============================");

    spacial_attack(player1.weapon);
    spacial_attack(player2.weapon);
    spacial_attack(player3.weapon);

    println!("Player 1 health: {}", player1.health);
    println!("Player 2 health: {}", player2.health);
    println!("Player 3 health: {}", player3.health);

    println!("=============================");
}