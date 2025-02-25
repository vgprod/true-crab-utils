pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary_search(&arr, &5), Some(4));
        assert_eq!(binary_search(&arr, &9), Some(8));
        assert_eq!(binary_search(&arr, &1), Some(0));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(binary_search(&arr, &10), None);
        assert_eq!(binary_search(&arr, &0), None);
    }

    #[test]
    fn test_binary_search_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, &5), None);
    }

    #[test]
    fn test_binary_search_single_element() {
        let arr = [5];
        assert_eq!(binary_search(&arr, &5), Some(0));
        assert_eq!(binary_search(&arr, &10), None);
    }
}
