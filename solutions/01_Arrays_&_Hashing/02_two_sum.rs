use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Result<[u32; 2], String> {
    let mut hashmap: HashMap<i32, u32> = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        let difference = target - num;
        let num_index = index.try_into().unwrap();

        if let Some(idx) = hashmap.get(&difference) {
            return Ok([*idx, num_index]);
        }
        hashmap.insert(*num, num_index);
    }

    return Err("Coconut oil".to_string());
}
