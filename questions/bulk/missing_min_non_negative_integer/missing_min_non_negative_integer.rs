fn missing_min_non_negative_int(mut nums: Vec<i32>) -> u32 {
    for i in 0..nums.len() {
        if nums[i] >= 0 && nums[i] < nums.len() as i32 && (nums[i] as usize) != i {
            let mut j = nums[i];
            nums[i] = -1;
            while j >= 0 && (j as usize) < nums.len() && nums[j as usize] != j {
                swap(&mut nums[j as usize], &mut j);
            }
        }
    }
    for i in 0..nums.len() {
        if nums[i] != i as i32 {
            return i as u32;
        }
    }
    nums.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solution() {
        let cases = vec![
            (vec![-1, 3, 2, 128, 0, 2], 1u32),
            (vec![0, 1, 2, 3], 4u32),
            (vec![1, 2, 3, 4], 0u32),
            (vec![4, 3, 2, 1], 0u32),
            (vec![-1, -1, -1, -1], 0u32),
        ];
        for c in cases {
            let actual = missing_min_non_negative_int(c.0);
            assert_eq!(c.1, actual);
        }
    }
}
