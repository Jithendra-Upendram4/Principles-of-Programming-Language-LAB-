struct Student {
    name: String,
    grade: char,
}

fn main() {
    let mut students = vec![
        Student {
            name: String::from("Alice"),
            grade: 'A',
        },
        Student {
            name: String::from("Bob"),
            grade: 'B',
        },
    ];

    display_students(&students);
    update_grade(&mut students[1], 'A');
    display_students(&students);
}

fn display_students(students: &Vec<Student>) {
    for student in students {
        println!("Name: {}, Grade: {}", student.name, student.grade);
    }
}

fn update_grade(student: &mut Student, new_grade: char) {
    student.grade = new_grade;
}

