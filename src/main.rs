use clap::Parser;
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

    let directory = match args.directory {
        Some(value) => value,
        None => String::from("."), // The current directory by default but can be changed
    };

    let dir_contents = fs::read_dir(directory).unwrap_or_else(|err| {
        println!("{}", err);
        exit(1)
    });

    let mut contents = Vec::new();

    for item in dir_contents {
        let item = item.unwrap();
        let item_name = item.file_name().into_string().unwrap();
        let mut info = HashMap::from([("name", item_name)]);
        if args.show_size {
            let metadata = item.metadata().unwrap();
            let size: u64 = metadata.len();
            info.insert("size", format!("{}", humansize::format_size(size, DECIMAL)));
        }
        contents.push(info);
    }

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
    println!("|\n|{:=<width$}|", "", width = total_width-1);
    for item in contents {
        let mut item = item.iter().collect::<Vec<_>>();
        item.sort();
        for (key, value) in item {
            print!("| {:longest$} ", value, longest = longest_names[key]);
        }
        println!("|");
    }
}
