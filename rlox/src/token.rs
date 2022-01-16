use token_type::Token_Type;

struct Token {
    tokentype: Token_Type,
    lexeme: &String,
    literal: (),
    line: u8
}

trait TokenTrait {
    fn create(&mut self, tokentype: Token_Type, lexeme: &String, literal: (), line: u8);
    fn to_string(&self) -> &String;
}