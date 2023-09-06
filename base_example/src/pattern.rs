trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("pilot");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("wizard");
    }
}

impl Human {
    fn fly(&self) {
        println!("...");
    }
}

pub fn run() {
    println!("----- trait_objects -----");
    let person = Human;
    Human::fly(&person);
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
