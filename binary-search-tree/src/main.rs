mod binary_search_tree;

use std::io::Write;

use binary_search_tree::BinarySearchTree;
use binary_search_tree::BinarySearchTreeType;

fn main() {
    // Check for valid number of arguments.
    let args: Vec<String> = std::env::args().collect();
    if std::env::args().len() < 2 {
        panic!("用法：{} <string、int/integer>", args[0]);
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
            panic!("\"{}\"不是有效的类型。", args[1]);
        }
    }

    loop {
        print!("输入想插入的元素：");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("输入值无效。");
        input = input.trim_end().to_string();
        println!();

        match tree {
            BinarySearchTreeType::Integer(ref mut integer_tree) => integer_tree.add(
                input
                    .parse::<i32>()
                    .expect(format!("\"{input}\"不是整数。\n").as_str()),
            ),
            BinarySearchTreeType::String(ref mut string_tree) => {
                string_tree.add(input);
            }
        }

        println!("树大小：{}", tree.size());
        println!("树：\n{}", tree.to_string());
    }
}
