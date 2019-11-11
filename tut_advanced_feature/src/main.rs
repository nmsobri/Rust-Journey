use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn dangerous() {
    println!("Hellor worrrr");
}

//fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//    let len = slice.len();
//
//    assert!(mid <= len);
//
//    return (
//        &mut slice[..mid],
//        &mut slice[mid..]
//    );
//}

fn super_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    let ptr = slice.as_mut_ptr();

    unsafe {
        return (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
        );
    }
}

static mut Counter: i32 = 0;

fn increment_counter() {
    unsafe {
        Counter += 1;
    }
}

fn main() {
    increment_counter();
    increment_counter();
    increment_counter();
    unsafe {
        println!("Counter is {}", Counter);
        println!("Abs of -3 according to C is {}", abs(-3));
    }
//    let mut a = vec![1, 2, 3, 4, 5, 6];
//
//    let (b, c) = super_split_at_mut(a.as_mut_slice(), 3);
//
//    println!("{:?}", b);
//    println!("{:?}", c);
//    unsafe {
//        dangerous();
//    }

//    let mut a = 5;
//
//    let b = &a as *const i32;
//    let c = &mut a as *mut i32;
//
//    unsafe {
//        *c = 999;
//
//        println!("b is {}", *b);
//        println!("c is {}", *c);
//    }
//
////    println!("b is {}", b);
//    println!("a is {}", a);
}
