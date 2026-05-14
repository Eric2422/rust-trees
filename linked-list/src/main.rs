use std::env;

mod linked_list;

fn main() {
    let mut list: linked_list::LinkedList<String> = linked_list::LinkedList::new();

    let args: Vec<String> = env::args().collect();
    if args.len() > 0 {
        for arg in args {
            list.add_to_head(arg);
        }
    }
}
