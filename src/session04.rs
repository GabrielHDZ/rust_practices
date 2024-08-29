//Enums
#[derive(Debug)]
enum Soldadura {
    MIG(String,String),
    MMG(String),
}
impl Soldadura {
    fn call(&self){
        println!("implemetnt  {:?}",&self);
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quarter from {state:?}!");
            25
        },
    }
}
pub fn run(){
    let soldador1=Soldadura::MIG(String::from("Microalambre"),String::from("Argon"));

    let soldador2=Soldadura::MMG(String::from("Electrodo revestido"));
    soldador2.call();
    soldador1.call();
    let _number1:Option<i32>=Option::Some(15);
    let _number2:Option<i32>=Option::None;
    println!("{:?}  {:?}",_number1,_number2);
    let data=value_in_cents(Coin::Dime);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
pub fn run_match(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

pub fn roll(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}




