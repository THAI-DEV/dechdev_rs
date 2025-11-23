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
