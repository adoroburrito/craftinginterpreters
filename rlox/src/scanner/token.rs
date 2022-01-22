use super::token_type::TokenType;

pub struct Token {
    pub tokentype: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: u8,
}

pub trait TokenTrait {
    fn get_string(&self) -> String;
}

impl TokenTrait for Token {
    fn get_string(&self) -> String {
        let tokentype = &self.tokentype;
        let lexeme = &self.lexeme;
        let literal = &self.literal;

        format!(
            "Token type: \"{:#?}\" // Token Lexeme: \"{lexeme}\" // Token Literal: \"{literal}\"",
            tokentype
        )
    }
}
