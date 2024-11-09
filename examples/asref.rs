struct Person{
    name:String,
    age:u32
}

impl AsRef<str> for Person {
    fn as_ref(&self)->&str{
        &self.name
    }
}
fn greet_person<P:AsRef<str>>(person:P){
    println!("{}",person.as_ref());
}

fn main(){
    let person=Person{name:String::from("name"),age:30};
    
    greet_person(person);

    greet_person(String::from("TEST"));

    greet_person("Tsese");
}