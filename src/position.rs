use std::cmp::Ordering;

fn cmp_value(lhs: f64, rhs: f64) -> Ordering {
    if lhs == rhs {
        Ordering::Equal
    } else if lhs > rhs {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn position(values: &Vec<f64>, idx: usize, len: usize, min: f64, max: f64) -> (f64, Vec<f64>) {
    let x = if len == 1 {
        0.5
    } else {
        idx as f64 / (len - 1) as f64
    };
    if min == max {
        return (x, values.iter().map(|_| 0.5).collect());
    }
    let y = values
        .iter()
        .map(|value| (value - min) / (max - min))
        .collect();
    (x, y)
}

pub fn values_to_fractions(values: &Vec<Vec<f64>>) -> Option<Vec<(f64, Vec<f64>)>> {
    let len = values.len();
    if len == 0 {
        return None;
    }
    let max = *values
        .iter()
        .flatten()
        .max_by(|lhs, rhs| cmp_value(**lhs, **rhs))
        .expect("asserted len > 0");
    let min = *values
        .iter()
        .flatten()
        .min_by(|lhs, rhs| cmp_value(**lhs, **rhs))
        .expect("asserted len > 0");

    let instructions = values
        .iter()
        .enumerate()
        .map(|(idx, values)| position(values, idx, len, min, max))
        .collect();
    Some(instructions)
}
