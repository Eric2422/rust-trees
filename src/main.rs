mod binary_search_tree;

use std::io::Write;

use binary_search_tree::BinarySearchTree;
use binary_search_tree::BinarySearchTreeType;
use rand::RngExt;

/// Generate a [`String`] of random characters with a random length of 1 to `max_length` (exclusive).
///
/// # Arguments
///
/// * `max_length` - The maximum length of the [`String`] (exclusive).
///
/// # Return
///
/// A [`String`] of random characters with a random length of 1 to `max_length` (exclusive).
fn random_string(max_length: u32) -> String {
    let string_length = rand::rng().random_range(1..max_length);

    let mut random_string = String::new();

    for _j in 0..string_length {
        random_string.push(rand::rng().sample(rand::distr::Alphanumeric) as char);
    }

    random_string
}

fn main() {
    // 查看有没有足够引数。
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("用法：{} <str/string、int/integer> <随机值数>", args[0]);
    }

    // 第二引数是数的类型。
    let mut tree;
    match args[1].as_str() {
        "int" | "integer" => {
            tree = BinarySearchTreeType::Integer(BinarySearchTree::new());
        }

        "str" | "string" => {
            tree = BinarySearchTreeType::String(BinarySearchTree::new());
        }

        &_ => {
            panic!("\"{}\"不是有效的类型。", args[1]);
        }
    }

    // 如果有第三引数，当为添加几个随机值。
    let num_random = if args.len() >= 3 {
        args[2]
            .parse::<i32>()
            .expect(format!("\"{}\"不是整数。\n", args[2]).as_str())
    } else {
        0
    };

    // 将随机值添加到树。
    match tree {
        BinarySearchTreeType::Integer(ref mut integer_tree) => {
            for _i in 0..num_random {
                loop {
                    if integer_tree.add(rand::random::<i32>()) {
                        break;
                    }
                }
            }
        }

        BinarySearchTreeType::String(ref mut string_tree) => {
            // Generate random strings of 1 to 5 characters.
            for _i in 0..num_random {
                loop {
                    if string_tree.add(random_string(6)) {
                        break;
                    }
                }
            }
        }
    }

    // 反复请用户输入新的值。
    loop {
        println!("树大小：{}", tree.size());
        println!("树：\n{}", tree.to_string());

        print!("输入想插入的元素（如果想退出，即输入回车）：");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("输入值无效。");
        input = input.trim_end().to_string();
        println!();

        if input == "" {
            break;
        }

        match tree {
            BinarySearchTreeType::Integer(ref mut integer_tree) => {
                integer_tree.add(
                    input
                        .parse::<i32>()
                        .expect(format!("\"{input}\"不是整数。\n").as_str()),
                );
            }
            BinarySearchTreeType::String(ref mut string_tree) => {
                string_tree.add(input);
            }
        }
    }
}
