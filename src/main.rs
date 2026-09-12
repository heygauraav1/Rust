fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", greet("Rust"));
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn it_greets_the_user() {
        assert_eq!(greet("World"), "Hello, World!");
    }
}
