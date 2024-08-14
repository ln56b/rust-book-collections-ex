fn main() {
let median_and_mode = compute_median_and_mode(vec![1, 10, 2, 3, 4, 5, 6, 7, 8, 9]);

println!("Median: {}", median_and_mode.0);
println!("Mode: {}", median_and_mode.1);

let pig_latin_consonant = format_pig_latin("first");
println!("Pig Latin consonant: {}", pig_latin_consonant);
let pig_latin_vowel = format_pig_latin("apple");
println!("Pig Latin vowel: {}", pig_latin_vowel);

let interface = employee_hashmap();
println!("Interface: {:?}", interface);
}

// Return the median and mode of a list of integers
fn compute_median_and_mode(numbers: Vec<i32>) -> (f64, i32) {
    let mut numbers = numbers;
    numbers.sort();
    let len = numbers.len();
    let median = if len % 2 == 0 {
        (numbers[len / 2 - 1] + numbers[len / 2]) as f64 / 2.0
    } else {
        numbers[len / 2] as f64
    };
    let mut mode = 0;
    let mut mode_count = 0;
    let mut current = 0;
    let mut current_count = 0;
    for &number in &numbers {
        if number == current {
            current_count += 1;
        } else {
            current = number;
            current_count = 1;
        }
        if current_count > mode_count {
            mode = current;
            mode_count = current_count;
        }
    }
    (median, mode)
}

// Convert a string so that the first consonant is moved to the end and "ay" is added to the end. If the string starts with a vowel, "hay" is added to the end.
fn format_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();
    if vowels.contains(&first_char) {
        format!("{}hay", word)
    } else {
        format!("{}{}ay", chars.collect::<String>(), first_char)
    }
}

 // Create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
 // Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
use std::collections::HashMap;
use std::io;

fn employee_hashmap() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.len() != 4 || words[0] != "Add" || words[2] != "to" {
            println!("Invalid input");
            continue;
        }
        let name = words[1];
        let department = words[3];
        let department = company.entry(department.to_string()).or_insert(Vec::new());
        department.push(name.to_string());
        department.sort();
    }
    for (department, names) in &company {
        println!("{}: {:?}", department, names);
    }
}