use crate::utils::group::{
    arrange_group_as_distribute, arrange_group_as_separate, convert_string_param_to_vector,
};

pub fn example_arrange_group() {
    let result = convert_string_param_to_vector("DM-01-01-01-01, MPS001", ",");
    println!("{:?}", result); // Output: ["DM-01-01-01-01", "MPS001"]

    println!("-------------------");

    let input: Vec<String> = (1..=9).map(|i| format!("{:02}", i)).collect();

    let out_true = arrange_group_as_separate(&input, 2, true);
    println!("isAsymmetryMember = true => {:?}", out_true);

    let out_false = arrange_group_as_separate(&input, 2, false);
    println!("isAsymmetryMember = false => {:?}", out_false);

    println!("-------------------");

    let out_distribute = arrange_group_as_distribute(&input, 2);
    println!("arrang_group_as_distribute => {:?}", out_distribute);
}
