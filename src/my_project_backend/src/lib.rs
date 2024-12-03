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
fn add_article(title: String, date: u32, author: String, tags: Vec<String>, content: String) {
    let new_article = Article::new(title, date, author, tags, content);
    ARTICLES.with(|articles| articles.borrow_mut().push(new_article));
}
