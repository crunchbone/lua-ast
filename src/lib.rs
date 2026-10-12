pub mod lexer;
pub mod enums;

#[cfg(test)]
mod tests {
    use super::lexer;

    const TEST_FOLDER: &str = "test_files";

    #[ignore] // Fix after implementation
    #[test]
    fn lexer_tokenize() {
        let file_name = format!("{}/{}", TEST_FOLDER, "tokens.lua");

        assert_eq!(lexer::tokenize(&file_name).unwrap_or(vec![]), ["local", "x", "=", "10", "print(x)"]);
    }
}
