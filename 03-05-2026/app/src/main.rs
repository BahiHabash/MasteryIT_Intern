mod book;
use crate::book::Book;
use std::io;
use std::str::FromStr;

type Result<T> = std::result::Result<T, &'static str>;

fn main() {
    let mut library: Vec<Book> = Vec::new();

    println!("--- Welcome to the Rusty Library Manager ---");
    println!("(Demonstrating Chapters 1-5 of the Rust Book)");

    loop {
        print_menu();
        let choice = read_input::<u32>();

        let choice = match choice {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => {
                let _ = add_new_book(&mut library);
            }
            2 => {
                let _ = list_all_books(&library);
            }
            3 => {
                let _ = borrow_or_return_book(&mut library);
            }
            4 => {
                println!("Exiting the Rusty Library Manager. Happy reading!");
                break;
            }
            _ => println!("Invalid choice. Please pick 1, 2, 3, or 4."),
        }
    }
}

fn print_menu(){
    println!("\n--- Menu ---");
    println!("1. Add a new book");
    println!("2. List all books");
    println!("3. Borrow/Return a book");
    println!("4. Exit");
    println!("Enter your choice (1-4):");
}

fn read_input<T>() -> Result<T> where T : FromStr {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().parse::<T>().map_err(|_| "Invalid input")
}

fn add_new_book(library: &mut Vec<Book>) -> Result<()> {
    println!("Enter book title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line");

    println!("Enter book author:");
    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Failed to read line");

    println!("Enter page count:");
    let mut pages_str = String::new();
    io::stdin().read_line(&mut pages_str).expect("Failed to read line");
    let pages: u32 = pages_str.trim().parse().unwrap_or(0);

    let book = Book::new(title.trim().to_string(), author.trim().to_string(), pages);

    library.push(book);
    println!("Book successfully added to the library!");

    Ok(())
}

fn list_all_books(library: &Vec<Book>) -> Result<()> {
    println!("\n--- Library Inventory ---");
    if library.is_empty() {
        println!("The library is currently empty.");
    } else {
        for book in library {
            book.display();
        }
    }

    Ok(())
}

fn borrow_or_return_book(library: &mut Vec<Book>) -> Result<()> {
    println!("Enter the index of the book (0, 1, 2...):");
    let index = read_input::<usize>();

    match index {
        Ok(i) => {
            if let Some(book) = library.get_mut(i) {
                book.toggle_borrow();
                let action = if book.is_borrowed { "borrowed" } else { "returned" };
                println!("You successfully {} the book: {}", action, book.title);
            } else {
                println!("Error: No book found at index {}.", i);
                return Err("Book not found");
            }
        }
        Err(_) => {
            println!("Error: Invalid index.");
            return Err("Invalid index");
        }
    }

    Ok(())
}
