use super::token_type::TokenType;

pub struct Token {
    tokentype: TokenType,
    lexeme: String,
    literal: (),
    line: u8
}

pub trait TokenTrait {
    fn get_string(&self) -> String;
}

impl TokenTrait for Token {
    fn get_string(&self) -> String {
        return String::from("{self.tokentype} {self.lexeme} {self.literal}");
    }
}
