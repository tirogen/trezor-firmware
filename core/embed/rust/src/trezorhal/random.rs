pub fn uniform(n: u32) -> u32 {
    unsafe { super::ffi::random_uniform(n) }
}

/// Shuffles the given slice in place.
pub fn shuffle<T>(slice: &mut [T]) {
    // Fisher-Yates shuffle.
    for i in (1..slice.len()).rev() {
        let j = uniform(i as u32 + 1) as usize;
        slice.swap(i, j);
    }
}

/// Shuffles the given slice in place.
/// Preserves the position of a specified index.
pub fn shuffle_all_but_one<T>(slice: &mut [T], unchanged_index: usize)
where
    T: PartialEq + Copy,
{
    let unchanged_element = slice[unchanged_index];
    // Fisher-Yates shuffle.
    for i in (1..slice.len()).rev() {
        let j = uniform(i as u32 + 1) as usize;
        slice.swap(i, j);
    }
    let new_index = slice.iter().position(|x| *x == unchanged_element).unwrap();
    slice.swap(unchanged_index, new_index);
}

/// Return a random number between `min` and `max` (inclusive).
pub fn uniform_between(min: u32, max: u32) -> u32 {
    assert!(max > min);
    uniform(max - min + 1) + min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_between_test() {
        for _ in 0..10 {
            assert!((10..=11).contains(&uniform_between(10, 11)));
            assert!((10..=12).contains(&uniform_between(10, 12)));
            assert!((256..=512).contains(&uniform_between(256, 512)));
        }
    }
}
