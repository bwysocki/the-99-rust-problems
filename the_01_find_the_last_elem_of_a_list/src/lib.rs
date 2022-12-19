pub fn the_last_elem_of_a_list<T>(list: &Vec<T>) -> Option<&T> {
    list.last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_not_empty() {
        let my_vec = vec!("a", "b", "c");
        let result = the_last_elem_of_a_list(&my_vec);
        assert_eq!(result, Some(&"c"));
    }

    #[test]
    fn it_works_with_empty() {
        let my_vec: Vec<&str> = vec!();
        let result = the_last_elem_of_a_list(&my_vec);
        assert_eq!(result, None);
    }
}
