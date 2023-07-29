use clap::Parser;
use colored::*;
use humansize::{self, DECIMAL};
use std::{
    collections::HashMap,
    fs::{self},
    process::exit,
};

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

    let mut titles: Vec<_> = longest_names.iter().collect();
    titles.sort();
    let mut total_width = 0;
    for (key, value) in titles {
        print!("| {:longest$} ", key, longest = value);
        total_width += value + 3;
    }
    println!("|\n{:=<width$}", "", width = total_width+1);
    for item in contents {
        let mut item = item.iter().collect::<Vec<_>>();
        item.sort_by_key(|(key, _)| *key);
        for (key, value) in item {
            print!("| {:longest$} ", value, longest = longest_names[key]);
        }
        println!("|");
    }
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

        const DIR_COLOUR: &str = "blue";

        let colour = if metadata.is_dir() { DIR_COLOUR } else { "green" };

        let mut info = HashMap::new();

        let item_name = item.file_name().into_string().unwrap();
        let item_name = match colour {
            "blue" => item_name.blue(),
            "green" => item_name.green(),
            _ => item_name.normal(),
        };
        info.insert("name", item_name);
       
        if args.show_size {
            let size: u64 = metadata.len();
            let size =  match colour {
                "blue" => format!("{}", humansize::format_size(size, DECIMAL)).blue(),
                "green" => format!("{}", humansize::format_size(size, DECIMAL)).green(),
                _ => format!("{}", humansize::format_size(size, DECIMAL)).normal(),
            };
            info.insert("size", size);
        }
        contents.push(info);
    }
}
