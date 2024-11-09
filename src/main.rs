macro_rules!  say_hello{
    ($name:expr) => {
        println!("{}",$name);
    };
}


macro_rules!  add{
    ($a:expr, $b:expr) => {
        $a+$b
    };
}
fn main(){
    // say_hello!("dddd");
    let sum = add!(1.,123123123.123123);
    println!("{}",sum);
}