use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BookGenre {
    Fiction { subgenre: String },
    NonFiction { topic: String },
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub genre: BookGenre,
    pub published_year: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: u64,
    pub title: String,
    pub author: String,
    pub genre: BookGenre,
    #[serde(rename = "year")]
    pub published_year: u32,
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        Self {
            id: book.id,
            title: book.title,
            author: book.author,
            genre: book.genre,
            published_year: book.published_year,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub genre: BookGenre,
    pub published_year: u32,
}
