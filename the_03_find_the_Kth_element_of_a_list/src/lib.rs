pub fn find_the_the_kth_element_of_a_list<T: Clone>(list: &Vec<T>, idx: usize) -> Option<&T> {
    list.get(idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_vector() {
        assert_eq!(find_the_the_kth_element_of_a_list(&vec![1, 1, 2, 3, 5, 8], 3), Some(&3));
    }

    #[test]
    fn it_works_for_short_vector() {
        assert_eq!(find_the_the_kth_element_of_a_list(&vec![1], 2), None);
    }

    #[test]
    fn it_works_for_short_vector_2() {
        assert_eq!(find_the_the_kth_element_of_a_list(&vec![1, 2], 0), Some(&1));
    }

    #[test]
    fn it_works_for_empty_vector() {
        assert_eq!(find_the_the_kth_element_of_a_list::<u8>(&vec![], 5), None);
    }
}
