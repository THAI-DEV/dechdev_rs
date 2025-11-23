use std::fmt::Debug;

pub fn union_slice<T: PartialEq + Clone>(slice1: &[T], slice2: &[T]) -> Vec<T> {
    let mut result = Vec::new();

    for item in slice1 {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }
    for item in slice2 {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }

    result
}

pub fn intersect_slice<T: PartialEq + Clone>(slice1: &[T], slice2: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for item in slice1 {
        if slice2.contains(item) && !result.contains(item) {
            result.push(item.clone());
        }
    }
    result
}

pub fn difference_slice<T: PartialEq + Clone>(slice1: &[T], slice2: &[T]) -> Vec<T> {
    let mut result = Vec::new();
    for item in slice1 {
        if !slice2.contains(item) {
            result.push(item.clone());
        }
    }

    result
}

pub fn combine_slice<T: Clone>(slice1: &[T], slice2: &[T]) -> Vec<T> {
    let mut result = slice1.to_vec();
    result.extend_from_slice(slice2);

    result
}

pub fn find_index<T: PartialEq>(v: &[T], target: &T) -> Vec<usize> {
    v.iter()
        .enumerate()
        .filter_map(|(index, item)| if *item == *target { Some(index) } else { None })
        .collect()
}

pub fn reverse_slice<T: Copy>(slice: &[T]) -> Vec<T> {
    let len = slice.len();
    let mut result = Vec::with_capacity(len);
    for i in (0..len).rev() {
        result.push(slice[i]);
    }

    result
}

pub fn sort_slice<T: Copy + Debug>(slice: &[T], is_sorted_ascending: bool) -> Vec<T> {
    let mut result = slice.to_vec();
    if is_sorted_ascending {
        result.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    } else {
        result.sort_by(|a, b| format!("{:?}", b).cmp(&format!("{:?}", a)));
    }
    result
}

pub fn remove_slice<T: Copy>(slice: &[T], index_list: &[usize]) -> Vec<T> {
    //remove duplicate value in index_list
    let mut unique_indices: Vec<usize> = index_list.to_vec();
    unique_indices.sort_unstable();
    unique_indices.dedup();

    let mut result = Vec::new();
    for (i, item) in slice.iter().enumerate() {
        if !unique_indices.contains(&i) {
            result.push(*item);
        }
    }
    result
}

pub fn mutate_sort_slice<T: Copy + Debug>(slice: &mut [T], is_sorted_ascending: bool) {
    if is_sorted_ascending {
        slice.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
    } else {
        slice.sort_by(|a, b| format!("{:?}", b).cmp(&format!("{:?}", a)));
    }
}

pub fn mutate_reverse_slice<T: Copy>(slice: &mut [T]) {
    let len = slice.len();
    for i in 0..len / 2 {
        slice.swap(i, len - 1 - i);
    }
}

pub fn mutate_remove_slice<T: Copy>(slice: &mut Vec<T>, index_list: &[usize]) {
    //remove duplicate value in index_list
    let mut unique_indices: Vec<usize> = index_list.to_vec();
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
