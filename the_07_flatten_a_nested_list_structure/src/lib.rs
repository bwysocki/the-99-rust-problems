use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
struct List<T: Clone> {
    vec: Vec<Item<T>>
}
impl <T: Clone + Debug> List<T> {
    pub fn new(vec: Vec<Item<T>>) -> List<T>{
        List { vec }
    }
    pub fn flattern(&self) -> List<T> {
        let transformed = self.vec
            .iter()
            .flat_map(|val|{
                vec!(val);
                match val {
                    Item::ELEM(elem) => vec![elem.clone()],
                    Item::VEC(v) => v.clone()
                }
            })
            .map(|val| {
                Item::ELEM(val)
            })
            //.cloned()
            .collect::<Vec<_>>();
        List::new(transformed)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Item<T> {
    ELEM(T),
    VEC(Vec<T>)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me() {
        let list: List<usize> = List::new(vec![Item::ELEM(2), Item::ELEM(5), Item::VEC(vec![1,2,3]), Item::ELEM(12)]);
        let transformed = list.flattern();
        println!("The list is: {:?}", list);
        println!("The transformed list is: {:?}", transformed);
        let expected_old: List<usize> = List::new(vec![Item::ELEM(2), Item::ELEM(5), Item::VEC(vec![1,2,3]), Item::ELEM(12)]);
        let expected_new: List<usize> = List::new(vec![Item::ELEM(2), Item::ELEM(5), Item::ELEM(1), Item::ELEM(2), Item::ELEM(3),Item::ELEM(12)]);
        assert_eq!(list, expected_old);
        assert_eq!(transformed, expected_new);
    }

}
