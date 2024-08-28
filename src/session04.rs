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

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _=> 1,
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
    let data=value_in_cents(Coin::Penny);
}


