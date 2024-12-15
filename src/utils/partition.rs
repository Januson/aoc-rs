pub fn partition<T: Clone>(iter: &[T], size: usize, step: usize) -> Vec<Vec<T>> {
    if size == 0 || step == 0 {
        return vec![];
    }

    iter.windows(size)
        .step_by(step)
        .map(|x| x.to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partitions() {
        let data = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(partition(&data, 2, 1), vec![[1, 2], [2, 3], [3, 4], [4, 5], [5, 6]]);
        assert_eq!(partition(&data, 3, 3), vec![[1, 2, 3], [4, 5, 6]]);
    }
}
