pub fn dupli_n(list: &[usize], n: usize) -> Vec<usize> {
    list
        .iter()
        .flat_map(|val| {
            vec!(val.clone(); n)
        })
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me() {
        let list: Vec<usize> = vec![1,1,1,2,5,5,5,5,7,8,9,9];
        let duplicaetd = dupli_n(&list, 2);
        println!("The list is: {:?}", list);
        println!("The transformed list is: {:?}", duplicaetd);
        let expected_new: Vec<usize> = vec![1,1,1,1,1,1,2,2,5,5,5,5,5,5,5,5,7,7,8,8,9,9,9,9];
        assert_eq!(duplicaetd, expected_new);
    }

}