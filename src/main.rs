fn main() {
    let rectangulo1=Rectangle{
        width:32.5,
        height:55.32
    };
    println!("Hello, world!, {:?}",calculo_rectangulo(&rectangulo1));
}
struct Unit;
struct Point(f32,f32);
struct Rectangle{
    width:f32,
    height:f32,
}

fn calculo_rectangulo(figure:&Rectangle)->u32{
    (figure.width * figure.height) as u32
}
