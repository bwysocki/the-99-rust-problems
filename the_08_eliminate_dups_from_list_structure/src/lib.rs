pub fn eliminate(list: &Vec<usize>) -> Vec<usize> {
    list
        .iter()
        .enumerate()
        .map(|(i, val)| {
            if i < list.len() && (list.get(i+1).is_none() || list.get(i+1).unwrap() != val)  {
                Some(val.clone())
            } else {
                None
            }
        })
        .filter(|val: &Option<usize>| {
            val.is_some()
        })
        .map(|val: Option<usize>| {
            val.unwrap()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me() {
        let list: Vec<usize> = vec![1,1,1,2,5,5,5,7,8,9,9];
        let transformed = eliminate(&list);
        println!("The list is: {:?}", list);
        println!("The transformed list is: {:?}", transformed);
        let expected_old: Vec<usize> = vec![1,1,1,2,5,5,5,7,8,9,9];
        let expected_new: Vec<usize> = vec![1,2,5,7,8,9];
        assert_eq!(list, expected_old);
        assert_eq!(transformed, expected_new);
    }

}
