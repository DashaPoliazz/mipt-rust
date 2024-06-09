pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    if k <= 0 {
        return vec![vec![]];
    }

    let mut result = vec![];

    for i in 0..arr.len() {
        let remaining = &arr[i + 1..];
        let sub_combination = combinations(remaining, k - 1);
        for mut combination in sub_combination {
            combination.insert(0, arr[i]);
            result.push(combination);
        }
    }

    return result;
}
