//! Choice sort
//!
//! Has O(n^2). Very dumb.

pub(super) fn selection_sort<T: Ord>(src: &mut [T]) {
    for i in 0..src.len() - 1 {
        let (position, _) = get_smallest(&src[i..]);
        // we add `i` to `position`, because `position` is an index in a slice of `src`,
        // not src itself. An assert below shows length difference between `src` and a slice of it.
        assert_eq!(&src[i..].len() + i, src.len());
        src.swap(i, i + position);
    }
}

// Could be used for selection sort which either mutates original source
// or which create new collection and puts elements there.
// In the first case you don't need returning `element`.
fn get_smallest<T: Ord>(src: &[T]) -> (usize, &T) {
    let (value, position) = src
        .iter()
        .enumerate()
        .map(|(i, value)| (value, i))
        .min()
        .expect("iterator is not empty");
    (position, value)
}

#[test]
fn simple() {
    let mut src = vec![9, 2, 3, 4, 1, 6, 8, 19, 20, 34];
    selection_sort(&mut src);
    assert_eq!(src, vec![1, 2, 3, 4, 6, 8, 9, 19, 20, 34]);
}
