pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();

        merge_sort(&mut left);
        merge_sort(&mut right);

        merge(arr, &left, &right);
    }
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    // Merge elements from left and right into arr
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [38, 27, 43, 3, 9, 82, 10];
        merge_sort(&mut arr);
        assert_eq!(arr, [3, 9, 10, 27, 38, 43, 82]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let mut arr: [i32; 0] = [];
        merge_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_merge_sort_single_element() {
        let mut arr = [5];
        merge_sort(&mut arr);
        assert_eq!(arr, [5]);
    }

    #[test]
    fn test_merge_sort_already_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse_sorted() {
        let mut arr = [5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
