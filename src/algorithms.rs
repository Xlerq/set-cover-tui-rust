use crate::SetCoverData;
use std::collections::HashSet;

pub fn greedy_algorithm(data: &SetCoverData) -> Vec<usize> {
    let universe_set: HashSet<usize> = data.universe.iter().copied().collect();
    let mut covered: HashSet<usize> = HashSet::new();
    let mut result = Vec::new();

    while !universe_set.is_subset(&covered) {
        let mut best_subset_idx = None;
        let mut best_value = f64::MIN;
        let mut best_new_elements = 0;
        let mut best_cost = usize::MAX;

        for (i, subset) in data.subsets.iter().enumerate() {
            if result.contains(&i) {
                continue;
            }

            let new_elements: Vec<usize> = subset.elements.iter()
            .filter(|&&elem| !covered.contains(&elem))
            .copied()
            .collect();

            let new_elements_count = new_elements.len();
            if new_elements_count == 0 {
                continue;
            }

            let value = new_elements_count as f64 / subset.cost as f64;
            if value > best_value || (value == best_value && new_elements_count > best_new_elements) || (value == best_value && new_elements_count == best_new_elements && subset.cost < best_cost) {
                best_value = value;
                best_new_elements = new_elements_count;
                best_cost = subset.cost;
                best_subset_idx = Some(i);
            }
        }

        if let Some(idx) = best_subset_idx {
            result.push(idx);
            for &elem in &data.subsets[idx].elements {
                covered.insert(elem);
            }
        } else {
            break;
        }
    }

    result
}

pub fn optimal_algorithm(data: &SetCoverData) -> Vec<usize> {
    let n = data.subsets.len();
    let universe_size = data.universe.len();

    let mut elem_to_bit: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for (bit, &elem) in data.universe.iter().enumerate() {
        elem_to_bit.insert(elem, bit);
    }

    let mut subset_masks: Vec<usize> = vec![0; n];
    for (i, subset) in data.subsets.iter().enumerate() {
        let mut mask = 0usize;
        for &elem in &subset.elements {
            if let Some(&bit) = elem_to_bit.get(&elem) {
                mask |= 1usize << bit;
            }
        }
        subset_masks[i] = mask;
    }

    let full_mask = (1usize << universe_size) - 1;
    let mut dp: Vec<usize> = vec![usize::MAX / 2; 1 << universe_size];
    dp[0] = 0;

    let mut chosen_subset: Vec<Option<(usize, usize)>> = vec![None; 1 << universe_size];

    for mask in 0..(1 << universe_size) {
        for i in 0..n {
            let new_mask = mask | subset_masks[i];
            let new_cost = dp[mask].saturating_add(data.subsets[i].cost);
            if new_cost < dp[new_mask] {
                dp[new_mask] = new_cost;
                chosen_subset[new_mask] = Some((mask, i));
            }
        }
    }

    if dp[full_mask] == usize::MAX / 2 {
        return vec![];
    }

    let mut result = Vec::new();
    let mut current_mask = full_mask;
    while current_mask != 0 {
        if let Some((prev_mask, subset_idx)) = chosen_subset[current_mask] {
            result.push(subset_idx);
            current_mask = prev_mask;
        } else {
            break;
        }
    }

    result.reverse();
    result
}
