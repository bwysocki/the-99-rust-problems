#[derive(Debug, PartialEq)]
pub enum Entry {
    Single(usize),
    Vector(Vec<usize>)
}

pub fn encode(list: &Vec<usize>) -> Vec<Entry> {
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
        .map(|val: Vec<usize>| {
            if val.len() > 1 {
                return Entry::Vector(vec![val.len(), *val.get(0).unwrap()])
            }
            return Entry::Single(*val.get(0).unwrap())
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me() {
        let list: Vec<usize> = vec![1,1,1,2,5,5,5,5,7,8,9,9];
        let transformed = encode(&list);
        println!("The list is: {:?}", list);
        println!("The transformed list is: {:?}", transformed);
        let expected_old: Vec<usize> = vec![1,1,1,2,5,5,5,5,7,8,9,9];
        let expected_new: Vec<Entry> = vec![
            Entry::Vector(vec![3,1,]),
            Entry::Single(2),
            Entry::Vector(vec![4, 5]),
            Entry::Single(7),
            Entry::Single(8),
            Entry::Vector(vec![2,9])];
        assert_eq!(list, expected_old);
        assert_eq!(transformed, expected_new);
    }

}
