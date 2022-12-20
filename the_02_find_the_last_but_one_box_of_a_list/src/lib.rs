pub fn find_the_last_but_one_box_of_a_list<T: Clone>(list: &Vec<T>) -> Vec<T> {
    if list.len() < 2 {
        return list.to_vec()
    }
    list[list.len() - 2..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_vector() {
        assert_eq!(find_the_last_but_one_box_of_a_list(&vec![1, 1, 2, 3, 5, 8]), vec![5,8]);
    }

    #[test]
    fn it_works_for_short_vector() {
        assert_eq!(find_the_last_but_one_box_of_a_list(&vec![1]), vec![1]);
    }

    #[test]
    fn it_works_for_short_vector_2() {
        assert_eq!(find_the_last_but_one_box_of_a_list(&vec![1, 2]), vec![1, 2]);
    }

    #[test]
    fn it_works_for_empty_vector() {
        assert_eq!(find_the_last_but_one_box_of_a_list::<u8>(&vec![]), vec![]);
    }
}
