pub fn is_palindrome<usize: PartialEq>(list: &Vec<usize>) -> bool {
    let len = list.len();
    if len < 1 {
        return  true;
    }
    let middle = list.len() / 2;
    for i in 0..middle {
        let left = &list[i];
        let right = &list[len - 1 - i];
        if (left != right) {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_elements() {
        let li = vec![1, 2, 3, 2, 1];
        assert_eq!(is_palindrome(&li), true);
    }

    #[test]
    fn test_even_elements() {
        let li = vec![1, 2, 2, 1];
        assert_eq!(is_palindrome(&li), true);
    }

    #[test]
    fn test_odd_elements_non() {
        let li = vec![1, 3, 3, 2, 1];
        assert_eq!(is_palindrome(&li), false);
    }

    #[test]
    fn test_even_elements_non() {
        let li = vec![1, 3, 2, 1];
        assert_eq!(is_palindrome(&li), false);
    }

    #[test]
    fn test_empty_list() {
        let li = Vec::<usize>::new();
        assert_eq!(is_palindrome(&li), true);
    }

    #[test]
    fn test_one_elemens() {
        let li = vec![1];
        assert_eq!(is_palindrome(&li), true);
    }
}
