/// A function to return "hi"
/// 
/// This function simply return the String "hi".
/// 
/// # Examples
/// 
/// ```
/// use git_learn::hi;
/// assert_eq!(String::from("hi"), hi());
/// ```
pub fn hi() -> String {
    String::from("hi")
}

#[cfg(test)]
mod tests {
    use super::hi;

    #[test]
    fn hi_returns_hi() {
        assert_eq!(String::from("hi"), hi());
    }
}
