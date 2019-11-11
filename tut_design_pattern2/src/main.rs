use tut_design_pattern2::*;


fn main() {
    let mut post: Box<Foo> = Box::new(Post::new());
    post.info();
//    post.add_text("Hello World");
//    post.content();

//    let mut post = post.review();

//    let mut post = post.approve();
//    println!("{}", post.content());
}
