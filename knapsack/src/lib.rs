use std::collections::HashMap;
use std::io::BufRead;

pub mod knapsack_problem;
pub mod problem_sizes;
pub mod scoring;

use crate::KnapsackProblem;
pub use knapsack_problem::*;
pub use problem_sizes::*;
pub use scoring::*;

//
// 基本の構造体 ＋ 読み込みなど
//

#[derive(Debug, Clone)]
pub struct Solution {
    // (問題名),(買った物),(なんかsolutionのID),(ステップ数)
    pub problem_name: String,
    pub buy: String,
    pub solution_name: String,
    pub time: i32,
}

impl Solution {
    fn cost(&self) -> usize {
        let buy_cost_map: HashMap<char, usize> = [
            ('B', 1000), // Manipulator
            ('F', 300),  // Fast Wheels
            ('L', 700),  // Drill
            ('R', 1200), // Teleport
            ('C', 2000), // Cloning
        ]
        .iter()
        .cloned()
        .collect();

        self.buy
            .chars()
            .map(|c| buy_cost_map.get(&c).unwrap())
            .sum()
    }

    fn cost100d(&self) -> usize {
        self.cost() / 100
    }
}

pub fn read_solutions(input_path: &str) -> Vec<Solution> {
    let file = std::fs::File::open(input_path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut solutions = vec![];

    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        let line = line.trim();
        if line == "" {
            continue;
        }

        let tokens: Vec<_> = line.split(',').collect();
        // dbg!(&tokens);

        solutions.push(Solution {
            problem_name: tokens[0].to_owned(),
            buy: tokens[1].to_owned(),
            solution_name: tokens[2].to_owned(),
            time: tokens[3].parse().unwrap(),
        })
    }

    eprintln!("Solutions: {}", solutions.len());

    solutions
}

pub fn get_solution_sets(solutions: &Vec<Solution>) -> Vec<Vec<Solution>> {
    let mut problem_to_solutions: HashMap<String, Vec<Solution>> = std::collections::HashMap::new();
    for solution in solutions {
        problem_to_solutions
            .entry(solution.problem_name.clone())
            .or_insert(vec![])
            .push(solution.clone());
    }
    eprintln!("Problems: {}", problem_to_solutions.len());

    let mut solution_sets: Vec<_> = problem_to_solutions.values().cloned().collect();
    solution_sets.sort_by_key(|solution_set| solution_set[0].problem_name.clone());
    solution_sets
}

//
// ナップサック問題関連
//

pub fn get_knapsack_problem(
    solution_sets: &Vec<Vec<Solution>>,
    budget: usize,
) -> KnapsackProblem {
    let problem_sizes = get_problem_sizes();

    KnapsackProblem {
        item_sets: solution_sets
            .iter()
            .map(|solutions| {
                let scores = get_scores(solutions, &problem_sizes);
                solutions
                    .iter()
                    .map(|solution| solution.cost100d())
                    .zip(scores)
                    .collect()
            })
            .collect(),
        capacity: budget / 100,
    }
}

pub fn solve(
    solutions: &Vec<Solution>,
    budget: usize,
) -> Vec<Solution> {
    let solution_sets = get_solution_sets(solutions);

    let knapsack_problem = get_knapsack_problem(&solution_sets, budget);
    let selection = solve_knapsack_problem(&knapsack_problem);

    let mut selected_solutions = vec![];
    for i in 0..selection.len() {
        let s = selection[i];
        if s == !0 {
            continue;
        } else {
            selected_solutions.push(solution_sets[i][s].clone());
        }
    }
    eprintln!("Selected: {}", selected_solutions.len());
    selected_solutions
}
