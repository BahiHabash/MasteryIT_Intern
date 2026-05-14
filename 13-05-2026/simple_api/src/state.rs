use crate::models::{Book, CreateBookRequest};
use std::sync::{Arc, Mutex};

pub struct AppState {
    books: Vec<Book>,
    next_id: u64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            books: vec![Book {
                id: 1,
                title: "The Rust Programming Language".to_string(),
                author: "Steve Klabnik & Carol Nichols".to_string(),
                genre: crate::models::BookGenre::NonFiction {
                    topic: "Programming".to_string(),
                },
                published_year: 2023,
            }],
            next_id: 2,
        }
    }

    pub fn add_book(&mut self, req: CreateBookRequest) -> anyhow::Result<Book> {
        if req.title.trim().is_empty() {
            anyhow::bail!("Book title cannot be empty");
        }
        let book = Book {
            id: self.next_id,
            title: req.title,
            author: req.author,
            genre: req.genre,
            published_year: req.published_year,
        };
        self.next_id += 1;
        self.books.push(book.clone());
        Ok(book)
    }

    pub fn get_all_books(&self) -> Vec<Book> {
        self.books.clone()
    }

    pub fn delete_book(&mut self, id: u64) -> bool {
        let len_before = self.books.len();
        self.books.retain(|b| b.id != id);
        self.books.len() < len_before
    }
}

pub type SharedState = Arc<Mutex<AppState>>;
