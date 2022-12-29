pub fn pack(list: &Vec<usize>) -> Vec<Vec<usize>> {
    list
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let mut idx: usize = i;
            let mut counter = 0;

            while list.get(idx).is_some() && list.get(idx).unwrap() == val {
                counter += 1;
                idx += 1;
            }
            let prev = i as i8 - 1;
            if prev < 0 || (list.get(prev as usize).is_some() && list.get(prev as usize).unwrap() != val) {
                Some(vec![val.clone(); counter])
            } else {
                None
            }
        })
        .filter(|val: &Option<Vec<usize>>| {
            val.is_some()
        })
        .map(|val: Option<Vec<usize>>| {
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
        let transformed = pack(&list);
        println!("The list is: {:?}", list);
        println!("The transformed list is: {:?}", transformed);
        let expected_old: Vec<usize> = vec![1,1,1,2,5,5,5,7,8,9,9];
        let expected_new: Vec<Vec<usize>> = vec![vec![1,1,1,],vec![2],vec![5, 5, 5],vec![7],vec![8],vec![9,9]];
        assert_eq!(list, expected_old);
        assert_eq!(transformed, expected_new);
    }

}
