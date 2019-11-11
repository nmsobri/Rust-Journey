trait Wizad {
    fn fly();
}

trait Pilot {
    fn fly();
}

struct Human {}

impl Human {
    fn fly() {
        println!("Human waving hand furiously");
    }
}

impl Pilot for Human {
    fn fly() {
        println!("This is your captain speaking");
    }
}

impl Wizad for Human {
    fn fly() {
        println!("Up");
    }
}


fn main() {
    Human::fly();
    <Human as Pilot>::fly();
    <Human as Wizad>::fly();
}
