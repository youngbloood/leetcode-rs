use core::num;

struct Solution;

impl Solution {
    /**
    https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3238/

    detail:
    Max Consecutive Ones

    Solution
    Given a binary array nums, return the maximum number of consecutive 1's in the array.

    Example 1:
    Input: nums = [1,1,0,1,1,1]
    Output: 3
    Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.

    Example 2:
    Input: nums = [1,0,1,1,0,1]
    Output: 2

    Constraints:

    1 <= nums.length <= 10^5
    nums[i] is either 0 or 1.
    */
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = 0i32;
        let mut temp = 0i32;
        for val in nums {
            if val == 1 {
                temp += 1;
            } else {
                if temp > max {
                    max = temp;
                }
                temp = 0;
            }
        }

        if temp > max {
            max = temp;
        }

        return max;
    }

    /**
    https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3237/


    Find Numbers with Even Number of Digits
    Given an array nums of integers, return how many of them contain an even number of digits.


    Example 1:
    Input: nums = [12,345,2,6,7896]
    Output: 2
    Explanation:
    12 contains 2 digits (even number of digits).
    345 contains 3 digits (odd number of digits).
    2 contains 1 digit (odd number of digits).
    6 contains 1 digit (odd number of digits).
    7896 contains 4 digits (even number of digits).
    Therefore only 12 and 7896 contain an even number of digits.

    Example 2:
    Input: nums = [555,901,482,1771]
    Output: 1
    Explanation:
    Only 1771 contains an even number of digits.

    Constraints:

    1 <= nums.length <= 500
    1 <= nums[i] <= 10^5
    */
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        fn helper(num: i32) -> bool {
            let mut left = num;
            let mut num_digits = 0;
            while left > 0 {
                left /= 10;
                num_digits += 1;
            }
            return num_digits % 2 == 0;
        }

        let mut even_nums = 0i32;
        for num in nums {
            if helper(num) {
                even_nums += 1;
            }
        }
        return even_nums;
    }

    /**
    https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3240/


    Squares of a Sorted Array

    Solution
    Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.



    Example 1:
    Input: nums = [-4,-1,0,3,10]
    Output: [0,1,9,16,100]
    Explanation: After squaring, the array becomes [16,1,0,9,100].
    After sorting, it becomes [0,1,9,16,100].

    Example 2:
    Input: nums = [-7,-3,2,3,11]
    Output: [4,9,9,49,121]


    Constraints:

    1 <= nums.length <= 10^4
    -10^4 <= nums[i] <= 10^4
    nums is sorted in non-decreasing order.


    Follow up: Squaring each element and sorting the new array is very trivial, could you find an O(n) solution using a different approach?
    */
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = nums.iter().map(|i| i * i).collect();
        res.sort();
        return res;
    }

    /**
    https://leetcode.com/explore/learn/card/fun-with-arrays/525/inserting-items-into-an-array/3245/

    Duplicate Zeros

    Solution
    Given a fixed-length integer array arr, duplicate each occurrence of zero, shifting the remaining elements to the right.

    Note that elements beyond the length of the original array are not written. Do the above modifications to the input array in place and do not return anything.



    Example 1:
    Input: arr = [1,0,2,3,0,4,5,0]
    Output: [1,0,0,2,3,0,0,4]
    Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]

    Example 2:
    Input: arr = [1,2,3]
    Output: [1,2,3]
    Explanation: After calling your function, the input array is modified to: [1,2,3]


    Constraints:

    1 <= arr.length <= 10^4
    0 <= arr[i] <= 9
    */
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut idx = 0;
        let total_len = arr.len();

        while idx < total_len {
            if arr[idx] == 0 && (idx + 1) < total_len {
                arr.insert(idx + 1, 0);
                arr.remove(arr.len() - 1);
                idx += 2;
                continue;
            }
            idx += 1;
        }
    }

    /**
    https://leetcode.com/explore/learn/card/fun-with-arrays/525/inserting-items-into-an-array/3253/

    Merge Sorted Array

    Solution
    You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

    Merge nums1 and nums2 into a single array sorted in non-decreasing order.

    The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.



    Example 1:

    Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
    Output: [1,2,2,3,5,6]
    Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
    The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
    Example 2:

    Input: nums1 = [1], m = 1, nums2 = [], n = 0
    Output: [1]
    Explanation: The arrays we are merging are [1] and [].
    The result of the merge is [1].
    Example 3:

    Input: nums1 = [0], m = 0, nums2 = [1], n = 1
    Output: [1]
    Explanation: The arrays we are merging are [] and [1].
    The result of the merge is [1].
    Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.

    Constraints:

    nums1.length == m + n
    nums2.length == n
    0 <= m, n <= 200
    1 <= m + n <= 200
    -10^9 <= nums1[i], nums2[j] <= 10^9


    Follow up: Can you come up with an algorithm that runs in O(m + n) time?
        */

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut idx = 0;
        while idx < n {
            nums1[(m + idx) as usize] = nums2[idx as usize];
            idx += 1;
        }
        nums1.sort();
        // if m == 0 {
        //     let mut idx = 0usize;
        //     nums1.remove(nums1.len() - 1);
        //     while idx < n as usize {
        //         nums1.insert(nums1.len(), nums2[idx]);
        //         idx += 1;
        //     }
        //     return;
        // }

        // let mut idx1 = 0usize;
        // let mut idx2 = 0usize;

        // while idx2 < n as usize {
        //     if nums1[idx1] > nums2[idx2] {
        //         nums1.insert(idx1, nums2[idx2]);
        //         nums1.remove(nums1.len() - 1);
        //         idx2 += 1;
        //     } else {
        //         if idx1 > m as usize {
        //             nums1.insert(idx1 + 1, nums2[idx2]);
        //             nums1.remove(nums1.len() - 1);
        //             idx2 += 1;
        //         }
        //     }
        //     idx1 += 1;
        // }
    }

    /**
    https://leetcode.com/explore/learn/card/fun-with-arrays/526/deleting-items-from-an-array/3247/

    Remove Element

    Solution
    Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The relative order of the elements may be changed.

    Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

    Return k after placing the final result in the first k slots of nums.

    Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

    Custom Judge:

    The judge will test your solution with the following code:

    int[] nums = [...]; // Input array
    int val = ...; // Value to remove
    int[] expectedNums = [...]; // The expected answer with correct length.
                                // It is sorted with no values equaling val.

    int k = removeElement(nums, val); // Calls your implementation

    assert k == expectedNums.length;
    sort(nums, 0, k); // Sort the first k elements of nums
    for (int i = 0; i < actualLength; i++) {
        assert nums[i] == expectedNums[i];
    }
    If all assertions pass, then your solution will be accepted.



    Example 1:
    Input: nums = [3,2,2,3], val = 3
    Output: 2, nums = [2,2,_,_]
    Explanation: Your function should return k = 2, with the first two elements of nums being 2.
    It does not matter what you leave beyond the returned k (hence they are underscores).

    Example 2:
    Input: nums = [0,1,2,2,3,0,4,2], val = 2
    Output: 5, nums = [0,1,4,0,3,_,_,_]
    Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
    Note that the five elements can be returned in any order.
    It does not matter what you leave beyond the returned k (hence they are underscores).


    Constraints:
    0 <= nums.length <= 100
    0 <= nums[i] <= 50
    0 <= val <= 100
        */
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0usize;
        while idx < nums.len() {
            if nums[idx] == val {
                nums.remove(idx);
                continue;
            }
            idx += 1;
        }
        return nums.len() as i32;
    }

    /**
    https://leetcode.com/explore/learn/card/fun-with-arrays/526/deleting-items-from-an-array/3248/

    Remove Duplicates from Sorted Array

    Solution
    Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same.

    Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

    Return k after placing the final result in the first k slots of nums.

    Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

    Custom Judge:

    The judge will test your solution with the following code:

    int[] nums = [...]; // Input array
    int[] expectedNums = [...]; // The expected answer with correct length

    int k = removeDuplicates(nums); // Calls your implementation

    assert k == expectedNums.length;
    for (int i = 0; i < k; i++) {
        assert nums[i] == expectedNums[i];
    }
    If all assertions pass, then your solution will be accepted.



    Example 1:
    Input: nums = [1,1,2]
    Output: 2, nums = [1,2,_]
    Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
    It does not matter what you leave beyond the returned k (hence they are underscores).

    Example 2:
    Input: nums = [0,0,1,1,1,2,2,3,3,4]
    Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
    Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
    It does not matter what you leave beyond the returned k (hence they are underscores).


    Constraints:

    0 <= nums.length <= 3 * 10^4
    -100 <= nums[i] <= 100
    nums is sorted in non-decreasing order.

        */

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        return nums.len() as i32;
    }

    /**
    https://leetcode.com/explore/learn/card/fun-with-arrays/527/searching-for-items-in-an-array/3250/

    Check If N and Its Double Exist

    Given an array arr of integers, check if there exists two integers N and M such that N is the double of M ( i.e. N = 2 * M).

    More formally check if there exists two indices i and j such that :

    i != j
    0 <= i, j < arr.length
    arr[i] == 2 * arr[j]


    Example 1:

    Input: arr = [10,2,5,3]
    Output: true
    Explanation: N = 10 is the double of M = 5,that is, 10 = 2 * 5.
    Example 2:

    Input: arr = [7,1,14,11]
    Output: true
    Explanation: N = 14 is the double of M = 7,that is, 14 = 2 * 7.
    Example 3:

    Input: arr = [3,1,7,11]
    Output: false
    Explanation: In this case does not exist N and M, such that N = 2 * M.


    Constraints:

    2 <= arr.length <= 500
    -10^3 <= arr[i] <= 10^3

    Hide Hint #1
    Loop from i = 0 to arr.length, maintaining in a hashTable the array elements from [0, i - 1].

    Hide Hint #2
    On each step of the loop check if we have seen the element 2 * arr[i] so far or arr[i] / 2 was seen if arr[i] % 2 == 0.
        */
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut idx = 0usize;
        while idx < arr.len() - 1 {
            let mut idx_j = 0usize;
            while idx_j < arr.len() {
                if arr[idx] == arr[idx_j] * 2 {
                    return true;
                }
                idx_j += 1;
                if idx_j == idx {
                    idx_j += 1;
                }
            }
            idx += 1;
        }
        return false;
    }

    /**
    https://leetcode.com/explore/learn/card/fun-with-arrays/527/searching-for-items-in-an-array/3251/

    Valid Mountain Array

    Solution
    Given an array of integers arr, return true if and only if it is a valid mountain array.

    Recall that arr is a mountain array if and only if:

    arr.length >= 3
    There exists some i with 0 < i < arr.length - 1 such that:
    arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
    arr[i] > arr[i + 1] > ... > arr[arr.length - 1]



    Example 1:
    Input: arr = [2,1]
    Output: false

    Example 2:
    Input: arr = [3,5,5]
    Output: false

    Example 3:
    Input: arr = [0,3,2,1]
    Output: true


    Constraints:

    1 <= arr.length <= 10^4
    0 <= arr[i] <= 10^4
        */
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut idx = 1usize;
        let mut up = false;
        let mut down = false;

        while idx < arr.len() {
            if arr[idx] == arr[idx - 1] {
                return false;
            }
            if arr[idx] > arr[idx - 1] {
                up = true;
                if down {
                    return false;
                }
            } else {
                down = true;
                if !up {
                    return false;
                }
            }
            idx += 1;
        }
        return up && down;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    enum AssertType {
        Eq,
        Ne,
        Nil,
    }

    struct Judge<T> {
        at: AssertType,
        src_val: T,
    }

    impl<T: Debug + Eq> Judge<T> {
        pub fn new(at: AssertType, src_val: T) -> Self {
            return Judge { at, src_val };
        }

        pub fn assert(&self, target_val: T) {
            match self.at {
                AssertType::Eq => {
                    assert_eq!(self.src_val, target_val);
                }
                AssertType::Ne => todo!(),
                AssertType::Nil => todo!(),
            }
        }
    }

    #[test]
    fn test_find_max_consecutive_ones() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        let val = Solution::find_max_consecutive_ones(nums);
        println!("test_find_max_consecutive_ones.val = {}", val);

        let nums = vec![1, 1, 0, 1, 1];
        let val = Solution::find_max_consecutive_ones(nums);
        println!("test_find_max_consecutive_ones.val = {}", val);
    }

    #[test]
    fn test_find_numbers() {
        let nums = vec![12, 345, 2, 6, 7896];
        let val = Solution::find_numbers(nums);
        println!("test_find_numbers.val = {}", val);

        let nums = vec![555, 901, 482, 1771];
        let val = Solution::find_numbers(nums);
        println!("test_find_numbers.val = {}", val);
    }

    #[test]
    fn test_finsorted_squares() {
        let nums = vec![-4, -1, 0, 3, 10];
        let val = Solution::sorted_squares(nums);
        println!("test_finsorted_squares.val = {:?}", val);

        let nums = vec![-7, -3, 2, 3, 11];
        let val = Solution::sorted_squares(nums);
        println!("test_finsorted_squares.val = {:?}", val);
    }

    #[test]
    fn test_duplicate_zeros() {
        let mut nums = vec![1, 0, 2, 3, 0, 4, 5, 0];
        Solution::duplicate_zeros(&mut nums);
        println!("test_duplicate_zeros.val = {:?}", nums);

        let mut nums = vec![1, 2, 3];
        Solution::duplicate_zeros(&mut nums);
        println!("test_duplicate_zeros.val = {:?}", nums);
    }

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        println!("test_merge.val = {:?}", nums1);

        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        println!("test_merge.val = {:?}", nums1);

        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        println!("test_merge.val = {:?}", nums1);

        let mut nums1 = vec![1, 0];
        let mut nums2 = vec![2];
        Solution::merge(&mut nums1, 1, &mut nums2, 1);
        println!("test_merge.val = {:?}", nums1);
    }

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        Solution::remove_element(&mut nums, 3);
        println!("test_remove_element.val = {:?}", nums);

        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        Solution::remove_element(&mut nums, 2);
        println!("test_remove_element.val = {:?}", nums);
    }

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        Solution::remove_duplicates(&mut nums);
        println!("test_remove_duplicates.val = {:?}", nums);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        Solution::remove_duplicates(&mut nums);
        println!("test_remove_duplicates.val = {:?}", nums);
    }

    #[test]
    fn test_check_if_exist() {
        let nums = vec![10, 2, 5, 3];
        let exist = Solution::check_if_exist(nums);
        println!("test_check_if_exist.val = {:?}", exist);

        let nums = vec![7, 1, 14, 11];
        let exist = Solution::check_if_exist(nums);
        println!("test_check_if_exist.val = {:?}", exist);

        let nums = vec![3, 1, 7, 11];
        let exist = Solution::check_if_exist(nums);
        println!("test_check_if_exist.val = {:?}", exist);
    }

    #[test]
    fn test_valid_mountain_array() {
        let test_example = vec![
            (vec![2, 1], Judge::<bool>::new(AssertType::Eq, false)),
            (vec![3, 5, 5], Judge::<bool>::new(AssertType::Eq, false)),
            (vec![0, 3, 2, 1], Judge::<bool>::new(AssertType::Eq, true)),
        ];

        for (args, judge) in test_example {
            let is_mountain = Solution::valid_mountain_array(args);
            judge.assert(is_mountain);
        }
    }
}
