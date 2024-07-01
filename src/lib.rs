use std::fs::{self, OpenOptions, File};
use std::io::{Write, BufRead, BufReader};
use std::error::Error;
use std::process;
use colored::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Config::order(&config)?;
    title()?;
    read_line_from_todo()?;
    
    Ok(()) 
}
pub struct Config {
    pub command: String,
    pub todo: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 && args[0] != String::from("stop") {
            return Err("not enough arguments");
        }

        let command = args[0].clone();
        let todo: String = args[1..].join(" ");

        Ok(Config { command, todo })
    }


    pub fn order(config: &Config) -> Result<(), Box<dyn Error>> {
        let file = "todo.txt";
        let todo = config.todo.clone();
        
        if config.command == "add" {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file)?;
            writeln!(file, "{}", todo)?;
        } else if config.command == "delete" {
            let contents = fs::read_to_string(file)?;
            let line_number: usize = search_case_insensitive(&config.todo, &contents);
            delete_line("todo.txt", line_number)?;
        } else if config.command == "stop" {
            process::exit(1);
        }  else {
            process::exit(1);
        }

        Ok(())
    }
}


pub fn title() -> Result<(), Box<dyn Error>> {
    let title = "

    Todo list
    
    ";
    println!("{}", title.bold().blue());
    
    Ok(())
}

pub fn read_line_from_todo() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("todo.txt")?;

    for (index, line) in contents.lines().into_iter().enumerate() {
        let index: i32 = index.to_string().trim().parse().expect("s");
        let index = (index+1).to_string();
        println!("  {}{}{}: {}", "[".magenta().bold(), index.cyan(), "]".magenta().bold(), line);
    }

    Ok(())
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn search_case_insensitive<'a>(
    query: &String,
    contents: &'a str,
) -> usize {
    let query = query.to_lowercase();
    let mut results: usize = 255;

    for (index, line) in contents.lines().into_iter().enumerate() {
        if line.to_lowercase().contains(&query) {
            results = index;
            break;
        }
    }

    results
}


pub fn delete_line(file_path: &str, line_number: usize) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            if index != line_number {
                line.ok()
            } else {
                None
            }
        })
        .collect();

    let mut file = File::create(file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}