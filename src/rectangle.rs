struct Unit;
struct Point(f32,f32);
#[derive(Debug)]
struct Rectangle{
    width:f64,
    height:f64,
}

fn calculo_rectangulo(figure:&Rectangle)->u32{
    (figure.width * figure.height) as u32 
}

pub fn view(){
    let rectangulo1=Rectangle{
        width:32.5,
        height:55.32
    };
    let scale =2;
    let rect2:Rectangle=Rectangle{
        width:dbg!(3.3 * scale as f64),
        height:12.3
    };
    println!("Hello, world!, {:?}",calculo_rectangulo(&rectangulo1));
    dbg!(&rect2);
}