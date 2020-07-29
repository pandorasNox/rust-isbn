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

    if last.is_digit(10) == false && last != 'X' {
        panic!("last letter \"{}\" is not a digit or \"X\"", last);
    }

    let sliced_isbn = (&cleaned_isbn[..cleaned_isbn.len()-1]).to_string();
    println!("sliced_isbn: {}", sliced_isbn);
}
