//! Quick sort
//!
//! Worst case - O(n^2). Average - O(n*log(n)). `O` constant time could vary, depending on
//! pivot choice, That's the reason we get 2 different `O` values for the algorithm.


// Implemented using recursion
// todo not optimal. Consider this https://github.com/jonhoo/orst/blob/master/src/quicksort.rs to make 17-18 cheaper
pub(super) fn quick_sort(src: Vec<u32>) -> Vec<u32> {
    // base case
    if src.len() < 2 {
        return src;
    }
    // recursion case
    let pivot = src.get(src.len()/2).expect("index out of range");
    let (mut lt_pivot, mut gt_pivot) = {
        let l = src.iter().filter(|&a| a < pivot).map(|a| *a).collect();
        let r = src.iter().filter(|&a| a > pivot).map(|a| *a).collect();
        (quick_sort(l), quick_sort(r))
    };
    lt_pivot.push(*pivot);
    lt_pivot.append(&mut gt_pivot);

    lt_pivot
}

#[test]
fn simple() {
    let src = vec![9, 2, 3, 4, 1, 6, 8, 19, 20, 34];
    assert_eq!(quick_sort(src), vec![1, 2, 3, 4, 6, 8, 9, 19, 20, 34]);
}