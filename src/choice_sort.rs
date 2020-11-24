//! Choice sort
//!
//! Has O(n^2). Very dumb.

pub(super) fn choice_sort(mut src: Vec<u32>) -> Vec<u32> {
    let mut ret = Vec::with_capacity(src.len());
    while src.len() != 0 {
        let (index, smallest) = get_smallest(&src);
        ret.push(smallest);
        src.remove(index);
    }
    ret
}

fn get_smallest(src: &[u32]) -> (usize, u32) {
    let (index, &element) = src
        .iter()
        .enumerate()
        .min_by(|(_, e1), (_, e2)| e1.cmp(e2))
        .expect("empty iterator");
    (index, element)
}

#[test]
fn simple() {
    let src = vec![9, 2, 3, 4, 1, 6, 8, 19, 20, 34];
    assert_eq!(choice_sort(src), vec![1, 2, 3, 4, 6, 8, 9, 19, 20, 34]);
}
