use candid::CandidType;

#[derive(Clone, CandidType)]
struct Comment {
    author: String,
    content: String,
}

#[derive(Clone, CandidType)]
pub struct Article {
    title: String,
    date: u64,
    author: String,
    tags: Vec<String>,
    content: String,
    comments: Vec<Comment>,
}

impl Article {
    pub fn new(title: String, author: String, tags: Vec<String>, content: String) -> Self {
        Self {
            title,
            date: ic_cdk::api::time(),
            author,
            tags,
            content,
            comments: Vec::new(),
        }
    }

    pub fn add_comment(&mut self, author: String, content: String) {
        self.comments.push(Comment { author, content });
    }
}