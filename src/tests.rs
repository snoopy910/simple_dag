#[cfg(test)]
use super::*;

#[test]
fn test_calculate_average_depth_happy_path() {
    // Happy test case for calculate_average_depth function
    let happy_dep = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(calculate_average_depth(&happy_dep), Ok(2.5));
    // Unhappy test case for calculate_average_depth function
    let unhappy_dep = vec![];
    assert_eq!(calculate_average_depth(&unhappy_dep), Err("Input for average depth is invalid".to_string()));
}

#[test]
fn test_calculate_average_nodes_per_depth_happy_path() {
    // Happy test case for calculate_average_nodes_per_depth function
    let happy_dep = vec![0, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    assert_eq!(calculate_average_nodes_per_depth(&happy_dep), Ok(3.0));
    // Unhappy test case for calculate_average_nodes_per_depth function
    let unhappy_dep = vec![];
    assert_eq!(calculate_average_nodes_per_depth(&unhappy_dep), Err("Input for nodes per depth is invalid".to_string()));
}

#[test]
fn test_calculate_average_in_references_happy_path() {
    // Happy test case for calculate_average_in_references function
    let happy_connections = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
    assert_eq!(calculate_average_in_references(&happy_connections), Ok(2.5));
    // Unhappy test case for calculate_average_in_references function
    let unhappy_connections = vec![];
    assert_eq!(calculate_average_in_references(&unhappy_connections), Err("Input for average in references is invalid".to_string()));
}

#[test]
fn test_has_cycle() {
    // Test case for has_cycle function
    let connections = vec![vec![1, 2], vec![2], vec![]];
    assert_eq!(has_cycle(3, &connections), false);
}

#[test]
fn test_is_valid_input() {
    // Test case for is_valid_input function
    let valid_input = "3\n1 1\n1 2\n2 3";
    assert_eq!(is_valid_input(valid_input), true);

    let invalid_input = "invalid input";
    assert_eq!(is_valid_input(invalid_input), false);
}

#[test]
fn test_construct_connections_util() {
    // Test case for construct_connections_util function
    let connections = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
    assert_eq!(construct_connections_util(3, &connections), vec![0, 1, 1]);
}
