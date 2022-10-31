use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

/// Reconstructs the path from start to end using the `came_from` map.
/// It works by starting from the end and following the path backwards.
/// Here we are taking in terms of the grid (vector of vectors) and not the
/// actual coordinates or nodes of the graph.
///
/// ### Arguments
///
/// * `came_from` - A map of positions to their previous positions.
/// * `current` - The current position.
///
/// ### Returns
///
/// A vector of nodes from start to end.
///
/// ### Example
///
/// ```
/// use pathfinding::reconstruct_path;
/// use std::collections::HashMap;
///
/// let mut came_from = HashMap::new();
/// came_from.insert((1, 1), (1, 0));
/// came_from.insert((1, 0), (0, 0));
/// came_from.insert((0, 0), (0, 1));
///     
/// let path = reconstruct_path(&came_from, (1, 1));
/// assert_eq!(path, vec![(0, 1), (0, 0), (1, 0), (1, 1)    ]);
/// ```
pub fn reconstruct_path(
    came_from: &HashMap<(i32, i32), (i32, i32)>,
    current: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut total_path = vec![current];
    let mut current = current;
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push(current);
    }
    total_path.reverse();
    total_path
}

/// Get the neighbors of an element in the 2d grid (up, down, left, right) via additional predicate.
/// The predicate is used to check if the neighbor is solid or not.
///
/// ### Arguments
///
/// * `row` - The row of the element.
/// * `col` - The column of the element.
/// * `grid` - The grid (consisting of vector of vectors).
/// * `is_solid` - The predicate function.
///
/// ### Returns
///
/// A vector of neighbors (index tuples).
///
/// ### Example
///
/// ```
/// use pathfinding::get_neighbors;
///
/// let grid = vec![
///    vec![1, 1, 1, 1, 1],
///    vec![1, 0, 0, 0, 1],
///    vec![1, 0, 0, 0, 1],
///    vec![1, 0, 0, 0, 1],
///    vec![1, 1, 1, 1, 1],
/// ];
///
/// let neighbors = get_neighbors(2, 2, &grid, |row, col, grid| {
///    grid[row][col] == 1
/// });
///
/// println!("{:?}", neighbors);
///
/// assert_eq!(neighbors, vec![(1, 2), (2, 1), (3, 2), (2, 3)]);
/// assert_eq!(neighbors.len(), 4);
/// ```
pub fn get_neighbors(
    row: i32,
    col: i32,
    grid: &Vec<Vec<i32>>,
    is_solid: fn(usize, usize, &Vec<Vec<i32>>) -> bool,
) -> Vec<(i32, i32)> {
    let mut neighbors = vec![];
    if row > 0 && !is_solid(row as usize - 1, col as usize, grid) {
        neighbors.push((row - 1, col));
    }
    if col > 0 && !is_solid(row as usize, col as usize - 1, grid) {
        neighbors.push((row, col - 1));
    }
    if row < grid.len() as i32 - 1 && !is_solid(row as usize + 1, col as usize, grid) {
        neighbors.push((row + 1, col));
    }
    if col < grid[0].len() as i32 - 1 && !is_solid(row as usize, col as usize + 1, grid) {
        neighbors.push((row, col + 1));
    }
    neighbors
}

/// A* - algorithm for finding the shortest path in an 2D grid array.
/// It uses a heuristic function to estimate the distance to the end.
///
/// ### Arguments
///
/// * `start` - The start position.
/// * `end` - The end position.
/// * `grid` - The grid (consisting of vector of vectors).
/// * `heuristic` - The heuristic function.
/// * `is_cell_solid` - The predicate function to check if a node is solid or not.
///
/// ### Returns
///
/// A vector of nodes from start to end. Same as the `reconstruct_path` function.
///
/// ### Example
///
/// ```
/// use pathfinding::astar;
/// use pathfinding::manhattan_distance;
///
/// let grid = vec![
///     vec![1, 1, 1, 1, 1],
///     vec![1, 0, 1, 0, 1],
///     vec![1, 0, 1, 0, 1],
///     vec![1, 0, 0, 0, 1],
///     vec![1, 1, 1, 1, 1],
/// ];
/// 
/// let path = astar(
///     (1, 1),
///     (1, 3),
///     &grid,
///     manhattan_distance,
///     |row, col, grid| grid[row][col] == 1,
/// );
/// 
/// println!("{:?}", path);
/// 
/// let expected_path = Some(vec![(1, 1), (2, 1), (3, 1), (3, 2), (3, 3), (2, 3), (1, 3)]);
/// assert_eq!(path, expected_path);
/// ```
pub fn astar(
    start: (i32, i32),
    end: (i32, i32),
    grid: &Vec<Vec<i32>>,
    heuristic: fn((i32, i32), (i32, i32)) -> i32,
    is_cell_solid: fn(usize, usize, &Vec<Vec<i32>>) -> bool,
) -> Option<Vec<(i32, i32)>> {
    let mut closed_set = HashSet::new();
    let mut open_set = HashSet::new();
    open_set.insert(start);

    let mut came_from = HashMap::<(i32, i32), (i32, i32)>::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, heuristic(start, end));

    let mut open_set_heap = BinaryHeap::new();
    open_set_heap.push((heuristic(start, end), start));

    while !open_set_heap.is_empty() {
        let current: (i32, i32) = open_set_heap.pop().unwrap().1;
        if current == end {
            return Some(reconstruct_path(&came_from, current));
        }
        open_set.remove(&current);
        closed_set.insert(current);

        for neighbor in get_neighbors(current.0, current.1, grid, is_cell_solid) {
            if closed_set.contains(&neighbor) {
                continue;
            }

            let tentative_g_score = g_score[&current] + 1;

            if !open_set.contains(&neighbor) {
                open_set.insert(neighbor);
                open_set_heap.push((heuristic(neighbor, end), neighbor));
            } else if tentative_g_score >= g_score[&neighbor] {
                continue;
            }

            came_from.insert(neighbor, current);
            g_score.insert(neighbor, tentative_g_score);
            f_score.insert(neighbor, tentative_g_score + heuristic(neighbor, end));
        }
    }
    None
}

/// The manhattan distance is the sum of the absolute differences of their Cartesian coordinates.
/// In a right triangle, the manhattan distance is equal to the sum of the lengths of the legs.
/// 
/// ### Arguments
/// 
/// * `a` - The first position.
/// * `b` - The second position.
/// 
/// ### Returns
/// 
/// An integer representing the distance between the two positions.
/// 
/// ### Example
/// 
/// ```
/// use pathfinding::manhattan_distance;
/// 
/// let distance = manhattan_distance((1, 1), (1, 3));
/// 
/// assert_eq!(distance, 2);
/// ```
#[allow(dead_code)]
pub fn manhattan_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

/// Diagonal distance is the maximum of the absolute differences of their Cartesian coordinates.
/// In a right triangle, the diagonal distance is equal to the length of the hypotenuse.
/// 
/// ### Arguments
/// 
/// * `a` - The first position.
/// * `b` - The second position.
///
/// ### Example
/// 
/// Simple diagonal distance from (0, 0) to (3, 4) 
/// 
/// ```
/// use pathfinding::diagonal_distance;
/// 
/// let distance = diagonal_distance((0, 0), (3, 4));
/// 
/// assert_eq!(distance, 4);
/// ```
/// 
/// But if we have a grid with a diagonal movement cost of 2, then the diagonal distance is 8.
/// 
/// ```
/// use pathfinding::diagonal_distance;
/// 
/// let distance = diagonal_distance((0, 0), (3, 4)) * 2;
/// 
/// assert_eq!(distance, 8);
/// ```
#[allow(dead_code)]
pub fn diagonal_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs().max((a.1 - b.1).abs())
}
