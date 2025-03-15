/// # Merge Sort
/// ------------
///
/// Merge sort is an efficient sorting algorithm, which works in a divide and
/// conquer method. It first splits the sub-array until single element is left
/// in each split, after that, it starts  merging all one by one.
///
/// Example:
/// We have an array [ 7, 3 10, 2], we start splitting it in the middle until
/// we get single element in an array
///
/// * STEP 1(split):              [ 7, 3] [10, 2]
/// * STEP 2(further split):      [7] [3] [10] [2]
/// we have single element, so we start merging arrays by sorting it
/// At this point, we compare each element and select smallest to put at index 0
/// for this we start comparing each item at index 0 since it is the smallest in
/// each array.
/// after that, we put another element by comparing
///
/// * STEP 3(merge singles):      [3, 7] [2, 10]
/// ... repeat until the whole array is sorted
/// * STEP 4(merge doubles):      [2, 3, 7, 10]
/// ...
///
/// * Time complexity of Merge sort is O(n log(n) )
///
/// *Tradeoffs:
/// - It has more space complexity since it requires storage to store temporary
///   results
/// - It requires copying sub arrays from the original array since we can not
///   directly swap elements in place.
///

fn merge(left: &Vec<isize>, right: &Vec<isize>) -> Vec<isize> {
    // ! both left and right should be sorted to successfully merge as sorted
    let mut merged = vec![];
    let mut left_ptr: usize = 0;
    let mut right_ptr: usize = 0;
    loop {
        let left_exhausted = left_ptr == left.len();
        let right_exhausted = right_ptr == right.len();
        if left_exhausted && right_exhausted {
            break;
        }
        // handle exhausted and non exhausted conditions
        if left_exhausted {
            merged.push(right[right_ptr]);
            right_ptr += 1
        } else if right_exhausted {
            merged.push(left[left_ptr]);
            left_ptr += 1
        } else if left[left_ptr] < right[right_ptr] {
            merged.push(left[left_ptr]);
            left_ptr += 1;
        } else {
            merged.push(right[right_ptr]);
            right_ptr += 1;
        }
    }
    merged
}

fn split(array: &Vec<isize>) -> (Vec<isize>, Vec<isize>) {
    let mid = array.len() / 2;
    let (v1, v2) = array.split_at(mid);
    (Vec::from(v1), Vec::from(v2))
}

fn merge_sort(array: &Vec<isize>) -> Vec<isize> {
    if array.len() <= 1 {
        return array.to_owned();
    }
    // split the array to get smaller slices until we get single items in an array
    let (left, right) = split(&array);
    // merge all splits using recursive merge_sort
    return merge(&merge_sort(&left), &merge_sort(&right));
}

fn main() {
    let unsorted = vec![4, 2, 9, 0, 7, 5, 1];
    let sorted = merge_sort(&unsorted);

    println!("Sorted data: {:?}", sorted);
}

#[cfg(test)]
mod tests {
    use crate::{merge, merge_sort, split};

    #[test]
    fn merge_singles() {
        assert_eq!(merge(&vec![2], &vec![1]), vec![1, 2])
    }

    #[test]
    fn merge_doubles() {
        assert_eq!(merge(&vec![22, 44], &vec![11, 33]), vec![11, 22, 33, 44])
    }

    #[test]
    fn merge_uneven() {
        assert_eq!(
            merge(&vec![22, 44], &vec![11, 33, 55]),
            vec![11, 22, 33, 44, 55]
        )
    }

    #[test]
    fn split_1() {
        assert_eq!(
            split(&vec![11, 22, 33, 44, 55]),
            (vec![11, 22], vec![33, 44, 55])
        )
    }

    #[test]
    fn sort_success() {
        assert_eq!(merge_sort(&vec![1, 2, 5, 4, 3, 6]), vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(merge_sort(&vec![7, 3, 10, 2]), vec![2, 3, 7, 10]);
    }

    #[test]
    fn sort_success_uneven() {
        assert_eq!(merge_sort(&vec![1, 2, 5, 4, 3]), vec![1, 2, 3, 4, 5])
    }
}
