use blog_in_rust::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // post.content(); // compile error

    let post = post.request_review();

    // post.content(); // compile error

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
