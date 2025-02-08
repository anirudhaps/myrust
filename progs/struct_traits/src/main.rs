struct Student {
    name: String,
    age: i32,
}

trait Reader {
    fn read(&mut self, data: String);
}

trait Writer {
    // default implementation for the trait function
    fn write(&self, data: String) {
        println!("I wrote {data}");
    }
}

/* Student implements Reader trait. Now it has to define read() method since
   its default implementation is not provided by the trait. */
impl Reader for Student {
    fn read(&mut self, data: String) {
        self.name = data;
    }
}

/* Student implements Writer trait. Since the write() method's default
   implementation is provided by the trait, Student struct choses not
   to override it. */
impl Writer for Student {}

impl Student {
    // associated function of the Struct Student
    fn new() -> Self {
        // Self denotes a Student 
        Self {
            name: "Amit".to_string(),
            age: 30,
        }
    }

    // methods
    fn print(&self) {
        println!("name: {}, age: {}", self.name, self.age);
    }

    fn update(&mut self, new_name: String, new_age: i32) {
        self.age = new_age;
        self.name = new_name;
    }
}

fn main() {
    let roll1 = Student {
        name: "Anirudha".to_string(),
        age: 34,
    };
    println!("Roll number-1: name: {}, age: {}", roll1.name, roll1.age);
    let mut roll2 = Student::new(); // way to invoke associated function
    println!("Roll number-2 (default): name: {}, age: {}", roll2.name, roll2.age);
    roll2.name = String::from("Srinivas");
    roll2.age = 26;
    println!("Roll number-2 (modified): name: {}, age: {}", roll2.name, roll2.age);

    let mut stud = Student::new();
    stud.update(String::from("Phani"), 39);
    stud.print();
    stud.read(String::from("Chandra"));
    stud.print();
    stud.write(String::from("something"));
}
