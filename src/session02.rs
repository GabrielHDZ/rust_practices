
//Ownership
//Reglas del compilador de rust

//referencias y prestamos
pub fn types_variables(){
    //stack 
    let _num:i32=-4;
    let mut _cadena:&str="cadena de tipo literal";
    let _verdadero:bool=true;
    //heap
    let mut _cadena_string:String=String::from("cadena inicializada desde String");
    _cadena_string.push_str("rust");
    let _cadena_retorno:String=mutability(_cadena,_cadena_string);
    //println!("cadena 1:{cadena_string}, cadena 2 {cadena_retorno}");
}

pub fn mutability(r1: &str, r2:String)->String{
    let mut s = String::from("hello");

    let r1 = &s; // no hay problema
    let r2 = &s; // no hay problema
    println!("{r1} y {r2}");
    // variables r1 y r2 no se usaran más a partir de aquí

    let r3 = &mut s; // no hay problema
    println!("{r3}");
    s
}
//example slices String
// es una forma de realizar una referencia hacia un fragmento de un String, normalmente 
pub fn first_word(s: &str) -> &str {
    //los valores que tenemos en el slice bytes, es basicamente un arreglo de tipo u8 con los caracteres en codigo ascii
    //de la cadena de texto que se recibio como parametro.
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        //b'' representa el valor del esoacio en ascii, seria el valor 32 de tipo u8, seria parecido a esto: let busqueda:u8=32;,
        //solo que de forma mas optima
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
pub fn run(){
    let my_string = String::from("hello world");
    
        // `first_word` funciona con slices de un string, sean parciales o completos.
        let _word = first_word(&my_string[0..6]);
        let _word = first_word(&my_string[..]);
        // `first_word` también funciona con referencias de un string, que son equivalentes
        // a un slice completo de un String
        let _word = first_word(&my_string);
    
        let my_string_literal = "hello world";
    
        // `first_word` funciona con slices de string literales, sean parciales o completos
        let _word = first_word(&my_string_literal[0..6]);
        let _word = first_word(&my_string_literal[..]);
    
        // Por que los strings literales son slices de strings,esto también funciona,
        // sin necesidad de usar la sintaxis de slices.
        let _word = first_word(my_string_literal);
}
