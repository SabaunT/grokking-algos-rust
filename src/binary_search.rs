//! Binary search
//!
//! Essentially, search if effective, because we decrease search area, lowering amount of search steps.
//! Each time we search for an element in 1/2 of the search area. For example, we have 240_000
//! elements in array. After first search step, in case we weren't so luck to find an element on
//! the first step, we through away 120_000 possible variants (new search is 240_000 - 120_000).

// todo AddAssign for nums
// todo type aliases to make it easy to read
/// Important: src should be sorted.
pub(super) fn binary_search(src: &[u32], element: u32) -> Option<usize> {
    if src.is_empty() {
        return None;
    }
    
    // index of the lowest and highest elements of the `src`.
    // such indexing helps us recognizing search area without actually mutating `src`.
    let mut low = 0;
    let mut high = src.len() - 1;
    
    // until search area is at least 1 element
    while low != high { 
        let mid = (low + high / 2) as usize;
        let guess = src[mid];

        if guess > element {
            high = mid - 1;
        }

        if guess < element {
            low = mid + 1;
        }

        if guess == element {
            return Some(mid);
        }
    }
    None
}

#[test]
fn test_bin_search() {
    let src = (0..101).collect::<Vec<_>>();
    assert_eq!(Some(50), binary_search(&src, 50));
}
