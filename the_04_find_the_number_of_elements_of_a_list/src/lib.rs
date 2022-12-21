pub fn find_the_number_of_elements_of_a_list<T: Clone>(list: &Vec<T>) -> usize {
    list.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_vector() {
        assert_eq!(find_the_number_of_elements_of_a_list(&vec![1, 1, 2, 3, 5, 8]), 6);
    }

    #[test]
    fn it_works_for_empty_vector() {
        assert_eq!(find_the_number_of_elements_of_a_list::<u8>(&vec![]), 0);
    }
}
