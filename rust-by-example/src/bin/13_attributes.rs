// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    feature_mod::feature_function();

    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}

#[allow(dead_code)]
fn unused_function() {}


#[cfg(feature = "feature_mod")]
mod feature_mod {
    pub fn feature_function() {
        println!("Hello from feature function!");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        println!("Test!");
    }
}
