use crate::utils::slice::{
    combine_slice, difference_slice, find_index, intersect_slice, union_slice,
};

pub fn example_slice() {
    let vec1 = vec!["1.0", "2.0", "3.0"];
    let vec2 = vec!["3.0", "4.0", "5.0", "6.0"];

    let r = union_slice(&vec1, &vec2);
    println!("union : {:?} ", r);

    let r = intersect_slice(&vec1, &vec2);
    println!("intersect : {:?}", r);

    let r = difference_slice(&vec1, &vec2);
    println!("difference : {:?}", r);

    let r = combine_slice(&vec1, &vec2);
    println!("combine : {:?}", r);

    let vec3 = vec!["a", "b", "c", "a", "d", "a"];
    let target = "a";

    let indices = find_index(&vec3, &target);
    println!("find_index of '{}' in {:?} : {:?}", target, vec3, indices);
}
