use rusheet_core::{CellError, CellValue};

/// SUM - Sum all numeric values
pub fn sum(values: &[CellValue]) -> CellValue {
    let mut total = 0.0;

    for value in values {
        match value {
            CellValue::Number(n) => total += n,
            CellValue::Boolean(b) => total += if *b { 1.0 } else { 0.0 },
            CellValue::Error(e) => return CellValue::Error(e.clone()),
            _ => {} // Skip empty and text
        }
    }

    CellValue::Number(total)
}

/// AVERAGE - Average of numeric values
pub fn average(values: &[CellValue]) -> CellValue {
    let mut total = 0.0;
    let mut count = 0;

    for value in values {
        match value {
            CellValue::Number(n) => {
                total += n;
                count += 1;
            }
            CellValue::Error(e) => return CellValue::Error(e.clone()),
            _ => {} // Skip empty, text, boolean for AVERAGE
        }
    }

    if count == 0 {
        CellValue::Error(CellError::DivisionByZero)
    } else {
        CellValue::Number(total / count as f64)
    }
}

/// COUNT - Count numeric values
pub fn count(values: &[CellValue]) -> CellValue {
    let count = values
        .iter()
        .filter(|v| matches!(v, CellValue::Number(_)))
        .count();

    CellValue::Number(count as f64)
}

/// COUNTA - Count non-empty values
pub fn counta(values: &[CellValue]) -> CellValue {
    let count = values
        .iter()
        .filter(|v| !matches!(v, CellValue::Empty))
        .count();

    CellValue::Number(count as f64)
}

/// MIN - Minimum numeric value
pub fn min(values: &[CellValue]) -> CellValue {
    let mut result: Option<f64> = None;

    for value in values {
        match value {
            CellValue::Number(n) => {
                result = Some(result.map_or(*n, |r| r.min(*n)));
            }
            CellValue::Error(e) => return CellValue::Error(e.clone()),
            _ => {}
        }
    }

    match result {
        Some(n) => CellValue::Number(n),
        None => CellValue::Number(0.0),
    }
}

/// MAX - Maximum numeric value
pub fn max(values: &[CellValue]) -> CellValue {
    let mut result: Option<f64> = None;

    for value in values {
        match value {
            CellValue::Number(n) => {
                result = Some(result.map_or(*n, |r| r.max(*n)));
            }
            CellValue::Error(e) => return CellValue::Error(e.clone()),
            _ => {}
        }
    }

    match result {
        Some(n) => CellValue::Number(n),
        None => CellValue::Number(0.0),
    }
}

/// ABS - Absolute value
pub fn abs(values: &[CellValue]) -> CellValue {
    if values.is_empty() {
        return CellValue::Error(CellError::InvalidValue);
    }

    match &values[0] {
        CellValue::Number(n) => CellValue::Number(n.abs()),
        CellValue::Error(e) => CellValue::Error(e.clone()),
        _ => CellValue::Error(CellError::InvalidValue),
    }
}

/// ROUND - Round to specified decimal places
pub fn round(values: &[CellValue]) -> CellValue {
    if values.is_empty() {
        return CellValue::Error(CellError::InvalidValue);
    }

    let num = match values[0].as_number() {
        Some(n) => n,
        None => return CellValue::Error(CellError::InvalidValue),
    };

    let decimals = if values.len() > 1 {
        values[1].as_number().unwrap_or(0.0) as i32
    } else {
        0
    };

    let factor = 10_f64.powi(decimals);
    CellValue::Number((num * factor).round() / factor)
}

/// FLOOR - Round down to specified decimal places
pub fn floor(values: &[CellValue]) -> CellValue {
    if values.is_empty() {
        return CellValue::Error(CellError::InvalidValue);
    }

    let num = match values[0].as_number() {
        Some(n) => n,
        None => return CellValue::Error(CellError::InvalidValue),
    };

    let significance = if values.len() > 1 {
        values[1].as_number().unwrap_or(1.0)
    } else {
        1.0
    };

    if significance == 0.0 {
        return CellValue::Error(CellError::DivisionByZero);
    }

    CellValue::Number((num / significance).floor() * significance)
}

/// CEILING - Round up to specified decimal places
pub fn ceiling(values: &[CellValue]) -> CellValue {
    if values.is_empty() {
        return CellValue::Error(CellError::InvalidValue);
    }

    let num = match values[0].as_number() {
        Some(n) => n,
        None => return CellValue::Error(CellError::InvalidValue),
    };

    let significance = if values.len() > 1 {
        values[1].as_number().unwrap_or(1.0)
    } else {
        1.0
    };

    if significance == 0.0 {
        return CellValue::Error(CellError::DivisionByZero);
    }

    CellValue::Number((num / significance).ceil() * significance)
}

/// SQRT - Square root
pub fn sqrt(values: &[CellValue]) -> CellValue {
    if values.is_empty() {
        return CellValue::Error(CellError::InvalidValue);
    }

    match values[0].as_number() {
        Some(n) if n >= 0.0 => CellValue::Number(n.sqrt()),
        Some(_) => CellValue::Error(CellError::NumError),
        None => CellValue::Error(CellError::InvalidValue),
    }
}

/// POWER - Raise to power
pub fn power(values: &[CellValue]) -> CellValue {
    if values.len() < 2 {
        return CellValue::Error(CellError::InvalidValue);
    }

    let base = match values[0].as_number() {
        Some(n) => n,
        None => return CellValue::Error(CellError::InvalidValue),
    };

    let exp = match values[1].as_number() {
        Some(n) => n,
        None => return CellValue::Error(CellError::InvalidValue),
    };

    let result = base.powf(exp);
    if result.is_nan() || result.is_infinite() {
        CellValue::Error(CellError::NumError)
    } else {
        CellValue::Number(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let values = vec![
            CellValue::Number(1.0),
            CellValue::Number(2.0),
            CellValue::Number(3.0),
        ];
        assert_eq!(sum(&values), CellValue::Number(6.0));
    }

    #[test]
    fn test_average() {
        let values = vec![
            CellValue::Number(2.0),
            CellValue::Number(4.0),
            CellValue::Number(6.0),
        ];
        assert_eq!(average(&values), CellValue::Number(4.0));
    }

    #[test]
    fn test_count() {
        let values = vec![
            CellValue::Number(1.0),
            CellValue::Text("hello".to_string()),
            CellValue::Number(2.0),
            CellValue::Empty,
        ];
        assert_eq!(count(&values), CellValue::Number(2.0));
    }

    #[test]
    fn test_min_max() {
        let values = vec![
            CellValue::Number(5.0),
            CellValue::Number(2.0),
            CellValue::Number(8.0),
        ];
        assert_eq!(min(&values), CellValue::Number(2.0));
        assert_eq!(max(&values), CellValue::Number(8.0));
    }

    #[test]
    fn test_round() {
        let values = vec![CellValue::Number(3.14159), CellValue::Number(2.0)];
        assert_eq!(round(&values), CellValue::Number(3.14));
    }
}
