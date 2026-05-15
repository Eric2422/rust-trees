mod binary_search_tree;

use std::io::Write;

use binary_search_tree::BinarySearchTree;
use binary_search_tree::BinarySearchTreeType;

fn main() {
    // Check for valid number of arguments.
    let args: Vec<String> = std::env::args().collect();
    if std::env::args().len() < 2 {
        panic!("Usage: {} <string, int/integer>", args[0]);
    }

    let mut tree;
    match args[1].as_str() {
        "string" => {
            tree = BinarySearchTreeType::String(BinarySearchTree::new());
        }
        "int" | "integer" => {
            tree = BinarySearchTreeType::Integer(BinarySearchTree::new());
        }
        &_ => {
            panic!("\"{}\" is not a valid type.", args[1]);
        }
    }

    loop {
        print!("Enter element to insert: ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Invalid input type.");
        input = input.trim_end().to_string();
        println!();

        match tree {
            BinarySearchTreeType::Integer(ref mut integer_tree) => integer_tree.add(
                input
                    .parse::<i32>()
                    .expect(format!("\"{input}\" is not an integer.\n").as_str()),
            ),
            BinarySearchTreeType::String(ref mut string_tree) => {
                string_tree.add(input);
            }
        }

        println!("Tree size: {}", tree.size());
        println!("Tree:\n{}", tree.to_string());
    }
}
