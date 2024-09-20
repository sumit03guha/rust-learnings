struct DraftPost {
    content: String,
}

impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

struct Post {
    content: String,
}

impl Post {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    fn content(&self) -> &str {
        &self.content
    }
}

fn main() {
    let mut post = Post::new();

    let text_to_add = "This is a blog.";

    post.add_text(&text_to_add);

    let review_requested_post = post.request_review();

    let approved_post = review_requested_post.approve();

    assert_eq!(approved_post.content(), text_to_add);
}
