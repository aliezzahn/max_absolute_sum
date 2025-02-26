pub struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut max_ending_here = nums[0];
        let mut min_ending_here = nums[0];
        let mut max_so_far = nums[0];
        let mut min_so_far = nums[0];
        
        for i in 1..nums.len() {
            max_ending_here = (max_ending_here + nums[i]).max(nums[i]);
            min_ending_here = (min_ending_here + nums[i]).min(nums[i]);
            
            max_so_far = max_so_far.max(max_ending_here);
            min_so_far = min_so_far.min(min_ending_here);
        }
        
        // Handle absolute value of min_so_far to avoid overflow
        let min_abs = if min_so_far == i32::MIN {
            i32::MAX
        } else {
            (min_so_far as i64).abs() as i32
        };
        
        max_so_far.max(min_abs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let nums = vec![1, -3, 2, 3, -4];
        assert_eq!(Solution::max_absolute_sum(nums), 5);
    }

    #[test]
    fn test_all_positive() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_absolute_sum(nums), 15);
    }

    #[test]
    fn test_all_negative() {
        let nums = vec![-1, -2, -3, -4, -5];
        assert_eq!(Solution::max_absolute_sum(nums), 15);
    }

    #[test]
    fn test_single_element_positive() {
        let nums = vec![5];
        assert_eq!(Solution::max_absolute_sum(nums), 5);
    }

    #[test]
    fn test_single_element_negative() {
        let nums = vec![-5];
        assert_eq!(Solution::max_absolute_sum(nums), 5);
    }

    #[test]
    fn test_all_zeros() {
        let nums = vec![0, 0, 0];
        assert_eq!(Solution::max_absolute_sum(nums), 0);
    }

    #[test]
    fn test_mixed_with_zeros() {
        let nums = vec![2, -1, 0, 3, -2];
        assert_eq!(Solution::max_absolute_sum(nums), 4);
    }

    #[test]
    fn test_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(Solution::max_absolute_sum(nums), 0);
    }

    #[test]
    fn test_large_numbers() {
        let nums = vec![i32::MAX, i32::MIN, 100];
        assert_eq!(Solution::max_absolute_sum(nums), i32::MAX);
    }
}