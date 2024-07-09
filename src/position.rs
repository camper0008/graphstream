use crate::value::Value;
use std::cmp::Ordering;

pub struct Positioned {
    pub positions: Vec<(Value, Value)>,
    min: Value,
    max: Value,
}

fn cmp_value(lhs: Value, rhs: Value) -> Ordering {
    if lhs == rhs {
        Ordering::Equal
    } else if lhs > rhs {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

pub fn values_to_fractions(values: &Vec<Value>) -> Option<Positioned> {
    let len = values.len();
    if len == 0 {
        return None;
    }
    let max = *values
        .iter()
        .max_by(|lhs, rhs| cmp_value(**lhs, **rhs))
        .expect("asserted len >= 1");
    let min = *values
        .iter()
        .min_by(|lhs, rhs| cmp_value(**lhs, **rhs))
        .expect("asserted len >= 1");

    fn position(value: Value, idx: usize, len: usize, min: Value, max: Value) -> (Value, Value) {
        let x = if len == 1 {
            0.5
        } else {
            idx as Value / (len - 1) as Value
        };
        if min == max {
            return (x, 0.5);
        }
        return (x, (value - min) / (max - min));
    }

    let positions = values
        .into_iter()
        .enumerate()
        .map(|(idx, &value)| position(value, idx, len, min, max))
        .collect();
    Some(Positioned {
        positions,
        min,
        max,
    })
}
