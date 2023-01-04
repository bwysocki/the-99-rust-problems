#[derive(Debug, PartialEq, Clone)]
pub enum Entry {
    Single(usize),
    Vector(Vec<usize>)
}

pub fn encode(list: &Vec<Entry>) -> Vec<usize> {
    list
        .iter()
        .map(|val| {
            match val {
                Entry::Single(v) => vec![v.clone()],
                Entry::Vector(v) => vec![v[1].clone(); v[0].clone()]
            }
        })
        .flat_map(|val: Vec<usize>| {
            val
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me() {
        let list: Vec<Entry> = vec![
            Entry::Vector(vec![3,1,]),
            Entry::Single(2),
            Entry::Vector(vec![4, 5]),
            Entry::Single(7),
            Entry::Single(8),
            Entry::Vector(vec![2,9])];

        let transformed = encode(&list);
        println!("The list is: {:?}", list);
        println!("The transformed list is: {:?}", transformed);
        let expected_new: Vec<usize> = vec![1,1,1,2,5,5,5,5,7,8,9,9];

        assert_eq!(expected_new, transformed);
    }

}
