//! Array operations: traversal, reversal, insertion, deletion, and merging.

pub fn traverse<T, F>(arr: &[T], mut process: F)
where
    F: FnMut(&T),
{
    for item in arr {
        process(item);
    }
}

pub fn reverse<T>(arr: &mut [T]) {
    let mut start = 0;
    let mut end = arr.len().saturating_sub(1);

    while start < end {
        arr.swap(start, end);
        start += 1;
        end -= 1;
    }
}

pub fn insert<T: Clone>(arr: &mut Vec<T>, element: T, position: usize) {
    arr.insert(position, element);
}

pub fn delete<T>(arr: &mut Vec<T>, position: usize) {
    arr.remove(position);
}

pub fn merge_and_sort<T: Ord + Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    merged.extend_from_slice(arr1);
    merged.extend_from_slice(arr2);
    merged.sort();
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse() {
        let arr = [2, 4, 10];
        let mut sum = 0;
        traverse(&arr, |x| sum += x);
        assert_eq!(sum, 16);
    }

    #[test]
    fn test_reverse() {
        let mut arr = vec!["a", "b", "c"];
        reverse(&mut arr);
        assert_eq!(arr, vec!["c", "b", "a"]);
    }

    #[test]
    fn test_insert_delete() {
        let mut arr = vec![2, 4, 10];
        insert(&mut arr, 100, 1);
        assert_eq!(arr, vec![2, 100, 4, 10]);

        delete(&mut arr, 2);
        assert_eq!(arr, vec![2, 100, 10]);
    }

    #[test]
    fn test_merge() {
        let arr1 = [2, 4];
        let arr2 = [3, 1];
        assert_eq!(merge_and_sort(&arr1, &arr2), vec![1, 2, 3, 4]);
    }
}
