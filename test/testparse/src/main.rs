#[derive(Debug, Clone)]
enum Token {
    Number(i32),
    Plus,
    Multiply,
    LeftParenthesis,
    RightParenthesis,
    EndOfInput,
}

struct Parser<'a> {
    tokens: Vec<Token>,
    current_token: Option<Token>,
    index: usize,
    input: &'a str,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        let tokens = Parser::tokenize(input);
        Parser {
            tokens: tokens,
            current_token: None,
            index: 0,
            input: input,
        }
    }

    fn tokenize(input: &'a str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut iter = input.chars().peekable();

        while let Some(c) = iter.next() {
            match c {
                '0'..='9' => {
                    let mut number = c.to_digit(10).unwrap() as i32;
                    while let Some(&next_char) = iter.peek() {
                        if let '0'..='9' = next_char {
                            let digit = iter.next().unwrap().to_digit(10).unwrap() as i32;
                            number = number * 10 + digit;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number));
                }
                '+' => tokens.push(Token::Plus),
                '*' => tokens.push(Token::Multiply),
                '(' => tokens.push(Token::LeftParenthesis),
                ')' => tokens.push(Token::RightParenthesis),
                _ => {} // Ignore other characters
            }
        }

        tokens.push(Token::EndOfInput);
        tokens
    }

    fn parse(&mut self) -> Result<i32, &'static str> {
        self.current_token = Some(self.tokens[self.index].clone());

        let result = self.parse_expression()?;

        match self.current_token.clone() {
            Some(Token::EndOfInput) => Ok(result),
            _ => Err("Invalid input"),
        }
    }

    fn parse_expression(&mut self) -> Result<i32, &'static str> {
        let mut result = self.parse_term()?;

        loop {
            match self.current_token.clone() {
                Some(Token::Plus) => {
                    let token = self.consume_token();
                    result += self.parse_term()?;
                    self.current_token = Some(token.clone());
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn parse_term(&mut self) -> Result<i32, &'static str> {
        let mut result = self.parse_factor()?;

        loop {
            match self.current_token.clone() {
                Some(Token::Multiply) => {
                    let token = self.consume_token();
                    result *= self.parse_factor()?;
                    self.current_token = Some(token.clone());
                }
                _ => break,
            }
        }

        Ok(result)
    }

    fn parse_factor(&mut self) -> Result<i32, &'static str> {
        match self.current_token.clone() {
            Some(Token::Number(value)) => {
                let token = self.consume_token();
                self.current_token = Some(token);
                Ok(value)
            }
            Some(Token::LeftParenthesis) => {
                let token = self.consume_token();
                let result = self.parse_expression()?;
                if let Some(Token::RightParenthesis) = self.current_token.clone() {
                    self.current_token = Some(self.consume_token());
                    Ok(result)
                } else {
                    Err("Missing closing parenthesis")
                }
            }
            _ => Err("Invalid factor"),
        }
    }

    fn consume_token(&mut self) -> Token {
        self.index += 1;
        self.tokens[self.index].clone()
    }
}

fn main() {
    let input = "2 + 3 * (4 + 5)";
    let mut parser = Parser::new(input);

    match parser.parse() {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
