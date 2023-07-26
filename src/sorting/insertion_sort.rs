pub fn insertion_sort(numbers: &mut [i32]) {
    if numbers.len() > 1 {
        for i in 1..numbers.len() {
            let mut j = i;
            while j > 0 && numbers[j] < numbers[j - 1] {
                numbers.swap(j - 1, j);
                j = j - 1;
            }

        }
    }
}

#[test]
fn test() {
    let mut n1: Vec<i32> = vec![5, 4, 3, 2, 1];
    insertion_sort(&mut n1);
    assert_eq!(n1, [1, 2, 3, 4, 5]);

    let mut n2: Vec<i32> = vec![0, 3, 2, 9, 5, 7, 6, 1, 4, 8];
    insertion_sort(&mut n2);
    assert_eq!(n2, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut n3: Vec<i32> = vec![-4, 4, -3, 3, -2, 2, -1, 1, 0];
    insertion_sort(&mut n3);
    assert_eq!(n3, [-4, -3, -2, -1, 0, 1, 2, 3, 4]);

    let mut n4: Vec<i32> = vec![100, -99, 34, 33, -11, 75, 0, 245, -333];
    insertion_sort(&mut n4);
    assert_eq!(n4, [-333, -99, -11, 0, 33, 34, 75, 100, 245]);

    let mut n5: Vec<i32> = vec![100, -10, 5, 0, 1, 0, 1, 0, 1, 0, 0, 1];
    insertion_sort(&mut n5);
    assert_eq!(n5, [-10, 0, 0, 0, 0, 0, 1, 1, 1, 1, 5, 100]);
}