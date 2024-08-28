pub fn add_svm(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_svm(2, 2);
        assert_eq!(result, 4);
    }
}
