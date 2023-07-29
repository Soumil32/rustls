use clap::Parser;
use humansize::{self, DECIMAL};
use std::{
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

    #[arg(short = 's', long="size")]
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
    let mut amount_of_properties = 1;
    if args.show_size {
        amount_of_properties += 1;
    }

    for item in dir_contents {
        let item = item.unwrap();
        let item_name = item.file_name().into_string().unwrap();
        let mut info = Vec::from([item_name]); 
        if args.show_size {
            let metadata = item.metadata().unwrap();
            let size: u64 = metadata.len();
            info.push(format!("{}", humansize::format_size(size, DECIMAL)));
        }
        contents.push(info);
    }
    
    let mut longest_names: Vec<usize> = Vec::new();
    for i in 0..amount_of_properties {
        longest_names.push(contents.iter().map(|item| item[i].len()).max().unwrap());
    }
    
    for item in contents {
        for (property, longest) in item.iter().zip(longest_names.iter()) {
            print!("| {:longest$} ", property,);
        }
        println!("|");
    }
}
