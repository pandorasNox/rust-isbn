fn main() {
    let maybe_isbn = std::env::args().nth(1).expect("no pattern given");
    let cleaned_isbn = maybe_isbn.replace("-", "");
    // println!("{}", cleaned_isbn);
    // 123456789-X

    if cleaned_isbn.chars().count() != 10 {
        panic!("isbn has to be 10 characters long, you currently got {}", cleaned_isbn.chars().count());
    }

    let maybe_last = cleaned_isbn.chars().last();
    let last = match maybe_last {
        Some(last) => last,
        None => {panic!("this is a terrible mistake!");},
    };

    let last_numeric = if last.is_digit(10) {last.to_digit(10)} else {Some(10)};
    let last_numeric = match last_numeric {
        Some(i) => i,
        None => panic!("last letter \"{}\" is not a digit or \"X\"", last),
    };

    let sliced_isbn = (&cleaned_isbn[..cleaned_isbn.len()-1]).to_string();
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
