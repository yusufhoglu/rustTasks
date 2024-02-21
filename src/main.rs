enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

fn print_publication_info(publication: &Publication) {
    match publication {
        Publication::Book(book) => {
            println!(
                "Kitap: {} yazar: {}, {} sayfa",
                book.title, book.author, book.page_count
            );
        }
        Publication::Magazine(magazine) => {
            println!(
                "Dergi: {} - Sayı: {}, Konu: {}",
                magazine.title, magazine.issue, magazine.topic
            );
        }
    }
}

fn main() {
    let book1 = Book {
        title: String::from("Rust programming by example"),
        author: String::from("Tushar Sharma"),
        page_count: 300,
    };
    let magazine1 = Magazine {
        title: String::from("Altyazı"),
        issue: 12,
        topic: String::from("Sinema"),
    };

    let publications: Vec<Publication> = vec![
        Publication::Book(book1),
        Publication::Magazine(magazine1),
    ];

    for publication in &publications {
        print_publication_info(publication);
    }
}
