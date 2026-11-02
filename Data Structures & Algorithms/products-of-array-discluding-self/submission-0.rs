impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product = 1;
        let mut result = vec![0i32; nums.len()];
        let mut zero_count = 0;
    
        for num in &nums {
            if num != &0 {
                product *= num;
            } else {
                zero_count += 1;

            }

            if zero_count > 1 {
                product = 0;
            }
        }

        for (i, num) in (&nums).into_iter().enumerate() {
            match zero_count {
                0 => {
                    result[i] = product / num;
                },
                1 => {
                    if num == &0 {
                        result[i] = product;
                    } else {
                        result[i] = 0;
                    }
                },
                _ => {
                    return result;
                }
            }
        }

        result
    }
}
