/// https://www.youtube.com/watch?v=MBpMYTTEvLU&list=PL6whQ6CW68oe3tcmFcBLYFQChqwYEaG7_&index=2

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;


use super::enums::*;

pub fn tokenize(file_path: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut source_reader: BufReader<File> = BufReader::new(File::open(file_path)?);

    let mut line = String::new();
    let mut line_number: usize = 0;

    loop {
        let len =  source_reader.read_line(&mut line)?;
        line_number += 1;
        
        if len == 0 {break};

        let mut chars: std::str::Chars<'_> = line.chars();
        let mut i: i32 = -1; // Char index (Not byte dependent)

        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                // Skip whitespace characters (Do nothing)
            } else if c.is_digit(10) {
                // Parse number literal
                let mut number: Vec<char>= vec![c];
                let mut is_float: bool = false;

                while let Some(n) = chars.next() && !n.is_whitespace() {
                    number.push(n);    
                    if n == '.' {is_float = true};
                }  

                let number: String = number.iter().collect();
                
                if is_float {
                    tokens.push(Token::NumLiteral(Num::Float(f64::from_str(&number[..])?)));
                } else if number.starts_with("0x") { // Hex check
                    tokens.push(Token::NumLiteral(Num::Int(usize::from_str_radix(&number[2..], 16)?)));
                } else {
                    tokens.push(Token::NumLiteral(Num::Int(usize::from_str_radix(&number[..], 10)?)));
                } // in the future we need exponential and negative numbers (Maybe?)
            } 

            i += 1;
        }

        tokens.push(Token::Terminator);
        line.clear();
    };
    
    Ok(tokens)
}
