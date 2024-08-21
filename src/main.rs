use std::io;

mod rectangle;
mod slice;
mod session01;
mod shadowing;
mod contratos;
mod session02;
mod session03;
fn main(){
    //rectangle::view();
    //slice::slice_example();
    //session01::conversions(); 
    //shadowing::shad();    
    //contratos::ejemplo();
    //session02::mutability();

    fn main() {
        let my_string = String::from("hello world");
    
        // `first_word` funciona con slices de un string, sean parciales o completos.
        let word = session02::first_word(&my_string[0..6]);
        let word = session02::first_word(&my_string[..]);
        // `first_word` también funciona con referencias de un string, que son equivalentes
        // a un slice completo de un String
        let word = first_word(&my_string);
    
        let my_string_literal = "hello world";
    
        // `first_word` funciona con slices de string literales, sean parciales o completos
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);
    
        // Por que los strings literales son slices de strings,esto también funciona,
        // sin necesidad de usar la sintaxis de slices.
        let word = first_word(my_string_literal);
    }
    let mut para:String=String::from("Holas palabras");
    let ress=session02::first_word(&para);
    //para.push_str("mas");
    println!("{ress}");
}
