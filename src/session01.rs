/// The function `session01` in Rust defines constants and variables to demonstrate basic usage.
fn session01(){
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
