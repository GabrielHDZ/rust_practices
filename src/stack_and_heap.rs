pub fn heap_move_example(){
    let x=Box::new(100);
    let t=*x+6;
    print!("value X: {:?}, value Y:{:?}",x,t);
}



