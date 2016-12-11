use std::iter::once;
use std::char;
use std::ascii::AsciiExt;
use std::collections::HashSet;

fn factorial(x: u8) -> u64 {
    (1..x as u64 + 1).product()
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(15), 1_307_674_368_000);
}

fn is_palindrome(word: &str) -> bool {
    word.chars().eq(word.chars().rev())
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("bob"));
    assert!(is_palindrome("anna"));
    assert!(is_palindrome("lagerregal"));

    assert!(!is_palindrome("peter"));
}

fn greatest_subsequencial_sum(x: &[i64]) -> &[i64] {
    (1..x.len() + 1).flat_map(|y| x.windows(y))
        .chain(once(&x[..0]))
        .max_by_key(|s| s.iter().sum::<i64>())
        .unwrap()
}

#[test]
fn test_greatest_subsequencial_sum() {
    let a = [1, 2, 39, 34, 20, -20, -16, 35, 0];
    assert_eq!(greatest_subsequencial_sum(&a), &a[0..5]);

    let b = [-3, -9, -8, -34];
    assert_eq!(greatest_subsequencial_sum(&b), &[]);
}

fn rot13(secret: &str) -> String {
    secret.chars().map(rotate).collect()
}

fn rotate(old: char) -> char {
    let mut digit = old.to_digit(36).unwrap() + 13;
    if digit >= 36 {
        digit %= 36;
        digit += 10;
    }
    let mut new = char::from_digit(digit, 36).unwrap();
    if old.is_uppercase() {
        new = new.to_ascii_uppercase();
    }
    new

}

#[test]
fn test_rot13() {
    assert_eq!(rot13("hello"), "uryyb");
    assert_eq!(rot13("uryyb"), "hello");

    assert_eq!(
        rot13("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"),
        "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm"
    );

    assert_eq!(rot13("peter"), "crgre");
}

fn rot26(secret: &str) -> String {
    secret.to_string()
}

#[test]
fn test_rot26() {
    assert_eq!(rot26("Karoline"), "Karoline");
}

fn used_chars_count(x: &[&str]) -> u64 {
    x.iter().
        flat_map(|s| s.chars()).
        filter(|c| !c.is_whitespace()).
        collect::<HashSet<char>>().len() as u64
}

#[test]
fn test_used_letters() {
    assert_eq!(used_chars_count(&["hi", "ih gitt"]), 4);
    assert_eq!(used_chars_count(&["peter"]), 4);
    assert_eq!(used_chars_count(&["p e t e r", "barbara"]), 6);
}

fn main() {
    used_chars_count(&["hi", "ih gitt"]);
}
