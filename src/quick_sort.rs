//! Quick sort
//!
//! Worst case - O(n^2). Average - O(n*log(n)). `O` constant time could vary, depending on
//! pivot choice, That's the reason we get 2 different `O` values for the algorithm.
//! Quick sort is implemented here using recursion.

pub(super) fn quick_sort<T: Ord>(src: &mut [T]) {
    // base case
    match src.len() {
        0 | 1 => return,
        2 => {
            if src[0] > src[1] {
                src.swap(0, 1);
            }
        }
        // recursion case
        _ => quick_sort_impl(src),
    }
}

fn quick_sort_impl<T: Ord>(src: &mut [T]) {
    // As it was stated, `O` varies in accordance to a chosen pivot value
    // We saw lot's of implementations using first/last value as a pivot.
    // Let's use the middle one as an example.
    let pivot_index = src.len() / 2;

    // So here is partition. Partition is a core of quick sort. It's aim is to
    // place all the values less than the pivot to the left of it
    // and all the values greater than the pivot to the right of it.
    //
    // So `i` is an index of values less than the pivot, but `j`, otherwise, is an index of values greater than the pivot.
    let mut i = 0;
    let mut j = src.len() - 1;
    while i <= j {
        // Using first or last element as a pivot makes a bit easier rust implementation, because
        // you just go through `let a = src.split_first_mut()` or `let a = src.split_last_mut()` which makes easier
        // 1) handling some test cases, 2) not getting pivot value in every loop (but we have to, because of mutable `swap` op in `else`).
        // if you didn't get it, look at this: https://github.com/jonhoo/orst/blob/da6ba90195f94ec334f382b07498fc4e01795f20/src/quicksort.rs#L17-L42
        let pivot = &src[pivot_index];
        if &src[i] <= pivot {
            // that's fine, current value is in the right place
            // ignore pivot, just go through it
            //
            // Never tries to get value out of `src` bound,
            // because we iterate over src until i <= j, where j = [src.len(), src.len() - 1, ... , 0]
            i += 1;
        } else if &src[j] >= pivot {
            // that's fine, current value is in the right place
            // ignore pivot, just go through it
            if j == 0 {
                // Going through pivot from right to left means,
                // that we could reach the beginning of the `src`.
                break;
            }
            j -= 1;
        } else {
            // If src[i] > pivot and src[j] is less than pivot,
            // it means we have found values with wrong positions.
            // Swap them!
            src.swap(i, j);
            // and go further...
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }

    // That's the other core part of the quick sort.
    // After partition we have an unsorted slice of values,
    // where order of values has a significant attribute:
    // we could place pivot in some place of slice, such that the slice will look like:
    // [unsorted less | pivot | unsorted greater ].
    //
    // So what's the new index for pivot? Another great part of partition is that
    // after it has "sorted" elements, `i` and `j` point to the right position for the pivot.
    // For example, let's use `i` as pointer to the new valid pivot position.
    //
    // There could be 2 different cases for the new valid pivot position.
    //
    // First case is when we ended up processing values after `pivot_index`.
    // As we could see from the `while i <= j`, we finish loop when this `i - 1 == j` (1) will be true.
    // We know that `i` is an index of value less than pivot. It means that at the end of partition we have `i` pointing
    // to the last value of `src`, which is less than pivot, so we can just swap positions between pivot and value under `i`.
    // Seems to be right? But not. Due to (1) condition, after partition `i` points to the value,
    // which is either greater than pivot or out of `src` bounds. So we should swap pivot element with `i-1` element.
    //
    // Second case is when we ended up processing values before `pivot_index`.
    // This differs from the first case in a very crucial way. In first case went go out of the while loop,
    // because `i` got increased to value greater than `j`. Here we go out of the while loop, because `j` got decreased to value less than `i`.
    // In this case the last operation in the loop is the one on line 57. This actually means, that by the end of partition, `j + 1` points to the first
    // element of `src` that is greater than pivot, Due to (1) condition, `j + 1 == i`, so we just swap element under `i` with pivot.
    //
    // There is no third case when we ended up on `i == j == pivot_index`, because
    // 1) we always go through pivot, 2) we end up only when (1) condition is met.

    let b = if i > pivot_index { i - 1 } else { i };
    src.swap(pivot_index, b);

    // recursively sort [unsorted less]
    quick_sort(&mut src[..b]);
    // and [unsorted greater]
    quick_sort(&mut src[b + 1..]);
}

#[test]
fn simple() {
    let mut tests = [
        (
            vec![9, 2, 3, 4, 1, 6, 8, 19, 20, 34],
            vec![1, 2, 3, 4, 6, 8, 9, 19, 20, 34],
        ),
        (
            vec![10, 80, 30, 70, 40, 50, 90],
            vec![10, 30, 40, 50, 70, 80, 90],
        ),
        (vec![2, 3, 4, 5, 10, 1, 11], vec![1, 2, 3, 4, 5, 10, 11]),
        (
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ),
        (vec![1, 5, 3, 4], vec![1, 3, 4, 5]),
        (vec![1, 2, 3, 0, 5], vec![0, 1, 2, 3, 5]),
        (vec![1, 2, 3], vec![1, 2, 3]),
        (vec![3, 1, 2], vec![1, 2, 3]),
        (vec![2, 1, 3], vec![1, 2, 3]),
        (
            vec![6, 1, 7, 9, 3, 8, 2, 5, 4, 0],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        ),
        (vec![3, 2], vec![2, 3]),
        (vec![8, 3, 7, 9, 6, 1, 9, 10], vec![1, 3, 6, 7, 8, 9, 9, 10]),
        (
            vec![8, 2, 78, 892, 11, 0, 34],
            vec![0, 2, 8, 11, 34, 78, 892],
        ),
        (
            vec![9, 03, 83, 9, 2, 0, 1, 65, 2, 822, 9, 11, 22, 3, 3, 3, 47],
            vec![0, 1, 2, 2, 3, 3, 3, 3, 9, 9, 9, 11, 22, 47, 65, 83, 822],
        ),
        (
            vec![-6, 9, 0, 1, 17, 91, 0, 178],
            vec![-6, 0, 0, 1, 9, 17, 91, 178],
        ),
        (
            vec![-3, -2, -1, -9, -5, -1, -19, -33],
            vec![-33, -19, -9, -5, -3, -2, -1, -1],
        ),
        (
            vec![-5, -6, -7, 0, 0, 0, 0, -8, 1, 2, 3],
            vec![-8, -7, -6, -5, 0, 0, 0, 0, 1, 2, 3],
        ),
    ];
    for (input, expected) in tests.iter_mut() {
        let _ = quick_sort(input);
        assert_eq!(input, expected);
    }
}
