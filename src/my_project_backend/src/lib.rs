use std::cell::RefCell;

mod article;
use article::Article;

mod config;
use config::Config;

thread_local! {
    static ARTICLES: RefCell<Vec<Article>> = RefCell::new(Vec::new());
    static CONFIG: RefCell<Config> = RefCell::new(Config::new());
}

#[ic_cdk::query]
fn get_articles() -> Vec<Article> {
    ARTICLES.with(|articles| articles.borrow().clone())
}

#[ic_cdk::update]
fn add_article(
    title: String,
    author: String,
    tags: Vec<String>,
    content: String,
) -> Result<Article, String> {
    let config = CONFIG.with(|config| config.borrow().clone());

    if title.len() == 0 {
        return Err("Title cannot be empty".to_string());
    }

    if title.len() >= config.max_title_length as usize {
        return Err("Title is too long".to_string());
    }

    if author.len() == 0 {
        return Err("Author cannot be empty".to_string());
    }

    if content.len() == 0 {
        return Err("Content cannot be empty".to_string());
    }

    if content.len() >= config.max_content_length as usize {
        return Err("Content is too long".to_string());
    }

    if tags.len() > config.max_tags_count as usize {
        return Err("Too many tags".to_string());
    }

    if tags.iter().any(|tag| !config.tags.contains(tag)) {
        return Err("Invalid tag".to_string());
    }

    let new_article = Article::new(title, author, tags, content);
    ARTICLES.with(|articles| articles.borrow_mut().push(new_article));

    let last_article = ARTICLES.with(|articles| {
        articles
            .borrow()
            .last()
            .expect("Vec should not be empty")
            .clone()
    });
    return Ok(last_article);
}

#[ic_cdk::query]
fn get_config() -> Config {
    CONFIG.with(|config| config.borrow().clone())
}

#[ic_cdk::update]
fn update_config(new_config: Config) {
    CONFIG.with(|config| *config.borrow_mut() = new_config);
}