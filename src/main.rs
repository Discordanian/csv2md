extern crate clap;
extern crate csv;
use clap::{App, Arg};
use csv::Error;
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::vec::Vec;

// If the index refers to the first row
fn first_row(i: usize) -> bool {
    i == 0
}

// If the index refers to the first col
fn first_col(i: usize) -> bool {
    first_row(i)
}

// pass in a reference to a mutable HashMap
fn parse_args(cfg: &mut HashMap<String, String>) {
    let matches = App::new("CSV 2 Mark Down")
        .version("1.1")
        .author("Kurt Schwind <Kurt.Schwind@gmail.com>")
        .about("Takes a CSV file and outputs a Obsidian ready markdown table")
        .arg(
            Arg::with_name("infile")
                .short("i")
                .long("infile")
                .value_name("FILE")
                .help("A CSV Input File")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .value_name("FILE")
                .help("A Mark Down Output File to write to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fieldseperator")
                .short("f")
                .long("fs")
                .value_name("Field Seperator")
                .help("[Unsupported] Field Seperator, defaults to comma ','")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Turn on Debug")
                .takes_value(false),
        )
        .get_matches();

    // Gets a value for input if supplied by user, or defaults to blank
    cfg.insert(
        "infile".to_string(),
        matches.value_of("infile").unwrap_or("").to_string(),
    );
    cfg.insert(
        "outfile".to_string(),
        matches.value_of("outfile").unwrap_or("").to_string(),
    );
}

// Print the contents of the configuration
fn print_config(cfg: &HashMap<String, String>) {
    println!("{:?}", cfg);
}

// Read the CSV data from either a file or from STDIN
// Populate data and lengths
fn read_csv(
    infile: &str,
    data: &mut Vec<Vec<String>>,
    lengths: &mut Vec<usize>,
) -> Result<(), Box<Error>> {
    // Read from STDIN if no infile is given
    if infile.is_empty() {
        let mut rdr = csv::Reader::from_reader(io::stdin());
        if let Ok(header) = rdr.headers() {
            let mut vec_row: Vec<String> = Vec::new();
            for r in header {
                vec_row.push(r.to_string());
                lengths.push(r.to_string().chars().count() + 2)
            }
            data.push(vec_row);
        }
        for record in rdr.records() {
            let row = record.unwrap();
            let mut vec_row: Vec<String> = Vec::new();
            for (i, r) in row.iter().enumerate() {
                let value = r.to_string();
                let cur_length = value.chars().count() + 2;
                vec_row.push(value);
                lengths[i] = cmp::max(lengths[i], cur_length);
            }
            data.push(vec_row);
        }
    } else {
        // infile is set so open that and read the CSV from there.
        // println!("Going to attempt to read file [{}]", infile);
        let mut rdr = csv::Reader::from_path(infile)?; // The possible Error
        if let Ok(header) = rdr.headers() {
            let mut vec_row: Vec<String> = Vec::new();
            for r in header {
                vec_row.push(r.to_string());
                lengths.push(r.to_string().chars().count() + 2)
            }
            data.push(vec_row);
        }
        for record in rdr.records() {
            let row = record.unwrap();
            let mut vec_row: Vec<String> = Vec::new();
            for (i, r) in row.iter().enumerate() {
                let value = r.to_string();
                let cur_length = value.chars().count() + 2;
                vec_row.push(value);
                lengths[i] = cmp::max(lengths[i], cur_length);
            }
            data.push(vec_row);
        }
    }
    Ok(()) // The 'good' return
}

// Return a seperator row "|-----|-----|" using the lengths Vector
fn sep_row(lengths: Vec<usize>) -> String {
    let mut retval: String = "|".into();
    for l in lengths {
        let mut i: usize = 0;
        while i < l {
            retval = format!("{}{}", retval, "-");
            i += 1;
        }
        retval = format!("{}{}", retval, "|");
    }
    retval
}

// Take the Vector of Strings and flatten them out to a single string
// join data with newlines
fn vec_to_string(data: Vec<String>) -> String {
    let mut retval: String = "".into();

    for line in data {
        retval = format!("{}{}\n", retval, line);
    }
    retval
}

// Take the data and the lengths and pad accordingly
// Adding a seperator row
fn print_data(data: Vec<Vec<String>>, lengths: Vec<usize>) -> Vec<String> {
    let mut retval: Vec<String> = Vec::new();
    // println!("Data Vec<Vec<String>> String, String");
    // println!("{:?}", data);
    for (row_num, rows) in data.iter().enumerate() {
        let mut line: String = "".into();
        for (i, col) in rows.iter().enumerate() {
            if first_col(i) {
                line = format!(
                    "|{}{:width$}|",
                    line,
                    " ".to_string() + col,
                    width = lengths[i]
                );
            } else {
                line = format!(
                    "{}{:width$}|",
                    line,
                    " ".to_string() + col,
                    width = lengths[i]
                );
            }
        }
        retval.push(line);
        if first_row(row_num) {
            retval.push(sep_row(lengths.to_owned()));
        }
    }
    retval
}

// Dump the counts to STDOUT or to a file if one was given
fn write_contents(outfile: &str, md: String) -> std::io::Result<()> {
    if outfile.is_empty() {
        println!("{}", md);
    } else {
        fs::write(outfile, md)?;
    }
    Ok(())
}

fn main() {
    let mut cfg: HashMap<String, String> = HashMap::new();
    let mut data: Vec<Vec<String>> = Vec::new();
    let mut lengths: Vec<usize> = Vec::new();
    let mut md: String = "INITIALIZE".into();

    parse_args(&mut cfg); // pass function reference to mutable variable
                          // print_config(&cfg);

    // Read the CSV data
    match read_csv(&cfg["infile"], &mut data, &mut lengths) {
        Ok(()) => {
            md = vec_to_string(print_data(data, lengths));
        }
        Err(e) => println!("Error: read_csv {}", e),
    };

    // Write the Mark Down
    match write_contents(&cfg["outfile"], md) {
        Ok(()) => {
            // All good and we did what we wanted
        }
        Err(e) => println!("Error: write_contents {}", e),
    };
}
