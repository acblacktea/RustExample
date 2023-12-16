trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn universtiy(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_usename(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My git username is {}",
        student.name(),
        student.universtiy(),
        student.fav_language(),
        student.git_usename()
    )
}

fn main() {}