pub fn rec_sum(array: &[u32]) -> u32 {
    if array.len() == 1 {
        return array[0]
    }
    array[0] + rec_sum(&array[1..])
}

pub fn length(array: &[u32]) -> u32 {
    if array == [] {
        return 0
    }
    1 + length(&array[1..])
}

pub fn max(array: &[u32]) -> u32 {
    if array.len() == 1 {
        return array[0]
    }

    if array[0] > max(&array[1..]) {
        return array[0]
    }

    max(&array[1..])
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn rec_sum_test() {
        assert_eq!(12, super::rec_sum(&[2,4,6]));
        assert_eq!(0, super::rec_sum(&[0]));
    }

    #[test]
    fn length_test() {
        assert_eq!(3, super::length(&[2,4,6]));
        assert_eq!(1, super::length(&[0]));
    }

    #[test]
    fn max_test() {
        assert_eq!(283213, super::max(&[2, 4, 6, 283213, 12, 23]));
        assert_eq!(0, super::max(&[0]))
    }
} 

