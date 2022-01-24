struct Binary {
	left: Expr,
	operator: Token,
	right: Expr,
}

struct Grouping {
	expression: Expr,
}

struct Literal {
	value: &str,
}

struct Unary {
	operator: Token,
	right: Expr,
}

enum Expr {
	Binary(Binary),
	Grouping(Grouping),
	Literal(Literal),
	Unary(Unary),
}
