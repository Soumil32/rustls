use clap::Parser;
use colored::*;
use humansize::{self, DECIMAL};
use std::{collections::HashMap, fs, process::exit};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to list
    #[arg()]
    directory: Option<String>,

    /// Show the size
    #[arg(short = 's', long = "size")]
    show_size: bool,

    /// Show the file types
    #[arg(short = 't', long = "types")]
    show_types: bool,
}

fn main() {
    let args = Args::parse();
    let terminal_size: Vec<_> = termsize::get().iter().map(|size| {(size.cols, size.rows)}).collect();
    
    let (contents, longest_names) = search_directory(args);

    let mut titles: Vec<_> = Vec::from_iter(longest_names.iter());
    titles.sort();

    let mut output = String::new();
    let mut total_width = 0;
    for (key, value) in titles {
        output += &format!("| {:value$} ", to_title(key)).to_string();
        total_width += value + 3;
    }
    let amount_of_columns = terminal_size[0].0 as usize / total_width;
    output += &format!("|\n{:=<width$}", "", width = total_width + 1);

    for item in contents {
        let mut item = item.iter().collect::<Vec<_>>();
        item.sort_by_key(|(key, _)| *key);
        for (key, value) in item {
            output += &format!("| {:longest$} ", value, longest = longest_names[key]);
        }
        output += "|\n";
    }
    println!("{}", output)
}

fn to_title(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn search_directory(args: Args) -> (Vec<HashMap<&'static str, ColoredString>>, HashMap<&'static str, usize>) {
    let directory = match args.directory {
        Some(value) => value,
        None => String::from("."), // The current directory if no directory is specified
    };
    let dir_contents = fs::read_dir(directory).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1)
    });

    let mut contents = Vec::new();
    let mut longest_names: HashMap<&str, usize> = HashMap::new(); // Property name -> longest length

    for item in dir_contents {
        let item = item.unwrap();
        let metadata = item.metadata().unwrap();

        const DIR_COLOUR: Color = Color::Blue;
        const FILE_COLOUR: Color = Color::White;

        let colour = if metadata.is_dir() {
            DIR_COLOUR
        } else {
            FILE_COLOUR
        };

        let mut info = HashMap::new();

        let mut item_name = item.file_name().into_string().unwrap().color(colour);
        if colour == DIR_COLOUR {
            item_name = item_name.bold();
        }
        let current_longest_name = longest_names
            .entry("name")
            .or_insert(item_name.len());
        if item_name.len() > *current_longest_name {
            *current_longest_name = item_name.len();
        }

        info.insert("name", item_name);

        if args.show_size {
            let size: u64 = metadata.len();
            let mut size = humansize::format_size(size, DECIMAL).color(colour);
            if colour == DIR_COLOUR {
                size = size.bold();
            }

            let current_longest_size = longest_names
                .entry("size")
                .or_insert(size.len());
            if size.len() > *current_longest_size {
                *current_longest_size = size.len();
            }

            info.insert("size", size);
        }

        if args.show_types {
            let file_type = match metadata.is_file() {
                true => {
                    let mut mime_type = new_mime_guess::from_path(item.path())
                        .first_raw()
                        .unwrap_or("")
                        .split('/');
                    let main_type = mime_type.nth(0).unwrap_or("");
                    let sub_type = mime_type.nth(0);
                    match sub_type {
                        Some(result) => result.color(colour),
                        None => main_type.color(colour),
                    }
                }
                false => "/".color(colour).bold(),
            };
            let current_longest_type = longest_names
                .entry("type")
                .or_insert(file_type.len());
            if file_type.len() > *current_longest_type {
                *current_longest_type = file_type.len();
            }

            info.insert("type", file_type);
        }
        contents.push(info);
    }
    (contents, longest_names)
}
