pub fn binary_search(array: &[i32], item: &i32) -> Option<usize> {
    let mut smaller = 0;
    let mut bigger = array.len() - 1;

    while smaller <= bigger {
        let mut middle = (smaller + bigger) / 2;
        let element = array[middle];

        if element == *item {
            return Some(middle);
        }
        if element> *item {
            bigger = middle - 1;
        }
        if element < *item {
            smaller = middle + 1;
        }
    }
    None
}