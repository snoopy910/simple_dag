// Importing necessary modules from the standard library
use std::collections::{HashMap, VecDeque};
use std::fs;

/// Calculates the average depth of a given array of depths.
/// 
/// # Arguments
///
/// * `dep` - A reference to an array of depths (`&[i32]`).
///
/// # Returns
///
/// The average depth as a floating-point number (`f64`).
///
fn calculate_average_depth(dep: &[i32]) -> Result<f64, String> {
    if dep.len() == 0 {
      return Err("Input for average depth is invalid".to_string());  
    } else {
        let sum: i32 = dep.iter().sum();
        let average = sum as f64 / dep.len() as f64;
        Ok(average)
    }
}

/// Calculates the average number of nodes per depth level in a given array of depths.
/// 
/// # Arguments
///
/// * `dep` - A reference to an array of depths (`&[i32]`).
///
/// # Returns
///
/// The average nodes per depth as a floating-point number (`f64`).
///
fn calculate_average_nodes_per_depth(dep: &[i32]) -> Result<f64, String> {
    if dep.len() == 0 {
        return Err("Input for nodes per depth is invalid".to_string());
    } else {
    let mut counts = HashMap::new();

    for &elem in dep {
        let count = counts.entry(elem).or_insert(0);
        *count += 1;
    }

    let result_vec: Vec<_> = counts
        .values()
        .cloned()
        .filter(|&count| count > 1)
        .collect();

    let sum: i32 = result_vec.iter().sum();
    let average: f64 = sum as f64 / result_vec.len() as f64;

    Ok(average)
    }
}

/// Calculates the average number of incoming references for each node in the graph.
///
/// # Arguments
///
/// * `connections` - A reference to a 2D array representing connections between nodes (`&[Vec<i32>]`).
///
/// # Returns
///
/// The average number of incoming references as a floating-point number (`f64`).
///
fn calculate_average_in_references(connections: &[Vec<i32>]) -> Result<f64, String> {
    if connections.len() == 0 {
        Err("Input for average in references is invalid".to_string())
    } else {
        let sum: usize = connections.iter().map(|row| row.len()).sum();
        let average: f64 = sum as f64 / connections.len() as f64;

        Ok(average)
    }
}

/// Checks if the graph represented by the given connections has a cycle.
///
/// # Arguments
///
/// * `n` - The number of nodes in the graph.
/// * `connections` - A reference to a 2D array representing connections between nodes (`&[Vec<i32>]`).
///
/// # Returns
///
/// `true` if the graph has a cycle, `false` otherwise.
///
fn has_cycle(n: i32, connections: &[Vec<i32>]) -> bool {
    let mut visited = vec![false; n as usize];
    let mut rec_stack = vec![false; n as usize];

    for i in 0..n as usize {
        if has_cycle_util(i, &mut visited, &mut rec_stack, connections) {
            return true;
        }
    }

    false
}

/// Utility function to check for cycles using depth-first search (DFS).
///
/// # Arguments
///
/// * `node` - Current node being visited
/// * `visited` - Vector to track visited nodes.
/// * `rec_stack` - Stack to track nodes in the current recursion stack
/// * `connections` - Adjacency list representing the graph connections
/// 
/// # Returns
///
/// `true` if the current status has a cycle, `false` otherwise.
///
fn has_cycle_util(
    node: usize,
    visited: &mut Vec<bool>,
    rec_stack: &mut Vec<bool>,
    connections: &[Vec<i32>],
) -> bool {
    if !visited[node] {
        visited[node] = true;
        rec_stack[node] = true;

        for &neighbor in &connections[node] {
            if !visited[neighbor as usize]
                && has_cycle_util(neighbor as usize, visited, rec_stack, connections)
            {
                return true;
            } else if rec_stack[neighbor as usize] {
                return true;
            }
        }
    }

    rec_stack[node] = false;
    false
}

/// Validates the input contents to ensure it meets the expected format.
///
/// # Arguments
///
/// * `contents` - A reference to the input contents as a string (`&str`).
///
/// # Returns
///
/// `true` if the input is valid, `false` otherwise.
///
fn is_valid_input(contents: &str) -> bool {
    let mut lines = contents.lines();
    if let Some(first_line) = lines.next() {
        if let Ok(n) = first_line.parse::<i32>() {
            let mut num_lines = 0;
            let mut max_index = 0;

            for (index, line) in lines.enumerate() {
                let parents_of_node: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if parents_of_node.len() != 2 {
                    println!("Attributes of each node has only 2, left and right");
                    return false;
                }

                if parents_of_node.iter().any(|&x| x <= 0) {
                    println!("Valid parents information");
                    return false;
                }

                if parents_of_node.iter().max().unwrap() > &max_index {
                    max_index = *parents_of_node.iter().max().unwrap();
                }

                num_lines = index + 1;
            }

            if num_lines != n as usize {
                println!("Number of lines must match the number of nodes in the first line");
                return false;
            }

            if max_index > n + 1 {
                println!("Max index of nodes can't exceed the number of nodes");
                return false;
            }

            true // Return true if all checks pass
        } else {
            println!("Number of nodes must be an integer");
            false // Return false if the number of nodes is not an integer
        }
    } else {
        println!("The file is empty");
        false // Return false if the file is empty
    }
}

/// Constructs a 2D array representing connections between nodes based on the input contents.
///
/// # Arguments
///
/// `filename` - The name of the file containing the input contents.
///
/// # Returns
///
/// A result containing the number of nodes and a 2D array representing connections between nodes (`Result<(i32, Vec<Vec<i32>>), Box<dyn std::error::Error>>`).
///
fn construct_connections(
    filename: &str,
) -> Result<(i32, Vec<Vec<i32>>), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;

    if !is_valid_input(&contents) {
        return Err("Invalid input format".into()); // Return an error if the input format is invalid
    }
    let mut connections: Vec<Vec<i32>> = Vec::new(); // Initialize a vector to store the connections
    let mut n: i32 = 0; // Initialize the number of nodes

    for (index, line) in contents.lines().enumerate() {
        if index == 0 {
            n = line.parse()?;
            connections = vec![Vec::new(); (n + 1) as usize];
        } else {
            let connection: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let left = connection[0] as usize;
            let right = connection[1] as usize;
            connections[left - 1].push(index as i32);
            connections[right - 1].push(index as i32);
        }
    }
    Ok((n + 1, connections))
}

/// Constructs an array representing the depth of each node in the graph using BFS (Breadth-First Search).
///
/// # Arguments
///
/// * `n` - The number of nodes in the graph.
/// * `connections` - A reference to a 2D array representing connections between nodes (`&[Vec<i32>]`).
///
/// # Returns
///
/// An array representing the depth of each node in the graph (`Vec<i32>`).
///
fn construct_connections_util(n: i32, connections: &[Vec<i32>]) -> Vec<i32> {
    let mut visited = vec![false; n as usize];
    let mut queue = VecDeque::new(); // Select queue as a data structure for BFS
    let mut dep = vec![0; n as usize];
    queue.push_back(0);
    visited[0] = true;

    while let Some(node) = queue.pop_front() {
        for &neighbor in &connections[node as usize] {
            if !visited[neighbor as usize] {
                visited[neighbor as usize] = true;
                dep[neighbor as usize] = dep[node as usize] + 1;
                queue.push_back(neighbor);
            }
        }
    }
    dep
}

fn main() {
    let filename = "database.txt";
    let (n, connections) = match construct_connections(filename) {
        Ok((parsed_n, parsed_connections)) => (parsed_n, parsed_connections),
        Err(e) => {
            eprintln!("Failed to construct connections: {}", e);
            return;
        }
    };

    let has_cycle = has_cycle(n, &connections);
    if has_cycle {
        println!("DAG has cycle");
    } else {
        let dep = construct_connections_util(n, &connections);

        if let Ok(avg_depth) = calculate_average_depth(&dep) {
            println!("> AVE DAG DEPTH: {}", avg_depth);
        } else {
            println!("Error calculating average depth");
        }
        
        if let Ok(avg_nodes_per_depth) = calculate_average_nodes_per_depth(&dep) {
            println!("> AVG NODES PER DEPTH: {}", avg_nodes_per_depth);
        } else {
            println!("Error calculating average nodes per depth");
        }

        if let Ok(avg_ref) = calculate_average_in_references(&connections) {
            println!("> AVG IN-REF: {}", avg_ref);
        } else {
            println!("Error calculating average references");
        }
    }
}

#[cfg(test)]
mod tests;
