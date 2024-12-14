use std::fs::File;
use std::io::Write;

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    // Create a vector of students
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        // Add more students here
    ];

    // Display student details
    println!("Student Name\tMatric. Number\tDepartment\tLevel");
    for student in &students {
        println!("{}\t{}\t{}\t{}", student.name, student.matric_number, student.department, student.level);
    }

    // Create a file to write the data
    let mut file = File::create("students.txt").expect("Failed to create file");

    // Write student details to the file
    for student in &students {
        writeln!(file, "{}\t{}\t{}\t{}", student.name, student.matric_number, student.department, student.level).expect("Failed to write to file");
    }
}