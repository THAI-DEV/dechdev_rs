use crate::utils::slice::{
    combine_slice, difference_slice, find_index, intersect_slice, mutate_remove_slice,
    mutate_reverse_slice, mutate_sort_slice, remove_slice, reverse_slice, sort_slice, union_slice,
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

    let r = reverse_slice(&vec1);
    println!("reverse : {:?}", r);

    let vec4 = ["f", "b", "c", "d", "a"];
    let r = sort_slice(&vec4, false);
    println!("sort_slice : {:?}", r);

    let vec5 = ["f", "b", "c", "d", "a"];
    let r = remove_slice(&vec5, &[1, 3]);
    println!("remove_slice : {:?}", r);

    let mut vex6 = ["f", "b", "c", "d", "a"];
    mutate_reverse_slice(&mut vex6);
    println!("mutate_reverse_slice : {:?}", vex6);

    mutate_sort_slice(&mut vex6, true);
    println!("mutate_sort_slice : {:?}", vex6);

    let mut v4 = vec!["f", "b", "c", "d", "a"];
    mutate_remove_slice(&mut v4, &[3, 1, 0]);
    println!("mutate_remove_slice : {:?}", v4);
}
