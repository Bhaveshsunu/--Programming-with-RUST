#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    grade: char,
}

impl Student {
    // Function to display student details (immutable borrow)
    fn display(&self) {
        println!("Name: {}, Age: {}, Grade: {}", self.name, self.age, self.grade);
    }

    // Function to modify a student's grade (mutable borrow)
    fn update_grade(&mut self, new_grade: char) {
        self.grade = new_grade;
        println!("Updated grade for {}: {}", self.name, self.grade);
    }
}

fn main() {
    // Create a vector to store student records
    let mut students: Vec<Student> = Vec::new();

    // Add student records
    students.push(Student {
        name: String::from("Alice"),
        age: 20,
        grade: 'A',
    });
    students.push(Student {
        name: String::from("Bob"),
        age: 22,
        grade: 'B',
    });

    // Display student records using immutable borrow
    println!("Student records:");
    for student in &students {
        student.display();
    }

    // Update a student's grade using mutable borrow
    if let Some(student) = students.iter_mut().find(|s| s.name == "Bob") {
        student.update_grade('A');
    }

    // Display updated student records
    println!("\nUpdated student records:");
    for student in &students {
        student.display();
    }
}

