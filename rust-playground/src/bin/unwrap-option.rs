fn main()  {
    let res = try_to_parse().unwrap();
    println!("{:?}", res);
}

fn try_to_parse() -> Option<i32> {
    //let x: i32 = "123".parse(); // x = 123
    let res = "24a".parse();
    if let Ok(x) = res {
        Some(x)
    } else {
        None
    }
}

