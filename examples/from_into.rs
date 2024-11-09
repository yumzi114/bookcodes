#[derive(Debug)]
struct Point{
    x:i32,
    y:i32,
}
impl From<(i32,i32)> for Point {
    fn from(tuple:(i32,i32))->Self{
        Point {x:tuple.0, y:tuple.1}
    }
}

fn main(){
    let tuple=(1,2);
    let pt:Point = Point::from(tuple);
    println!("{:?}",pt);

    let pt:Point = tuple.into();
    println!("{:?}",pt);
}