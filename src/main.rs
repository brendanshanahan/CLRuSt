mod sorting;
use crate::sorting::insertion_sort::*;

fn main() {
    let mut numbers: Vec<i32> = vec![5, 4, 3, 2, 1];
    println!("unsorted: {:?}", numbers);
    insertion_sort(&mut numbers);
    println!("sorted:   {:?}", numbers);
}
