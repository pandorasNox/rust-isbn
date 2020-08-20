use isbn::isbn;

fn main() {
    let maybe_isbn = std::env::args().nth(1).expect("no pattern given");
    let cleaned_isbn = maybe_isbn.replace("-", "");
    // println!("{}", cleaned_isbn);
    // 123456789-X

    let isbn = match isbn::Isbn::from_string(cleaned_isbn) {
        Ok(isbn) => isbn,
        Err(err) => {panic!("{}", err)},
    };

    match isbn {
        isbn::Isbn::Isbn10(res) => println!("given \"{}\" is a valid isbn", res)
    }
}
