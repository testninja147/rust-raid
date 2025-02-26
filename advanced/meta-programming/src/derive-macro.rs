use xml_derive::Serialize;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct Student {
    id: usize,
    first_name: String,
    last_name: String,
    subjects: Vec<String>,
}
fn main() {
    println!("Derive Macro");
    let student_1 = Student {
        id: 1,
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        subjects: vec![
            String::from("Computer Science"),
            String::from("Mathematics"),
        ],
    };
    println!("Student 1: {:?}", student_1);
    println!("Serialized Data:\n{}", student_1.serialize());
}
