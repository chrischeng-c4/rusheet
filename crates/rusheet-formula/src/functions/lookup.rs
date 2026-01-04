use rusheet_core::{CellError, CellValue};

/// MATCH - Search for a value in an array and return its relative position
/// Args: lookup_value, lookup_array (slice), match_type
pub fn match_fn(
    lookup_value: &CellValue,
    lookup_array: &[CellValue],
    match_type: i32,
) -> CellValue {
    if lookup_array.is_empty() {
        return CellValue::Error(CellError::NotAvailable);
    }

    match match_type {
        0 => match_exact(lookup_value, lookup_array),
        1 => match_less_than(lookup_value, lookup_array),
        -1 => match_greater_than(lookup_value, lookup_array),
        _ => CellValue::Error(CellError::InvalidValue),
    }
}

fn match_exact(target: &CellValue, array: &[CellValue]) -> CellValue {
    // Basic linear search for exact match
    // Support wildcards? Excel supports * and ? for text in match_type 0.
    // For now, let's just do equality.
    
    for (i, value) in array.iter().enumerate() {
        if values_equal(target, value) {
            return CellValue::Number((i + 1) as f64);
        }
    }
    
    CellValue::Error(CellError::NotAvailable)
}

fn match_less_than(target: &CellValue, array: &[CellValue]) -> CellValue {
    // Array must be sorted ascending.
    // Finds largest value that is <= target.
    // Since it's sorted, we can iterate and stop when we exceed target, or use last match.
    
    let mut best_idx = None;
    
    for (i, value) in array.iter().enumerate() {
        match compare_values(value, target) {
            Some(ordering) => {
                if ordering <= 0 { // value <= target
                    best_idx = Some(i + 1);
                } else {
                    // value > target. Since sorted ascending, we can stop.
                    // But Excel might strictly require sorting. If not sorted, result is undefined.
                    // We'll assume best effort or strict undefined.
                    // Let's just update best_idx.
                }
            },
            None => {} // Skip mismatching types?
        }
    }
    
    match best_idx {
        Some(idx) => CellValue::Number(idx as f64),
        None => CellValue::Error(CellError::NotAvailable),
    }
}

fn match_greater_than(target: &CellValue, array: &[CellValue]) -> CellValue {
    // Array must be sorted descending.
    // Finds smallest value that is >= target.
    
    let mut best_idx = None;
    
    for (i, value) in array.iter().enumerate() {
        match compare_values(value, target) {
            Some(ordering) => {
                if ordering >= 0 { // value >= target
                    best_idx = Some(i + 1);
                }
            },
            None => {}
        }
    }
    
    match best_idx {
        Some(idx) => CellValue::Number(idx as f64),
        None => CellValue::Error(CellError::NotAvailable),
    }
}

fn values_equal(a: &CellValue, b: &CellValue) -> bool {
    match (a, b) {
        (CellValue::Number(n1), CellValue::Number(n2)) => (n1 - n2).abs() < 1e-10,
        (CellValue::Text(s1), CellValue::Text(s2)) => s1.to_lowercase() == s2.to_lowercase(),
        (CellValue::Boolean(b1), CellValue::Boolean(b2)) => b1 == b2,
        _ => false,
    }
}

fn compare_values(a: &CellValue, b: &CellValue) -> Option<i8> {
    match (a, b) {
        (CellValue::Number(n1), CellValue::Number(n2)) => {
            if n1 < n2 { Some(-1) } else if n1 > n2 { Some(1) } else { Some(0) }
        },
        (CellValue::Text(s1), CellValue::Text(s2)) => {
            let cmp = s1.to_lowercase().cmp(&s2.to_lowercase());
            Some(if cmp == std::cmp::Ordering::Less { -1 } else if cmp == std::cmp::Ordering::Greater { 1 } else { 0 })
        },
        (CellValue::Boolean(b1), CellValue::Boolean(b2)) => {
            let n1 = if *b1 { 1 } else { 0 };
            let n2 = if *b2 { 1 } else { 0 };
            if n1 < n2 { Some(-1) } else if n1 > n2 { Some(1) } else { Some(0) }
        }
        _ => None, // Incomparable types
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_exact() {
        let array = vec![
            CellValue::Number(10.0),
            CellValue::Number(20.0),
            CellValue::Number(30.0),
        ];
        
        assert_eq!(match_fn(&CellValue::Number(20.0), &array, 0), CellValue::Number(2.0));
        assert!(matches!(match_fn(&CellValue::Number(25.0), &array, 0), CellValue::Error(CellError::NotAvailable)));
    }

    #[test]
    fn test_match_less_than() {
        // Sorted ascending: 10, 20, 30
        let array = vec![
            CellValue::Number(10.0),
            CellValue::Number(20.0),
            CellValue::Number(30.0),
        ];
        
        // Match 25 -> Should find 20 (index 2)
        assert_eq!(match_fn(&CellValue::Number(25.0), &array, 1), CellValue::Number(2.0));
        
        // Match 9 -> #N/A (less than smallest)
        assert!(matches!(match_fn(&CellValue::Number(9.0), &array, 1), CellValue::Error(CellError::NotAvailable)));
        
        // Match 35 -> 30 (index 3)
        assert_eq!(match_fn(&CellValue::Number(35.0), &array, 1), CellValue::Number(3.0));
    }

    #[test]
    fn test_match_greater_than() {
        // Sorted descending: 30, 20, 10
        let array = vec![
            CellValue::Number(30.0),
            CellValue::Number(20.0),
            CellValue::Number(10.0),
        ];
        
        // Match 25 -> Should find 30 (index 1) - smallest value >= 25
        assert_eq!(match_fn(&CellValue::Number(25.0), &array, -1), CellValue::Number(1.0));
        
        // Match 5 -> 10 (index 3)
        assert_eq!(match_fn(&CellValue::Number(5.0), &array, -1), CellValue::Number(3.0));
        
        // Match 35 -> #N/A (greater than largest)
        assert!(matches!(match_fn(&CellValue::Number(35.0), &array, -1), CellValue::Error(CellError::NotAvailable)));
    }
    
    #[test]
    fn test_match_text() {
        let array = vec![
            CellValue::Text("Apple".to_string()),
            CellValue::Text("Banana".to_string()),
            CellValue::Text("Cherry".to_string()),
        ];
        
        assert_eq!(match_fn(&CellValue::Text("banana".to_string()), &array, 0), CellValue::Number(2.0));
    }
}
