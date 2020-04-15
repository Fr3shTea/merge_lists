use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut args_iter = env::args().skip(1);

    let output_file_arg = match args_iter.next() {
        Some(output_filename) => output_filename,
        None => {
            println!("Output file doesn't specified.");
            println!("Usage: ./merge_lists [output_file] [file1] [file2] [file3] ... [fileN]");

            return;
        }
    };

    let mut merged_list: BTreeSet<String> = BTreeSet::new();

    for filename in args_iter {
        let reader = BufReader::new(match File::open(filename) {
            Ok(file) => file,
            Err(err) => {
                println!("Error occured while reading file: {}. Skipping...", err);

                continue;
            }
        });

        merged_list.extend(reader.lines().map(|line| match line {
            Ok(line) => line.trim().to_string(),
            Err(err) => {
                println!("Error occured while reading line: {}", err);

                String::new()
            }
        }).filter(|line| line.len() > 0));
    }

    let mut output_file_writer = BufWriter::new(match File::create(output_file_arg) {
        Ok(file) => file,
        Err(err) => {
            println!("Error occured: {}", err);

            return;
        }
    });

    for line in merged_list {
        if let Err(err) = writeln!(output_file_writer, "{}", line) {
            println!("Cannot write to file: {}", err);

            break;
        }
    }
}
