use std::{io, num::ParseIntError};

struct contract{
    month:String,
    name:String,
    is_active:bool,
    amount:i64
}
pub fn ejemplo(){
    //example array, solo guarda datos del mismo tipo
    let months = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];
    //example tupla, puede guardar datos de diferentes tipos
    let contrato:(&str,i64,bool)=("carlos",2_500,false);
    let (name,amount,status)=contrato;

    //declaramos una variable de tipo String, que se inicializa en el heap
    let mut entrada=String::new();
    println!("ingrese el mes de consulta de datos");
    io::stdin()
        .read_line(&mut entrada)
        .expect("Mes no registrado, para el usuario");       
    let s2=entrada.clone();
    println!("{entrada}");

    //stack
    let s3="mundo";
    let s4=s3;
    
    println!("{s3} , {s4}");

    println!("values {:?}  {:?}", contrato.0, contrato.1);
    

}

fn example2(){
    const MAX_ITERATORS:u32=50;  

    let caracter='C';
    println!("{}",caracter);
    let caracter=34;
    println!("{}",caracter);
    
    
}