struct Person {
    name: String,
    surname: String,
    age: u8
}

trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello my name is {0}.{1}", self.name, self.surname);
    }
    fn can_speak(&self) -> bool{   
        if self.age > 2 {
            return true;
        }
        return false;
    }
}
fn main () {
    let person = Person {
        name: String::from("Pickles"),
        surname: String::from("Mr"),
        age: 5
    };
    println!("Can {1}.{0} speak? {2}", person.name, person.surname, person.can_speak() );
}
