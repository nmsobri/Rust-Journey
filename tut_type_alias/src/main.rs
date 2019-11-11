type Slier = String;
type Student<T> = Vec<T>;

fn main() {
//    let nama: Slier = "Nor Mohd Sobri".to_string();
//
//    let mut students: Student<String> = Vec::new();
//    students.push("Sobri".to_string());
//    students.push("Bulma".to_string());
//
//
// println!("Hello, world!");

    let a = Some(55);

    let b = match a {
        Some(v) => v,
        None => "hello".to_string()
    };
}
