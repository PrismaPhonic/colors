extern crate regex;
use std::error::Error;
use std::{fs, env};
use std::process::Command;
use regex::Regex;

pub struct Color(i32, i32, i32);

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name!"),
        };

        Ok(Config { filename })
    }
}

fn gen_colors(filename: &str) -> Vec<String> {
    let output = Command::new("magick")
        .arg("convert")
        .arg(filename)
        .arg("-resize")
        .arg("10%")
        .arg("-colors")
        .arg("8")
        .arg("-format")
        .arg("\"%c\"")
        .arg("histogram:info:")
        .output()
        .expect("failed to execute process");

    let mut results = Vec::new();
    let color = Regex::new(r"#.{6}").unwrap();
    for line in String::from_utf8_lossy(&output.stdout).to_string().lines() {
        if let Some(m) = color.find(&line) {
            let mut result = String::new();
            result.push_str(m.as_str());
            results.push(result.to_string())
        }
    }

    results
}

fn gen_blocks() {

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let results = gen_colors(&config.filename);

    println!("Here are your colors:");
    for color in results {
        println!("{}", color);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let config = Config { filename: "Farr-Peter-Headshot.jpg".to_string()  };

        assert_eq!(gen_colors(config), vec!["#1D2C08", "#24580E", "#55403B", "#579047", "#9C6F5C", "#AF9171", "#84A48F", "#C7BFAD"]);
    }
}
