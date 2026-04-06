#![allow(unused)]
mod greeting;
mod math;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math() {
        assert_eq!(math::add(10, 20), 30);
        assert_eq!(math::multiply(3, 4), 12);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greeting::hello("smpl"), String::from("Hello, smpl!"));
    }
}
