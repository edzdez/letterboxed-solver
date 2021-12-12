mod trie;

use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use trie::Trie;

pub struct LetterBox {
    top: Vec<char>,
    bottom: Vec<char>,
    left: Vec<char>,
    right: Vec<char>,
    letters: HashSet<char>,
}

fn read_wordlist(file: String) -> Trie {
    let contents = fs::read_to_string(&file)
        .expect(&format!("Could not read file: {}", file));

    let mut trie = Trie::new();
    for word in contents.lines() {
        if word.len() >= 3 {
            trie.insert(String::from(word));
        }
    }

    trie
}

fn read_letterbox(file: String) -> LetterBox {
    let contents = fs::read_to_string(&file)
        .expect(&format!("Could not read file: {}", file));
    let letters = contents
        .chars()
        .filter(|c| *c == '\n')
        .collect();
    let mut contents = contents.lines();

    LetterBox {
        top: contents
            .next()
            .unwrap()
            .chars()
            .collect(),
        bottom: contents
            .next()
            .unwrap()
            .chars()
            .collect(),
        left: contents
            .next()
            .unwrap()
            .chars()
            .collect(),
        right: contents
            .next()
            .unwrap()
            .chars()
            .collect(),
        letters,
    }
}

fn generate_graph(valid_words: &Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();

    for word in valid_words {
        let node = graph.entry(word.clone()).or_insert(HashSet::new());
        for new_word in valid_words {
            if word != new_word && word.ends_with(new_word.chars().next().unwrap()) {
                node.insert(new_word.clone());
            }
        }
    }

    graph
}

fn generate_paths(adj: &HashMap<String, HashSet<String>>, starting_word: &String, i: usize, max_i: usize) -> Vec<Vec<String>> {
    assert!(max_i >= 1);
    if i == max_i {
        return vec![vec![starting_word.clone()]];
    }

    let mut paths = Vec::new();

    for word in adj.get(starting_word).unwrap() {
        // paths.push(vec![starting_word.clone()]);
        for new_path in generate_paths(adj, word, i + 1, max_i).iter_mut() {
            let mut path = vec![starting_word.clone()];
            path.append(new_path);

            paths.push(path);
        }
    }

    paths
}

fn is_valid_solution(path: &Vec<String>) -> bool {
    let mut set = HashSet::new();
    for word in path {
        for letter in word.chars() {
            set.insert(letter);
        }
    }

    set.len() == 12
}

fn find_solutions(adj: &HashMap<String, HashSet<String>>, words_long: usize) -> Vec<Vec<String>> {
    let mut solutions = Vec::new();

    for starting_word in adj.keys() {
        let paths = generate_paths(&adj, &starting_word, 1, words_long);

        for path in paths {
            if is_valid_solution(&path) {
                solutions.push(path);
            }
        }

    }

    solutions
}

fn write_solutions(sol: &Vec<Vec<String>>, num_solutions: usize) {
    assert!(sol.len() >= num_solutions);

    struct Wrapper(Vec<String>);
    impl Eq for Wrapper {}
    impl PartialEq<Self> for Wrapper {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl PartialOrd<Self> for Wrapper {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            return if self.0.len() < other.0.len() {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            };
        }
    }
    impl Ord for Wrapper {
        fn cmp(&self, other: &Self) -> Ordering {
            return if self.0.len() < other.0.len() {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
    }

    let sol: Vec<Wrapper> = sol
        .iter()
        .map(|solution| -> Wrapper {
            Wrapper(
                solution.clone()
            )
        })
        .collect();

    let sol = BTreeSet::from_iter(sol.iter());

    let best_solutions: Vec<String> = sol
        .iter()
        .take(num_solutions)
        .map(|solution| -> String{
            solution.0.join(" ")
        })
        .collect();

    let solution_string = best_solutions.join("\n");

    match fs::write("solutions.txt", solution_string) {
        Ok(()) => (),
        Err(_) => panic!("Failed to write solutions to solutions.txt"),
    }
}

fn main() {
    println!("Loading trie...");
    let start = std::time::Instant::now();
    let trie = read_wordlist(String::from("wordlist.txt"));
    let end = std::time::Instant::now();
    println!("Loaded trie in {}ms", (end - start).as_millis());

    println!("Loading letterbox...");
    let start = std::time::Instant::now();
    let letterbox = read_letterbox(String::from("letterbox.in"));
    let end = std::time::Instant::now();
    println!("Loaded letterbox in {}ms", (end - start).as_millis());

    println!("Finding valid words...");
    let start = std::time::Instant::now();
    let valid_words = trie.find_valid_words(&letterbox);
    let end = std::time::Instant::now();
    println!("Found {} valid words in {}ms", valid_words.len(), (end - start).as_millis());

    println!("Creating directed graph...");
    let start = std::time::Instant::now();
    let adj = generate_graph(&valid_words);
    let end = std::time::Instant::now();
    println!("Built directed graph in {}ms", (end - start).as_millis());

    println!("Finding 2 word solutions...");
    let start = std::time::Instant::now();
    let mut sol = find_solutions(&adj, 2);
    let end = std::time::Instant::now();
    println!("Found {} 2 word solutions in {}ms", sol.len(), (end - start).as_millis());

    if sol.len() < 50 {
        println!("Finding 3 word solutions...");
        let start = std::time::Instant::now();
        let mut three_word_sols = find_solutions(&adj, 3);
        let end = std::time::Instant::now();
        println!("Found {} 3 word solutions in {}ms", three_word_sols.len(), (end - start).as_millis());

        sol.append(&mut three_word_sols);
    }

    println!("Writing solutions to solutions.txt...");
    write_solutions(&sol, sol.len());
}
