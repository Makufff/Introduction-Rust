use tokio ;

async fn gather_herbs() {
    println!("Gathering herbs");
}

async fn brew_potion() {
    println!("Brewing potion");
}

async fn cast_spell() {
    println!("Casting spell");
}

#[tokio::main]
async fn main() {
    let task_1 = tokio::spawn(gather_herbs());
    let task_2 = tokio::spawn(brew_potion());
    let task_3 = tokio::spawn(cast_spell());

    let _ = tokio::join!(task_1, task_2, task_3);
}
