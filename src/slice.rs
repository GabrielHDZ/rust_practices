pub fn slice_example(){
    let my_array=[1,2,3,4,5];
    let my_slice=&my_array[1..3];
    let my_second_slice=&my_array[..];
    println!("{:?}",my_array);
    println!("{:?}",my_slice);
    println!("{:?}",my_second_slice);
}