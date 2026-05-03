pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
    pub is_borrowed: bool,
}

impl Book {
    pub fn new(title: String, author: String, pages: u32) -> Book {
        Book {
            title,
            author,
            pages,
            is_borrowed: false,
        }
    }

    pub fn display(&self) {
        let status = if self.is_borrowed { "Borrowed" } else { "Available" };
        println!(
            "\"{}\" by {} ({} pages) - Status: {}",
            self.title,
            self.author,
            self.pages,
            status
        );
    }

    pub fn toggle_borrow(&mut self) {
        self.is_borrowed = !self.is_borrowed;
    }
}