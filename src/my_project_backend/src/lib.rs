use std::cell::RefCell;

mod article;
use article::Article;

thread_local! {
    static ARTICLES: RefCell<Vec<Article>> = RefCell::new(Vec::new());
}

#[ic_cdk::query]
fn get_articles() -> Vec<Article> {
    ARTICLES.with(|articles| articles.borrow().clone())
}

#[ic_cdk::update]
fn add_article(title: String, author: String, tags: Vec<String>, content: String) -> Result<String, String> {
    if title.len() == 0 {
        return Err("Title cannot be empty".to_string());
    }

    if title.len() >= 64 {
        return Err("Title is too long".to_string());
    }

    let new_article = Article::new(title, author, tags, content);
    ARTICLES.with(|articles| articles.borrow_mut().push(new_article));
    Ok("OK".to_string())
}
