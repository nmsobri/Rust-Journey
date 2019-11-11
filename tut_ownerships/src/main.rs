fn main() {
    let arr = [3, 5, 6, 8, 9];

    let arr2 = &arr[1..4];
    println!("arr2 is {:?}", arr2);
    let mut s = String::from("hello world");

    let word = first_word(&s[6..]); // word will get the value 5

    println!("word is {}", word);

//    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally
    // invalid!

    println!("word is {}", word);
    println!("s is {:?}", &s[0..5]);
}

fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();
    for (index, &elem) in bytes.iter().enumerate() {
        if elem == b' ' {
            return &str[0..index];
        }
    }

    return &str[..];
}

fn take_ownership(some_string: String) {
    println!("The string is: {}", some_string);
}

fn give_ownership() -> String {
    let str = String::from("Hello");
    return str;
}

fn take_and_give_ownership(str: String) -> String {
    return str;
}

fn calculate_length(str: String) -> usize {
    let len = str.len();
    return len;
}

fn calculate_length2(str: &String) -> usize {
    return str.len();
}


fn make_copy(some_int: i32) {
    println!("The integer is {}", some_int);
}

fn change(str: &mut String) {
    str.push_str(", world");
}
