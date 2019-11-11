use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let greet = "Hello world".to_string();
        transmitter.send(greet);
    });

    let received_greet = receiver.recv().unwrap();
    println!("Received greet is {}", received_greet);

    //    let counter = Arc::new(Mutex::new(0));
    //    let mut handles = vec![];
    //
    //    for _ in 0..10 {
    //        let counter = Arc::clone(&counter);
    //        let handle = thread::spawn(move || {
    //            let mut num = counter.lock().unwrap();
    //
    //            *num += 1;
    //        });
    //        handles.push(handle);
    //    }
    //
    //    for handle in handles {
    //        handle.join().unwrap();
    //    }
    //
    //    println!("Result: {}", *counter.lock().unwrap());

    //    let counter = Mutex::new(0);
    //    let mut handles = vec![];
    //
    //    for _ in 1..10 {
    //        let th = thread::spawn(move || {
    //            let mut num = counter.lock().unwrap();
    //            *num += 1;
    //        });
    //
    //        handles.push(th);
    //    }
    //
    //    for handle in handles {
    //        handle.join().unwrap();
    //    }
    //    let m = Mutex::new(5);
    //
    //    {
    //        let mut num = m.lock().unwrap();
    //        *num = 99;
    //    }
    //
    //    println!("m is {:?}", *m.lock().unwrap());
    //    let (transmitter, reciever) = mpsc::channel();
    //
    //    let second_transmitter = mpsc::Sender::clone(&transmitter);
    //
    //    thread::spawn(move || {
    //        let a = vec![10, 20, 30, 40, 50];
    //
    //        for b in a {
    //            transmitter.send(b);
    //            thread::sleep(Duration::from_secs(1));
    //        }
    //    });
    //
    //    thread::spawn(move || {
    //        let a = vec![60, 70, 80, 90, 100];
    //
    //        for b in a {
    //            second_transmitter.send(b);
    //            thread::sleep(Duration::from_secs(1));
    //        }
    //    });
    //
    //    for val in reciever {
    //        println!("Received: {}", val);
    //    }

    //    let (transmitter, receiver) = mpsc::channel();
    //
    //    thread::spawn(move || {
    //        let a = vec!["hello", "world", "foo", "bar"];
    //
    //        for val in a {
    //            transmitter.send(val);
    //            thread::sleep(Duration::from_secs(1));
    //        }
    //    });
    //
    //    for i in receiver {
    //        println!("Received: {}", i);
    //    }

    //    let (tranmitter, receiver) = mpsc::channel();
    //
    //    thread::spawn(move || {
    //        let a = String::from("Hello");
    //        tranmitter.send(a).unwrap();
    //        println!("a is {}", a);
    //    });
    //
    //    let b = receiver.recv().unwrap();
    //    println!("Received {}", b);

    //    let a = vec![1, 3, 5];
    //
    //    let th = thread::spawn(move || {
    //        println!("Here is the vector value {:?}", a);
    //    });
    //
    //    //wait for main thread, if not main thread wont bother to wait for sub thread to finish its execution
    //    th.join().unwrap();
    //    println!("{:?}", a);

    //    let thread_handler = thread::spawn(|| {
    //        for i in 1..11 {
    //            println!("Printing {} in sub thread", i);
    //            thread::sleep(Duration::from_millis(1));
    //        }
    //    });
    //
    //    thread_handler.join().unwrap();
    //
    //    for i in 1..6 {
    //        println!("Printing {} in main thread", i);
    //        thread::sleep(Duration::from_millis(1));
    //    }
}
