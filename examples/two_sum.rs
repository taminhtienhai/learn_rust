fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut start = 0;
    let end = nums.len() - 1;

    while start <= end {
        let leftover = target - nums[start];

        if let Ok(position) = nums.binary_search(&leftover) {
            if position == start { start += 1; continue; }

            return vec![start as i32, position as i32];
        }

        start += 1;
    }

    return vec![];
}

fn main() {
    let result = two_sum(vec![0,4,3,0], 6);

    println!("{result:?}");
}