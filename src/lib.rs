extern crate regex;
extern crate termcolor;
use regex::Regex;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use std::{env, fs};
use termcolor::{Color, ColorChoice, NoColor, ColorSpec, StandardStream, WriteColor};

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

fn gen_colors(filename: &str) -> Vec<Color> {
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
    let color = Regex::new(r"rgb.*").unwrap();
    let rgb_num = Regex::new(r"\d+").unwrap();
    for line in String::from_utf8_lossy(&output.stdout).to_string().lines() {
        if let Some(m) = color.find(&line) {
            let mut c_vec = Vec::new();

            for cap in rgb_num.captures_iter(m.as_str()) {
                c_vec.push(*&cap[0].parse::<u8>().unwrap())
            }

            let color = Color::Rgb(c_vec[0], c_vec[1], c_vec[2]);
            results.push(color)
        }
    }

    results
}

fn gen_blocks_line(colors: &Vec<Color>) {
    for color in colors {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout
            .set_color(ColorSpec::new().set_bg(Some(*color)))
            .unwrap();
        write!(&mut stdout, "      ").unwrap();
        stdout
            .set_color(&ColorSpec::new())
            .unwrap();
        write!(&mut stdout, "   ").unwrap();
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let colors = gen_colors(&config.filename);

    println!("Here are your colors:");
        for _ in 0..3 {
            gen_blocks_line(&colors);
            println!("");
        }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {}
}
