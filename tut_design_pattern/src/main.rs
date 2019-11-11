use tut_design_pattern::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate salad for lunch today");
    assert_eq!("", post.content());

    post.review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate salad for lunch today", post.content());
}
