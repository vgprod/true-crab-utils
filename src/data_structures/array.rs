//! Array operations: traversal, reversal, insertion, deletion, and merging.

/// Traverse array and apply a closure to each element.
///
/// **When to use**: Process every element (e.g., sum, print).
///
pub fn traverse<T, F>(arr: &[T], mut process: F)
where
    F: FnMut(&T),
{
    for item in arr {
        process(item);
    }
}
/// # Example
/// ```
/// let arr = [2, 4, 10];
/// let mut sum = 0;
/// traverse(&arr, |x| sum += x);
/// assert_eq!(sum, 16);
/// ```
///
/// Reverse array elements in-place.
///
/// **When to use**: Reverse element order (e.g., palindromes).
///
pub fn reverse<T>(arr: &mut [T]) {
    let mut start = 0;
    let mut end = arr.len().saturating_sub(1);

    while start < end {
        arr.swap(start, end);
        start += 1;
        end -= 1;
    }
}
/// # Example
/// ```
/// let mut arr = vec![2, 4, 10];
/// reverse(&mut arr);
/// assert_eq!(arr, vec![10, 4, 2]);
/// ```
///
/// Insert element at specified position.
///
/// **When to use**: Add an element at a specific index.
///
pub fn insert<T: Clone>(arr: &mut Vec<T>, element: T, position: usize) {
    arr.insert(position, element);
}
/// # Example
/// ```
/// let mut arr = vec![2, 4, 10];
/// insert(&mut arr, 100, 1);
/// assert_eq!(arr, vec![2, 100, 4, 10]);
/// ```
///
/// Delete element at specified position.
///
/// **When to use**: Remove an element at a specific index.
///
pub fn delete<T>(arr: &mut Vec<T>, position: usize) {
    arr.remove(position);
}
/// # Example
/// ```
/// let mut arr = vec![2, 4, 10];
/// delete(&mut arr, 1);
/// assert_eq!(arr, vec![2, 10]);
/// ```
/// Merge two arrays and sort the result.
///
/// **When to use**: Combine and sort two arrays.
///
pub fn merge_and_sort<T: Ord + Clone>(arr1: &[T], arr2: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    merged.extend_from_slice(arr1);
    merged.extend_from_slice(arr2);
    merged.sort();
    merged
}
/// # Example
/// ```
/// let arr1 = [2, 4];
/// let arr2 = [3, 1];
/// assert_eq!(merge_and_sort(&arr1, &arr2), vec![1, 2, 3, 4]);
/// ```
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
        let mut arr = vec![2, 4, 10];
        reverse(&mut arr);
        assert_eq!(arr, vec![10, 4, 2]);
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
