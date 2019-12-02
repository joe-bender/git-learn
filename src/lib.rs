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
