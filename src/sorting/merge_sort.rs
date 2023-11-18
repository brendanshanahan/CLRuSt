pub fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut merged: Vec<i32> = vec![0; left.len() + right.len()];
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged[i + j] = left[i];
            i = i + 1;
        } else {
            merged[i + j] = right[j];
            j = j + 1;
        }
    }
    while i < left.len() {
        merged[i + j] = left[i];
        i = i + 1;
    }
    while j < right.len() {
        merged[i + j] = right[j];
        j = j + 1;
    }
    merged
}

pub fn merge_sort(numbers: Vec<i32>) -> Vec<i32> {
    if numbers.len() <= 1 {
        return numbers;
    } else {
        let n = numbers.len() / 2;
        let left: Vec<i32> = merge_sort(numbers[0..n].to_vec());
        let right: Vec<i32> = merge_sort(numbers[n..numbers.len()].to_vec());
        let merged = merge(left, right);
        return merged
    }
}

#[test]
fn test_merge() {
    let left: Vec<i32> = vec![0, 1, 2];
    let right: Vec<i32> = vec![3, 4, 5];
    let merged = merge(left, right);
    assert_eq!(merged, [0, 1, 2, 3, 4, 5]);

    let left: Vec<i32> = vec![0, 0, 0, 1, 2];
    let right: Vec<i32> = vec![1, 1, 1];
    let merged = merge(left, right);
    assert_eq!(merged, [0, 0, 0, 1, 1, 1, 1, 2]);
}

#[test]
fn test_merge_sort() {
    let numbers: Vec<i32> = vec![5, 4, 3, 2, 1];
    let sorted = merge_sort(numbers);
    assert_eq!(sorted, [1, 2, 3, 4, 5]);

    let numbers: Vec<i32> = vec![5, 1, 4, 9, 3, 3, 7, 0, 2, 0, 10, 1];
    let sorted = merge_sort(numbers);
    assert_eq!(sorted, [0, 0, 1, 1, 2, 3, 3, 4, 5, 7, 9, 10]);
}