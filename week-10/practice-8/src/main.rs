struct Employee {
    ceo: String,
    company: String,
    age: u32,
}

fn main() {
    let emp1 = Employee {
        company: String::from("Microsoft Corporation"),
        ceo: String::from("Satya Nadella"),
        age: 56,
    };
    display(emp1);
    
    
}

fn display(emp: Employee) {
    println!("Name is: {} company is {} age is {}", emp.ceo, emp.company, emp.age);
}
