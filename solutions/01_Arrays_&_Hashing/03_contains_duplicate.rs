fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut duplciates_set: HashSet<i32> = HashSet::new();

    for num in nums {
        if duplciates_set.contains(&num) {
            return true;
        }
        duplciates_set.insert(num);
    }

    return false;
}
