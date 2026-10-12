/// https://www.youtube.com/watch?v=MBpMYTTEvLU&list=PL6whQ6CW68oe3tcmFcBLYFQChqwYEaG7_&index=2

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use super::enums::*;

pub fn tokenize(file_path: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut source_reader: BufReader<File> = BufReader::new(File::open(file_path)?);

    let mut line = String::new();

    loop {
        let len =  source_reader.read_line(&mut line)?;
        if len == 0 {break};

        let mut i = 0;

        while i < len {
            // Identify tokens here
            i += 1;
        }

        tokens.push(Token::Terminator);
        line.clear();
    };
    
    Ok(tokens)
}
