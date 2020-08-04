
enum Isbn {
    Isbn10(String),
}

impl Isbn {
    fn from_string(maybe_isbn:String) -> Result<Isbn, String> {
        if maybe_isbn.chars().count() != 10 {
            let err = format!("isbn has to be 10 characters long, you currently got {}", maybe_isbn.chars().count());
            return Err(err.to_string());
        }

        let maybe_last = maybe_isbn.chars().last();
        let last = match maybe_last {
            Some(last) => last,
            None => return Err("should never be reached".to_string()),
        };

        let last_numeric = if last.is_digit(10) {last.to_digit(10)} else {Some(10)};
        let last_numeric = match last_numeric {
            Some(i) => i,
            None => return Err(format!("last letter \"{}\" is not a digit or \"X\"", last).to_string()),
        };

        let sliced_isbn = (&maybe_isbn[..maybe_isbn.len()-1]).to_string();
    
        let mut result = 0;
        let mut counter = 10;
        for character in sliced_isbn.chars() {
            result = match character.to_digit(10) {
                Some(i) => result + counter * i,
                None => return Err(format!("letter \"{}\" is not a digit", character).to_string()),
            };
            counter = counter - 1;
        }

        result = result + last_numeric;

        if result % 11 != 0 {
            return Err(format!("{} is not a valid ISBN", maybe_isbn));
        }

        return Ok(Isbn::Isbn10(maybe_isbn));
    }
}

fn main() {
    let maybe_isbn = std::env::args().nth(1).expect("no pattern given");
    let cleaned_isbn = maybe_isbn.replace("-", "");
    let cleaned_isbn_cp = cleaned_isbn.clone();
    // println!("{}", cleaned_isbn);
    // 123456789-X

    let isbn = match Isbn::from_string(cleaned_isbn) {
        Ok(isbn) => isbn,
        Err(err) => {panic!("{}", err)},
    };

    match isbn {
        Isbn::Isbn10(res) => println!("given \"{}\" is a valid isbn", res)
    }
}
