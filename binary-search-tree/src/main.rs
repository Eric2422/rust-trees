mod binary_search_tree;

use std::io::Write;

use binary_search_tree::BinarySearchTree;
use binary_search_tree::BinarySearchTreeType;

fn main() {
    // Check for valid number of arguments.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("用法：{} <string、int/integer> <随机值数>", args[0]);
    }

    let num_random = if args.len() >= 3 {
        args[2]
            .parse::<i32>()
            .expect(format!("\"{}\"不是整数。\n", args[2]).as_str())
    } else {
        0
    };

    let mut tree;
    match args[1].as_str() {
        "int" | "integer" => {
            tree = BinarySearchTreeType::Integer(BinarySearchTree::new());
        }

        "string" => {
            tree = BinarySearchTreeType::String(BinarySearchTree::new());
        }

        &_ => {
            panic!("\"{}\"不是有效的类型。", args[1]);
        }
    }

    match tree {
        BinarySearchTreeType::Integer(ref mut integer_tree) => {
            for _i in 0..num_random {
                integer_tree.add(rand::random::<i32>());
            }
        }

        BinarySearchTreeType::String(ref mut string_tree) => {
            for _i in 0..num_random {
                string_tree.add(rand::random::<char>().to_string());
            }
        }
    }

    loop {
        println!("树大小：{}", tree.size());
        println!("树：\n{}", tree.to_string());

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
            BinarySearchTreeType::String(ref mut string_tree) => string_tree.add(input),
        }
    }
}
