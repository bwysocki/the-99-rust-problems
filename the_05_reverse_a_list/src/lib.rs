pub fn reverse_a_list<T: Clone>(list: &Vec<T>) -> Vec<T> {
    let mut cloned = list.clone(); //list.to_vec();
    cloned.reverse();
    cloned
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_vector() {
        let init = vec![1, 1, 2, 3, 5, 8];
        assert_eq!(reverse_a_list(&init), vec![8, 5, 3, 2, 1, 1]);
        assert_eq!(init, vec![1, 1, 2, 3, 5, 8]);
    }

    #[test]
    fn it_works_for_empty_vector() {
        assert_eq!(reverse_a_list::<u8>(&vec![]), vec![]);
    }
}
