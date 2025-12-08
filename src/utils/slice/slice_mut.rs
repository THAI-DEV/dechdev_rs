use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub fn sort_slice_mutably<T: Copy + Debug>(slice: &mut [T], is_sorted_ascending: bool) {
    if is_sorted_ascending {
        slice.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    } else {
        slice.sort_by(|a, b| format!("{:?}", b).cmp(&format!("{:?}", a)));
    }
}

pub fn reverse_slice_mutably<T: Copy>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len / 2 {
        slice.swap(i, len - 1 - i);
    }
}

pub fn remove_slice_mutably<T: Copy>(slice: &mut Vec<T>, remove_index_list: &[usize]) {
    //remove duplicate value in index_list
    let mut unique_indices: Vec<usize> = remove_index_list.to_vec();
    unique_indices.sort_unstable();
    unique_indices.dedup();

    let mut indices: Vec<usize> = unique_indices;
    indices.sort_unstable_by(|a, b| b.cmp(a)); // sort indices in descending order
    for &index in indices.iter() {
        if index < slice.len() {
            slice.remove(index);
        }
    }
}

pub fn unique_slice_mutably<T: Copy + PartialEq>(slice: &mut Vec<T>) {
    let mut result = Vec::new();
    for item in slice.iter() {
        if !result.contains(item) {
            result.push(*item);
        }
    }

    *slice = result;
}

pub fn remove_duplicate_slice_mutably<T: Copy + PartialEq + Eq + Hash>(slice: &mut Vec<T>) {
    //if item is duplicate, remove all occurrence of item
    let mut counts = HashMap::new();
    for item in slice.iter() {
        *counts.entry(item).or_insert(0) += 1;
    }

    let mut result = Vec::new();
    for item in slice.iter() {
        if let Some(&count) = counts.get(item)
            && count == 1
        {
            result.push(*item);
        }
    }

    *slice = result;
}

pub fn keep_duplicate_slice_mutably<T: Copy + PartialEq + Eq + Hash>(slice: &mut Vec<T>) {
    //if item is duplicate, keep all occurrence of item and remove unique items
    let mut counts = HashMap::new();
    for item in slice.iter() {
        *counts.entry(item).or_insert(0) += 1;
    }

    let mut result = Vec::new();
    for item in slice.iter() {
        if let Some(&count) = counts.get(item)
            && count > 1
        {
            result.push(*item);
        }
    }

    *slice = result;
}
