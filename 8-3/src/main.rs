use std::{collections::HashMap, io::{self, Write}};

/*
Using a hash map and vectors,
create a text interface to allow a user
to add employee names to a department in a company.
For example,
“Add Sally to Engineering” or
“Add Amir to Sales.”
Then let the user retrieve a

list of all people in a department

or

all people in the company by department, sorted alphabetically.
*/

/*** User inputs ***
Description: Adds an employee to a deparment
1 {Name} {Department}

Description: Lists all people in a department
2 {Department}

Description: Lists all people in the company by department sorted alphabetically (sorted by dep, then name)
3
*/
fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        print!("Action: ");
        io::stdout().flush().expect("Flush failed!");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut words = input.trim().split(" ");

        let action_num_word: &str = match words.next() {
            Some(first_word) => first_word,
            None => {
                println!("Can't be an empty string!");
                continue;
            }
        };

        let action_num: u8 = match action_num_word.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("First word has to be action number!");
                continue;
            }
        };

        if action_num == 1 {
            let name = match words.next() {
                Some(second_word) => second_word,
                None => {
                    println!("Wrong format!");
                    continue;
                }
            };
            let department = match words.next() {
                Some(third_word) => third_word,
                None => {
                    println!("Wrong format!");
                    continue;
                }
            };
            departments.entry(department.to_string())
                .and_modify(|employees| { employees.push(name.to_string()) })
                .or_insert(vec![name.to_string()]);
            
        } else if action_num == 2 {
            let department = match words.next() {
                Some(second_word) => second_word,
                None => {
                    println!("Wrong format!");
                    continue;
                }
            };
            
            let employees = match departments.get(department) {
                Some(employees) => employees,
                None => {
                    println!("No such department!");
                    continue;
                }
            };

            let employees_string = employees.join(", ");
            println!("Employees in department {department}: {employees_string}");

        } else if action_num == 3 {
            let mut departments_keys: Vec<&String> = departments.keys().collect();
            departments_keys.sort_unstable();

            for key in departments_keys {
                // cloned because previous reference, otherwise could sort original vector.
                let mut employees = departments.get(key).expect("Key does not exist.").clone();
                employees.sort_unstable();
                let employees_string = employees.join(", ");
                println!("Employees in department {key}: {employees_string}");
            }
        } else {
            println!("Please submit a valid action!");
            continue;
        }
    }
}
