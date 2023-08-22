// Plan:
// Create a function that takes a string literal and reverses it.
// sam = mas
// ginger menace = ecanem regnig

pub fn reverse(string: &str) -> &str {
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_takes_an_input() {
        assert_eq!(reverse("a"), "a");
    }

    #[test]
    fn it_changes_the_input() {
        assert_ne!(reverse("abc"), "abc");
    }
}
