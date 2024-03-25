#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use std::env;
    use std::fs;
    #[test]
    fn test_lexer() {
        let args: Vec<String> = env::args().collect();
        if args.len() == 0 {
            panic!("code file not found");
        }
        let filename = &args[args.len() - 1];

        let code =
            fs::read_to_string(filename).expect("Something went wrong when reading the file");

        let mut lexer = Lexer::new(&code);
        lexer.print_all_token();
    }
}
