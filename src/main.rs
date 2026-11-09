//! A tiny tour of Rust — one lesson per module.
//!
//! cargo run          run every lesson
//! cargo run -- 2     run only lesson 2
//! cargo run -- list  print the table of contents
//! cargo test         run the unit tests that ship with each lesson

mod errors;
mod ownership;
mod traits_iter;
mod types;

type Demo = fn();

fn main() {
    let lessons: [(&str, &str, Demo); 4] = [
        ("1", "ownership, borrowing, lifetimes", ownership::demo),
        ("2", "structs, methods, enums, pattern matching", types::demo),
        ("3", "Option, Result, `?`, custom errors", errors::demo),
        ("4", "traits, generics, iterators, HashMap", traits_iter::demo),
    ];

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.first().map(String::as_str) == Some("list") {
        for (number, title, _) in lessons {
            println!("{number}. {title}");
        }
        return;
    }

    let selected: Vec<&(&str, &str, Demo)> = match args.first() {
        Some(choice) => lessons.iter().filter(|(number, _, _)| *number == choice.as_str()).collect(),
        None => lessons.iter().collect(),
    };

    if selected.is_empty() {
        println!("unknown lesson {:?} — try `cargo run -- list`", args[0]);
        std::process::exit(1);
    }

    for (number, title, demo) in selected {
        println!("\n=== lesson {number}: {title} ===");
        demo();
    }
}
