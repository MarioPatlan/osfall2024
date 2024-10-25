#[derive(Debug)]

struct Student {
    name: String,
    major: String,
}

impl Student {

    fn new_student(name: String, major: String) -> Self {
        Self {
            name: name,
            major: major,
        }
    }

    fn introduce_yourself(&self) {
        println!("My name is {} and my major is {}.",self.name, self.major);
    }

    fn change_major (major: String)

}

fn main() {

    let name = String::from("Mario Patlan");
    let major = String::from("Comp Sci");

    let s = Student::new_student(name, major);

    println!("{:?}", s);

    s.introduce_yourself();

}