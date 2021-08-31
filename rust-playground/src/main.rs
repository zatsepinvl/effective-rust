use restaurant::back_of_house::cook;
use restaurant::serve;

mod restaurant;

fn main() {
    let s1 = String::from("hello");
    {
        let s2 = &s1;
        println!("{}", first_word(s2));
    }
    println!("{}", s1);
}

fn length(s: &String) -> usize {
    return s.len();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
