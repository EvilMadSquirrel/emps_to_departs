use std::collections::HashMap;

use std::io;
fn main() {
    println!("To add person to department, type:\n\t \"Add (person) to (department)\"");
    println!("To show persons in department, type department name");
    println!("To show all persons, type \"All\"");
    println!("To exit program, type \"Exit\"");
    let mut organization = HashMap::new();
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Error read from console");

        let command = String::from(command.trim());
        let command = command.split_whitespace();
        let mut words = vec![];
        for word in command {
            words.push(String::from(word));
        }
        let first_command = &words[0];
        if first_command == "Add" {
            let name = String::from(&words[1]);
            let department = String::from(&words[3]);
            let department = organization.entry(department).or_insert(vec![]);
            department.push(name);
        } else if first_command == "All" {
            let keys: Vec<String> = organization.keys().cloned().collect();
            for key in keys {
                let values = organization.get_mut(&key).unwrap();
                values.sort();
                println!("{}:", key);
                for value in values {
                    println!("\t{}", value);
                }
            }
        } else if first_command == "Exit" {
            break;
        } else {
            let values = organization.get_mut(first_command).unwrap();
            values.sort();
            println!("{}:", first_command);
            for value in values {
                println!("\t{}", value);
            }
        }
    }
}
