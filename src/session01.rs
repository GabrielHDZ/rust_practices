/// The function `session01` in Rust defines constants and variables to demonstrate basic usage.
pub fn session01(){
    const MAX_POINTS:u32=100_000;
    let mut counter=1;
    counter+=1;
    let counter=counter*2;
    println!("max points {:?}",MAX_POINTS);
    println!("counter value {:?}", counter);    
}

pub fn primitives(){
    //INTEGER signed i8 i16 i32 i64 i128 isize
    let number_int_signed:i32 =32;

    //unsigned u8 u16 u32 u64 u128 usize
    let number_int_unsigned:u32 =100;

    //floating
    let number_f:f32=5.55;
    let is_true:bool=true;
    let character:char='A';
    let mut array_1:[i32;4]=[12,12,23,4];
    array_1[2]=3;
    let tupla:(i32,bool,&str)=(2,true,"cierto");

    //Slice provide a flexible way to work  with arrays
    let cp_array:&[i32]=&array_1[1..=3];
    for i in cp_array{
        println!("value in slice referenced array {:?}",i);
    }

    //STRNG secuence of characters
    //string slice or String literals 
    let cadena_1:&str="cadena de caracteres(referencia)";
    //owned strings dynamic alocate(owned)
    let cadena_2:String=String::from("Cadena de caracteres(String)");

}

pub fn referencias(){
    let x=43;
    let mut y=12;
    let referencia=&x;
    let mut_1:&i32=&y;
    let mut_referencia=&mut y;
    *mut_referencia += 5;

    // Accessing the value through the reference
    println!("Value through immutable reference: {}", *referencia);  // 10
    println!("Value through mutable reference: {}", y);  // 15
}

pub fn conversions(){
    let decimal: f32 = 64.31;
    let integer = decimal as u16;
    println!("decimal = {}", decimal); // 64.31
    println!("integer = {}", integer); // 64

    let character: char = 'A';
    let integer = character as u8;
    println!("character = {}", character); // A
    println!("integer = {}", integer); // 65

    let integer: u8 = 65;
    let character = integer as char;
    println!("integer = {}", integer); // 65
    println!("character = {}", character); // A

    let integer: i32 = 65;
    let character = integer as char;  // invalid cast
    // only `u8` can be cast as `char`, not `i32`
    println!("integer = {}" , integer);
    println!("character = {}", character);

    let boolean1: bool = false;
    let boolean2: bool = true;
    let integer1 = boolean1 as i32;
    let integer2 = boolean2 as i32;
    println!("boolean1 = {}", boolean1); // false
    println!("boolean2 = {}", boolean2); // true
    println!("integer1 = {}", integer1); // 0
    println!("integer2 = {}", integer2); // 1

    let decimal: f32 = 65.321;
    let character = decimal as char; // Error: only `u8` can be cast as `char`, not `f32`
    println!("decimal = {}", decimal);
    println!("character = {}", character);


}