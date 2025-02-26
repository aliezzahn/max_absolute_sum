# Max Absolute Sum

A Rust library that calculates the maximum absolute sum of any subarray within a given array of integers.

![CI](https://github.com/aliezzahn/max_absolute_sum/actions/workflows/ci.yml/badge.svg)
![CD](https://github.com/aliezzahn/max_absolute_sum/actions/workflows/cd.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Problem Description

This library solves the "Maximum Absolute Sum of Any Subarray" problem. Given an array of integers, it finds the maximum absolute sum of any contiguous subarray. The absolute sum is defined as the maximum value between the largest positive sum and the absolute value of the largest negative sum of any subarray.

### Examples

- **Input**: `[1, -3, 2, 3, -4]`  
  **Output**: `5`  
  **Explanation**: The subarray `[2, 3]` has a sum of 5, which is the maximum absolute sum.
- **Input**: `[2, -5, 1, -4, 3, -2]`  
  **Output**: `8`  
  **Explanation**: The subarray `[-5, 1, -4]` has a sum of -8, with an absolute value of 8.

## Features

- **Time Complexity**: O(n), where n is the length of the input array
- **Space Complexity**: O(1) extra space
- Efficient single-pass algorithm based on a modified Kadane's algorithm
- Handles edge cases including integer overflow (e.g., `i32::MIN`)
- Comprehensive test suite covering typical and edge cases

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
max_absolute_sum = { path = "../path/to/max_absolute_sum" }
```

Or if published to crates.io (future possibility):

```toml
[dependencies]
max_absolute_sum = "0.1.0"
```

## Usage

```rust
use max_absolute_sum::Solution;

fn main() {
    let nums = vec![1, -3, 2, 3, -4];
    let result = Solution::max_absolute_sum(nums);
    println!("Maximum absolute sum: {}", result); // Outputs: 5
}
```

## API

### `Solution::max_absolute_sum(nums: Vec<i32>) -> i32`

Calculates the maximum absolute sum of any subarray.

- **Parameters**:
  - `nums`: Vector of 32-bit integers
- **Returns**:
  - `i32`: Maximum absolute sum found
- **Complexity**:
  - Time: O(n)
  - Space: O(1)

## Building and Testing

Clone the repository and run:

```bash
# Build the library
cargo build

# Run tests
cargo test
```

## Implementation Details

The solution uses a modified Kadane's algorithm that:

1. Tracks maximum and minimum sums ending at each position (`max_ending_here`, `min_ending_here`)
2. Maintains global maximum and minimum sums (`max_so_far`, `min_so_far`)
3. Uses only four i32 variables for O(1) space complexity
4. Processes the array in a single pass
5. Handles integer overflow by safely computing the absolute value of minimum sums

## Test Cases

The library includes tests covering:

- **Basic case**: `[1, -3, 2, 3, -4]` → 5
- **All positive**: `[1, 2, 3, 4, 5]` → 15
- **All negative**: `[-1, -2, -3, -4, -5]` → 15
- **Single element**: `[5]` → 5, `[-5]` → 5
- **All zeros**: `[0, 0, 0]` → 0
- **Mixed with zeros**: `[2, -1, 0, 3, -2]` → 4
- **Alternating**: `[1, -2, 1, -2, 1]` → 3 (corrected from `[1, -1, 1, -1, 1]` which yields 1)
- **Empty**: `[]` → 0
- **Large numbers**: `[i32::MAX, i32::MIN, 100]` → `i32::MAX`

### Note on Alternating Test

The original test case `[1, -1, 1, -1, 1]` expecting 3 was incorrect, as its maximum absolute sum is 1. It has been corrected to `[1, -2, 1, -2, 1]` to match the expected output of 3, aligning with a plausible intent based on the problem's context.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -am 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Create a new Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Based on LeetCode problem "Maximum Absolute Sum of Any Subarray"
- Built with Rust and Cargo
- Created by [Your Name]

## Troubleshooting

If tests fail:

- Ensure Rust is up to date: `rustup update`
- Check for integer overflow in edge cases; the current implementation handles this with i64 casting
