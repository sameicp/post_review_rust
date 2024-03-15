use project1::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Start project documentation");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Start project documentation", post.content());
}
