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

    #[arg(short = 's', long = "size")]
    show_size: bool,
}

fn main() {
    let args = Args::parse();

    let mut contents = Vec::new();

    search_directory(args, &mut contents);

    let longest_names = get_longest_string_lengths(&contents);

    let mut titles: Vec<_> = Vec::from_iter(longest_names.iter());
    titles.sort();

    let mut total_width = 0;
    for (key, value) in titles {
        print!("| {:value$} ", to_title(key));
        total_width += value + 3;
    }
    println!("|\n{:=<width$}", "", width = total_width + 1);

    for item in contents {
        let mut item = item.iter().collect::<Vec<_>>();
        item.sort_by_key(|(key, _)| *key);
        for (key, value) in item {
            print!("| {:longest$} ", value, longest = longest_names[key]);
        }
        println!("|");
    }
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

fn get_longest_string_lengths<'a>(
    contents: &Vec<HashMap<&'a str, ColoredString>>,
) -> HashMap<&'a str, usize> {
    let mut longest_names: HashMap<&str, usize> = HashMap::new(); // Property name -> longest length
    let properties = contents[0].keys();
    for property in properties {
        longest_names.insert(
            property,
            contents
                .iter()
                .map(|item| item[property].len())
                .max()
                .unwrap(),
        );
    }
    longest_names
}

fn search_directory(args: Args, contents: &mut Vec<HashMap<&str, ColoredString>>) {
    let directory = match args.directory {
        Some(value) => value,
        None => String::from("."), // The current directory if no directory is specified
    };
    let dir_contents = fs::read_dir(directory).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1)
    });

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

        info.insert("name", item_name);

        if args.show_size {
            let size: u64 = metadata.len();
            let mut size = humansize::format_size(size, DECIMAL).color(colour);
            if colour == DIR_COLOUR {
                size = size.bold();
            }
            info.insert("size", size);
        }
        contents.push(info);
    }
}
