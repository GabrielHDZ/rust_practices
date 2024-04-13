mod slice;
mod session01;
mod shadowing;
fn main() {
    let rectangulo1=Rectangle{
        width:32.5,
        height:55.32
    };
    let scale =2;
    let rect2:Rectangle=Rectangle{
        width:dbg!(3.3 * scale as f32),
        height:12.3
    };
    println!("Hello, world!, {:?}",calculo_rectangulo(&rectangulo1));
    dbg!(&rect2);

    //slice::slice_example();
    //session01::conversions(); 

    shadowing::shad();    
}
struct Unit;
struct Point(f32,f32);
#[derive(Debug)]
struct Rectangle{
    width:f32,
    height:f32,
}

fn calculo_rectangulo(figure:&Rectangle)->u32{
    (figure.width * figure.height) as u32 
}
