use std ::fs;
use std ::io;

struct Student{
    name: String,
    age: u8,
    is_active: bool,
}

impl Student{
    // We return Result<Student, String> (String is our error type)
    fn create_student(name: String, age: u8) -> Result<Student, String> {
        if age<18{
          return Err(String::from("age is not valid. student must be older than 18"))
        }else{
            Ok(Student{
                name,
                age,
                is_active:true
            })
        }
       
}

}
// so this is how you read a file in rust you use fs::Read_to_string(then the file name in string form).
// it has inbuilt error handling so you can use? and it will return error if the file is not found
fn load_task()-> Result<String,io::Error>{
    let content = fs::read_to_string("task.txt")?;
    Ok(content)
}

fn save_student(student: &Student)-> Result<(),io::Error>{
    let content = format!("Name:{}\nAge:{}\nis_active{}",student.name,student.age,student.is_active);
    fs::write("student.txt",content)?;
    Ok(())
}


// Main must return Result to use `?`
// so we have to use box<dyn std::error ::Error> to return the inbuilt error from the function we cant just mix error types 
fn main() -> Result<(), Box<dyn std::error::Error>> {
 
    // Call Student::create_student (capital 'S')
    // create_student takes (String, u8)
    let student1 = Student::create_student(String::from("john"), 25)?;
    
    println!("Created student: {}", student1.name);
    
    save_student(&student1)?;

    match load_task(){
        Ok(content)=>println!("{}",content),
        Err(e)=> println!("error loading task {}",e)
    }

    Ok(())
}
// learnt a few things here main must return result to use ?
// i also learnt how to use a struct in a function using impl{with the function inside and we would be returning the struct }