pub fn round_to_precision(value: f64, precision: u32) -> f64 {
    let factor = 10f64.powi(precision as i32);
    (value * factor).round() / factor
}

pub fn format_integer_with_commas(value: isize) -> String {
    let is_negative = value < 0;
    let mut formatted = value.abs().to_string();
    let mut chars = formatted.chars().rev().collect::<Vec<_>>();
    for i in (3..chars.len()).step_by(3) {
        chars.insert(i, ',');
    }
    formatted = chars.into_iter().rev().collect();
    if is_negative {
        formatted.insert(0, '-');
    }
    formatted
}

pub fn format_float_with_commas(value: f64, precision: usize) -> String {
    let is_negative = value < 0.0;
    let abs_value = value.abs();
    let integer_part = abs_value.trunc() as i32;
    let fractional_part = abs_value.fract();

    let mut formatted_integer = format_integer_with_commas(integer_part as isize);

    if fractional_part > 0.0 {
        let fractional = format!("{:.precision$}", fractional_part, precision = precision);
        let fractional_str = fractional.trim_start_matches('0');
        formatted_integer.push_str(fractional_str);
    }

    if is_negative {
        formatted_integer.insert(0, '-');
    }

    formatted_integer
}

pub fn pad_integer_with_zero(value: isize, len: usize) -> String {
    let is_negative = value < 0;
    let mut formatted = value.abs().to_string();
    while formatted.len() < len {
        formatted = format!("0{}", formatted);
    }
    if is_negative {
        formatted.insert(0, '-');
    }
    formatted
}

pub fn pad_float_with_zero(value: f64, len: usize) -> String {
    let is_negative = value < 0.0;
    let mut formatted = value.abs().to_string();
    while formatted.len() < len {
        formatted = format!("0{}", formatted);
    }
    if is_negative {
        formatted.insert(0, '-');
    }
    formatted
}
