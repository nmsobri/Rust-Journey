use typename::TypeName;

enum Foo<T> {
    Yes(T),
    No(T),
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    pub fn get_x(&self) -> &T {
        return &self.x;
    }

    pub fn get_y(&self) -> &U {
        return &self.y;
    }

    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        return Point {
            x: self.x,
            y: other.y,
        };
    }
}

fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for elem in list.iter() {
        if elem > largest {
            largest = elem;
        }
    }

    return &largest;
}

//fn find_largest(list: &[i32]) -> i32 {
//    let mut largest = list[0];
//
//    for &elem in list.iter() {
//        if elem > largest {
//            largest = elem;
//        }
//    }
//
//    return largest;
//}

//fn find_largest_number(list: &[i32]) -> i32 {
//    let mut largest = list[0];
//
//    for &elem in list.iter() {
//        if elem > largest {
//            largest = elem;
//        }
//    }
//
//    return largest;
//}
//
//fn find_largest_char(list: &[char]) -> char {
//    let mut largest = list[0];
//
//    for &elem in list.iter() {
//        if elem > largest {
//            largest = elem;
//        }
//    }
//
//    return largest;
//}

fn main() {
    let v1 = vec![1, 3, 5, 7];
    let v2 = vec!['a', 'c', 'e', 'v'];

    let largest = find_largest(&v1);
    println!("Largest is {}", largest);

    let largest = find_largest(&v2);
    println!("Largest is {}", largest);
//    let p: Point<u32, f32> = Point { x: 4, y: 9.3 };
//    let q: Point<String, char> = Point { x: "Hello".to_string(), y: 'c' };
//    let r: Point<u32, char> = p.mixup(q);
//    println!("r is {:#?}", r);
//    let p: Foo<bool> = Foo::No(true);
//    let q: Foo<u32> = Foo::Yes(23);
//    let p: Point<u32, f32> = Point { x: 4, y: 4.5 };
//    let q: Point<f32, String> = Point { x: 4.5, y: "hello".to_string() };
//    let number_list = vec![34, 50, 25, 100, 65];
//    let result = find_largest(&number_list);
//    println!("The largest number is {}", result);
//
//    let char_list = vec!['y', 'm', 'a', 'q'];
//    let result = find_largest(&char_list);
//    println!("The largest char is {}", result);

//    let list = vec![1, 3, 5, 8, 6, 9, 15];
//    let mut largest = find_largest(&list);
//    println!("Largest is {}", largest);
//
//
//    let list2 = vec![56, 34, 99, 54, 44, 33, 4, 534];
//    let mut largest2 = find_largest(&list2);
//    println!("Largest2 is {}", largest2);
}
