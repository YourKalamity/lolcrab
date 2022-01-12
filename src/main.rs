/*
    lolcrab by YourKalamity
    A simple program that randomly colourises your input.
    https://github.com/YourKalamity/lolcrab/

    MIT License

    Copyright (c) 2022 Kalam 

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
 */

use std::env;
use colored::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use rand::Rng;
use atty::Stream;
use std::io;

fn help() -> String{
    return "
    lolcrab by YourKalamity
    Usage:
    lolcrab [FILE]
    Optionally, you can pipe data to lolcrab
    ".to_string();
}

fn random_colour() -> String {
    let colours = ["red", "green", "yellow", "blue", "magenta", "cyan", "white"];
    let mut rng = rand::thread_rng();
    let colour = colours[rng.gen_range(0..colours.len())];
    return colour.to_string();
}

fn random_colour_on_char(character: char) -> colored::ColoredString {
    match random_colour().as_str() {
        "red" => return character.to_string().red(),
        "green" => return character.to_string().green(),
        "yellow" => return character.to_string().yellow(),
        "blue" => return character.to_string().blue(),
        "magenta" => return character.to_string().magenta(),
        "cyan" => return character.to_string().cyan(),
        "white" => return character.to_string().white(),
        &_ => return character.to_string().white(),
    }
}

fn read_file_to_string(path: String) -> String {
    let path = Path::new(&path);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_why) => panic!("couldn't read {}: {}", display, _why),
        Ok(_) => return s
    }
}

fn load_stdin() -> io::Result<String> {
    if atty::is(Stream::Stdin) {
        return Err(io::Error::new(io::ErrorKind::Other, "Nothing piped"));
    }
    else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        return Ok(buffer);
    }

}

fn get_input() -> String{
    match load_stdin() {
        Err(_) => {
            let args: Vec<String> = env::args().collect();
            if args.len() == 2 {
                return read_file_to_string(args[1].clone());
            }
            else {
                return help();
            }
        },
        Ok(input) => return input
    }
}

fn colour_string(input: String) -> String {
    let mut output = String::new();
    for character in input.chars() {
        output.push_str(&random_colour_on_char(character).to_string());
    }
    return output;
}

fn main() {
    print!("{}", colour_string(get_input()));
    return
}