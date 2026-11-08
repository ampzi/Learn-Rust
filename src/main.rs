use std::io::{BufRead, Write, stdin};

struct Todo {
    id: u32,
    name: String,
    complated: bool,
}

fn get_input() -> String {
    let mut cmd = String::new();
    println!("Please enter command ( /add /complete /list /save /edit /finish /delete ) >");
    stdin().read_line(&mut cmd).expect("Failed input");
    cmd
}

fn add(lists: &mut Vec<Todo>) {
    let mut name = String::new();
    println!("Please enter name of todo >");
    stdin().read_line(&mut name).expect("Failed input");
    let id = lists.len() as u32;
    lists.push(Todo {
        id,
        name: name.trim().to_string(),
        complated: false,
    });
}

fn finish(lists: &Vec<Todo>) {
    save(lists);
    println!("Todos saved to todo.txt");
}

fn save(lists: &Vec<Todo>) {
    write(lists);
    println!("Todos saved to todo.txt");
}

fn edit(lists: &mut Vec<Todo>) {
    let mut s = String::new();
    println!("Please enter id of todo to edit >");
    stdin().read_line(&mut s).expect("Failed input");
    let id: u32 = s.trim().parse().unwrap_or(u32::MAX);
    for todo in lists.iter_mut() {
        if todo.id == id {
            let mut name = String::new();
            println!("Please enter new name of todo >");
            stdin().read_line(&mut name).expect("Failed input");
            todo.name = name.trim().to_string();
            println!("Todo with ID {} updated", id);
            return;
        }
    }
    println!("Todo with ID {} not found", id);

}

fn delete(lists: &mut Vec<Todo>) {
    let mut s = String::new();
    println!("Please enter id of todo to delete >");
    stdin().read_line(&mut s).expect("Failed input");
    let id: u32 = s.trim().parse().unwrap_or(u32::MAX);
    for i in 0..lists.len() {
        if lists[i].id == id {
            lists.remove(i);
            println!("Todo with ID {} deleted", id);
            return;
        }
    }
    println!("Todo with ID {} not found", id);
}

fn complete(lists: &mut Vec<Todo>) {
    let id: u32 = 0;
    println!("Please enter id of todo to complete >");
    stdin()
        .read_line(&mut id.to_string())
        .expect("Failed input");
    for todo in lists.iter_mut() {
        if todo.id == id {
            todo.complated = true;

            println!("Todo {} completed", todo.name);
            return;
        }
    }
    println!("Todo with ID {} not found", id);
}

fn list(lists: &Vec<Todo>) {
    for todo in lists.iter() {
        println!(
            "{} - {} - {}",
            todo.id,
            todo.name,
            if todo.complated {
                "completed"
            } else {
                "not completed"
            }
        );
    }
}

fn write(lists: &Vec<Todo>) {
    let mut file = std::fs::File::create("todo.txt").expect("Failed to create file");
    for todo in lists.iter() {
        let line = format!(
            "{} - {}\n",
            todo.name,
            if todo.complated {
                "completed"
            } else {
                "not completed"
            }
        );
        file.write_all(line.as_bytes())
            .expect("Failed to write to file");
    }
}

fn read() -> Vec<Todo> {
    let mut lists = Vec::new();
    let file = std::fs::File::open("todo.txt");
    if let Ok(file) = file {
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split(" - ").collect();
                if parts.len() == 2 {
                    let name = parts[0].to_string();
                    let complated = parts[1] == "completed";
                    lists.push(Todo {
                        id: lists.len() as u32,
                        name,
                        complated,
                    });
                }
            }
        }
    }
    lists
}

fn main() {
    let mut lists = read();
    loop {
        let cmd = get_input();
        match cmd.trim() {
            "/add" => add(&mut lists),
            "/complete" => complete(&mut lists),
            "/list" => list(&lists),
            "/edit" => edit(&mut lists),
            "/save" => save(&lists),
            "/finish" => {
                finish(&lists);
                break;
            }
            "/delete" => {
                delete(&mut lists);
            }
            _ => println!("Unknown command"),
        }
    }
}
