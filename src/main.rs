
enum Isbn {
    Isbn10(String),
    Error(String),
}

impl Isbn {
    fn from_string(maybe_isbn:String) -> Isbn {
        if maybe_isbn.chars().count() != 10 {
            let err = format!("isbn has to be 10 characters long, you currently got {}", maybe_isbn.chars().count());
            return Isbn::Error(err.to_string());
        }

        return Isbn::Isbn10(maybe_isbn)
    }
}

fn main() {
    let maybe_isbn = std::env::args().nth(1).expect("no pattern given");
    let cleaned_isbn = maybe_isbn.replace("-", "");
    let cleaned_isbn_cp = cleaned_isbn.clone();
    // println!("{}", cleaned_isbn);
    // 123456789-X

    let sono_isbn = Isbn::from_string(cleaned_isbn);
    match sono_isbn {
        Isbn::Isbn10(isbn10) => println!("{}", isbn10),
        Isbn::Error(err) => {panic!("{}", err)},
    };

    if cleaned_isbn_cp.chars().count() != 10 {
        panic!("isbn has to be 10 characters long, you currently got {}", cleaned_isbn_cp.chars().count());
    }

    let maybe_last = cleaned_isbn_cp.chars().last();
    let last = match maybe_last {
        Some(last) => last,
        None => {panic!("this is a terrible mistake!");},
    };

    let last_numeric = if last.is_digit(10) {last.to_digit(10)} else {Some(10)};
    let last_numeric = match last_numeric {
        Some(i) => i,
        None => panic!("last letter \"{}\" is not a digit or \"X\"", last),
    };

    let sliced_isbn = (&cleaned_isbn_cp[..cleaned_isbn_cp.len()-1]).to_string();
    println!("sliced_isbn: {}", sliced_isbn);

    let mut result = 0;
    let mut counter = 10;
    for character in sliced_isbn.chars() {
        result = match character.to_digit(10) {
            Some(i) => result + counter * i,
            None => panic!("letter \"{}\" is not a digit", character),
        };
        counter = counter - 1;
    }

    result = result + last_numeric;

    if let 0 = result % 11 {
       println!("{} is a valid ISBN", "maybe_isbn")
    } else {
        println!("{} is not a valid ISBN", "maybe_isbn")
    }
}
