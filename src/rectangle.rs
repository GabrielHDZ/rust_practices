struct Unit;
struct Point(f32,f32);
#[derive(Debug)]
struct Rectangle{
    width:f64,
    height:f64,
}

impl Rectangle{
    fn area(&self)->u32{
        (self.width * self.height) as u32 
    }
    fn width(&self) -> bool {
        self.width > 0.0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //fn asociada
    fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

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
    println!("Hello, world!, {:?}",rectangulo1.area());
    dbg!(&rect2);

    if rectangulo1.width() {
        println!("The rectangle has a nonzero width; it is {}", rectangulo1.width);
    }
}