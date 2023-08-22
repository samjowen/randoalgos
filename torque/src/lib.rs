// Plan:
// Create a function that takes a string literal and reverses it.
// sam = mas
// ginger menace = ecanem regnig

pub fn reverse(string: String) -> String {
    string.chars().rev().collect()
}

pub fn dog_reverse(string: String) -> String {
    let string_length = string.chars().count();
    let mut bytes =  String::into_bytes(string);
    let mut rev_byte_vec = vec![string_length];
    let mut i = 0;
    while i < string_length {
        rev_byte_vec.push(bytes.first());
        i = i + 1
    }
    String::from_utf8(rev_byte_vec)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_takes_an_input() {
        // Built in:
        assert_eq!(reverse(String::from("a")), "a");
        // Dogfooding:
        assert_eq!(dog_reverse(String::from("a")), "a")
    }

    #[test]
    fn it_changes_the_input() {
        assert_ne!(reverse(String::from("abc")), "abc");
        assert_ne!(dog_reverse(String::from("abc")), "abc");
    }

    #[test]
    fn it_reverses_the_string() {
        assert_eq!(reverse(String::from("garfield")), "dleifrag");
        assert_eq!(reverse(String::from("sam")), "mas");
    }

    #[test]
    fn it_checks_the_length_of_the_string() {
        assert_eq!((String::from("12345")).len(), 5);
    }
}
