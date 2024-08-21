//in this session 
struct Recomended;
struct Size(i32,i32,i32);
struct Product{
        name:String,
        price:u32,
        stock:u32,
    }
pub fn build_product(name:String,price:u32)->Product{
    Product{
        name,
        stock:1,
        price,
    }
}
pub fn estructuras(){
    //esto es una sentencia
    let mut galleta_gamesa=Product{
        name:String::from("Galletas Gamesa"),
        stock:200,
        price:15,
    };
    let mut galleta_puebla:Product=build_product(String::from("Puebla"), 18);

    let galleta_lara:Product=Product{
        name:String::from("Lara"),
        ..galleta_puebla
    };
    let comentario=Recomended;
    
    galleta_gamesa.price=12;

 }