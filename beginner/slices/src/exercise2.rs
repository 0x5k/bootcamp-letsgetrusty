fn main() {
    // Define the array of integers to search within.
    let nums = [1, 1, 2, 3, 5, 8, 13];
    // Define the target sum we are looking for in a subarray.
    let target_sum = 16;

    // Call the find_subarray function to search for a contiguous subarray
    // within 'nums' that sums up to 'target_sum'.
    // We pass a slice covering the entire 'nums' array using '&nums[..]'.
    let res = find_subarray(&nums[..], target_sum);

    // Check the result returned by find_subarray.
    // The function returns (nums.len(), nums.len()) as a signal if no subarray is found.
    // We check the first element of the tuple (the potential start index).
    if res.0 == nums.len() {
        // If the start index equals the array length, no subarray was found.
        println!("No subarray found");
    } else {
        // Otherwise, a subarray was found. The tuple 'res' contains (start_index, length).
        // Create a slice of the original 'nums' array using the found start index and length.
        // The slice range is `start .. start + length`.
        let found_slice = &nums[res.0..res.0 + res.1];
        // Print the found subarray. The `{:?}` format specifier is used for debug printing arrays/slices.
        println!("Subarray found: {:?}", found_slice);
    }
}

// This function searches for a contiguous subarray within the given slice 'nums'
// whose elements sum up to the target 'sum'.
// It prioritizes finding the longest possible subarray first.
// Parameters:
//   - nums: A slice of integers (&[i32]) representing the array to search within.
//   - sum: The target sum (i32) that the subarray elements should add up to.
// Returns:
//   - A tuple (usize, usize) representing (start_index, length) of the found subarray.
//   - If no such subarray is found, it returns (nums.len(), nums.len()) as a signal.
fn find_subarray(nums: &[i32], sum: i32) -> (usize, usize) {
    // Outer loop: Iterate through possible lengths of subarrays.
    // `(1..nums.len() + 1)` creates a range from 1 up to the total length of 'nums' (inclusive).
    // `.rev()` reverses this range, so we start checking the largest possible length
    // (the whole array) and go down to length 1. This ensures we find the longest match first.
    for len in (1..nums.len() + 1).rev() {
        // Inner loop: Iterate through all possible starting indices for a subarray of the current 'len'.
        // The logic `0..nums.len() - len + 1` ensures 'start' goes from 0 up to the last possible
        // starting position for a subarray of length 'len'.
        for start in 0..nums.len() - len + 1 {
            // Create a slice representing the current subarray candidate.
            // It starts at index 'start' and has length 'len'.
            let subarray_candidate = &nums[start..start + len];

            // Calculate the sum of the elements in the current subarray candidate
            // by calling the helper function `array_sum`.
            // Compare the calculated sum with the target 'sum'.
            if array_sum(subarray_candidate) == sum {
                // If the sum matches the target, we've found the desired subarray.
                // Immediately return the starting index ('start') and the length ('len').
                // Because the outer loop iterates length in reverse, this is the longest one possible
                // that starts earliest for that length.
                return (start, len);
            }
        }
    }

    // If the loops complete without finding any subarray that sums to the target 'sum'
    // (i.e., the `return` statement inside the loops was never hit),
    // return the special signal value (array_length, array_length) to indicate "not found".
    (nums.len(), nums.len())
}

// A helper function to calculate the sum of elements in an integer slice.
// Parameters:
//   - nums: A slice of integers (&[i32]).
// Returns:
//   - The sum of all elements in the slice (i32).
fn array_sum(nums: &[i32]) -> i32 {
    // Initialize the result (sum) to 0.
    let mut res = 0;
    // Iterate through each number ('num') in the input slice 'nums'.
    for num in nums {
        // Add the current number to the running sum.
        res += num;
    }
    // Return the final calculated sum.
    res
}
