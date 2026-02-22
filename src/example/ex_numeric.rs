use crate::utils::numeric::{
    format_float_with_commas, format_integer_with_commas, pad_float_with_zero,
    pad_integer_with_zero,
};

pub fn example_numeric() {
    ex_int();
    println!("----------------------------");
    ex_float();
}

fn ex_int() {
    let a = 1234567;
    let b = 123;

    let c = -1234567;
    let d = -123;

    let r1 = format_integer_with_commas(a);
    let r2 = format_integer_with_commas(b);
    let r3 = format_integer_with_commas(c);
    let r4 = format_integer_with_commas(d);
    let r5 = pad_integer_with_zero(b, 5);
    let r6 = pad_integer_with_zero(d, 5);
    let r7 = pad_integer_with_zero(b, 3);
    let r8 = pad_integer_with_zero(d, 3);

    println!("{}", r1);
    println!("{}", r2);
    println!("{}", r3);
    println!("{}", r4);
    println!("{}", r5);
    println!("{}", r6);
    println!("{}", r7);
    println!("{}", r8);
}

fn ex_float() {
    let a = 1234567.89;
    let b = 123.45;

    let c = -1234567.987;
    let d = -123.4567;

    let r1 = format_float_with_commas(a, 2);
    let r2 = format_float_with_commas(b, 2);
    let r3 = format_float_with_commas(c, 4);
    let r4 = format_float_with_commas(d, 3);
    let r5 = pad_float_with_zero(b, 7);
    let r6 = pad_float_with_zero(d, 9);

    println!("{}", r1);
    println!("{}", r2);
    println!("{}", r3);
    println!("{}", r4);
    println!("{}", r5);
    println!("{}", r6);
}
