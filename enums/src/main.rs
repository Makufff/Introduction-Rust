enum CrabbyStates {
    Fighting,
    Collecting(u32),
    Defending,
}

impl CrabbyStates {
    fn state_representation(&self){
        match self {
            CrabbyStates::Fighting => println!("Fighting"),
            CrabbyStates::Collecting(amount) => println!("Collecting {}", amount),
            CrabbyStates::Defending => println!("Defending"),
        }
    }
}

fn main() {
    let fighting : CrabbyStates = CrabbyStates::Fighting;
    let collecting : CrabbyStates = CrabbyStates::Collecting(10);
    let defending : CrabbyStates = CrabbyStates::Defending;

    fighting.state_representation();
    collecting.state_representation();
    defending.state_representation();
}
