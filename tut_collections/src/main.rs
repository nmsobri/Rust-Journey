use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug)]
enum Vals {
    Integer(i32),
    Float(f32),
    Text(String),
}

fn main() {
    let text = "hello world wonderful world";
    let mut map_words: HashMap<String, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map_words.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("map_words is {:#?}", map_words);


    let mut hash_map: HashMap<String, i32> = HashMap::new();
    hash_map.insert("one".to_string(), 1);
    hash_map.insert("two".to_string(), 2);
//    hash_map.insert("three".to_string(), 3);

    hash_map.entry("three".to_string()).or_insert(33);

    for elem in &hash_map {
        println!("hashMap elem is {:#?}", elem);
    }

    println!("first hastMap elem is {:#?}", hash_map.get(&"one".to_string()));

    let str = "Здравствуйте".to_string();
    let mut str_vec: Vec<char> = Vec::new();
//    let str_slice = &str[0..4];
//
//    println!("{:?}", str_slice.chars().nth(2));

    for c in str.chars() {
        println!("char is {}", c);
        str_vec.push(c);
    }
    println!("###########################");
    println!("str_vec is {:#?}", &str_vec);

//    let s1 = "Hello".to_string();
//    let s2 = " world".to_string();
//
//    println!("s1[0]", s1[0]);

//    let s3 = format!("{}, {}", s1, s2);

//    println!("{} {}", s1, s2);

    let mut rows: Vec<Vals> = Vec::new();
    rows.push(Vals::Integer(5));
    rows.push(Vals::Float(4.5));
    rows.push(Vals::Text("Hello Foobar".to_string()));

    for elem in rows {
        match elem {
            Vals::Integer(i) => println!("elem is {}", i),
            Vals::Float(i) => println!("elem is {}", i),
            Vals::Text(i) => println!("elem is {}", i)
        }
    }

    let mut vect: Vec<i32> = Vec::new();
    vect.push(3);
    vect.push(5);
    vect.push(7);

    println!("vect(1) is {:?}", vect.get(1));

    if let Some(i) = vect.get(0) {
        println!("got value, and value is {}", i);
    } else {
        println!("no value");
    }

    for i in &vect {
        println!("i is {}", i);
    }

    vect.push(55);
}
