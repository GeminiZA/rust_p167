struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut a = 0;
        let mut b = numbers.len() - 1;
        while numbers[a] + numbers[b] != target {
            if numbers[a] + numbers[b] < target {
                a += 1;
            } else if numbers[a] + numbers[b] > target {
                b -= 1;
            }
        }
        return vec![a as i32 + 1, b as i32 + 1];
    }
}

fn main() {
    let numbers: Vec<i32> = vec![-1,0];
    let target: i32 = -1;
    let result = Solution::two_sum(numbers, target);
    println!("{:?}", result);
}
