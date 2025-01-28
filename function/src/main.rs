fn main() {
    let result = crabby_task("gathering coins" , 12 ) ;
    println!("The result is {}", result);

    let result_1 = crabby_task("cooking" , 30 ) ;
    println!("The result is {}", result_1);
    let result_2 = crabby_task("hunting" , 60 ) ;
    println!("The result is {}", result_2);
}

fn crabby_task(task : &str , time : i32) -> String {
    format!("Crabby has sussessfully completed the task of {} in {} minutes!", task, time)
}
