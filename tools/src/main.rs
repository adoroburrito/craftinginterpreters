use std::{env, fs::File, io::Write, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length != 2 {
        println!("Usage: generate_ast <output directory>");
        process::exit(64);
    }

    let output_dir = &args[1];
    println!("{output_dir}");

    define_ast(
        output_dir,
        "Expr",
        Vec::from([
            "Binary     <+> left: Expr, operator: Token, right: Expr",
            "Grouping   <+> expression: Expr",
            "Literal    <+> value: &str",
            "Unary      <+> operator: Token, right: Expr",
        ]),
    )
}

fn define_ast(output_dir: &str, enum_name: &str, types: Vec<&str>) {
    let path = format!("{output_dir}/{enum_name}.rs");
    let mut file = File::create(path).unwrap();
    let mut struct_names: Vec<&str> = Vec::new();

    for t in types.iter() {
        let split: Vec<&str> = t.split("<+>").collect();

        let struct_name = split[0].trim();
        let field_list = split[1].trim();

        struct_names.push(struct_name);

        file.write_all(format!("struct {struct_name} {{\n").as_bytes())
            .unwrap();

        let to_insert = define_type(field_list);
        file.write_all(to_insert.as_bytes()).unwrap();

        file.write_all(format!("}}\n\n").as_bytes()).unwrap();
    }

    file.write_all(format!("enum {enum_name} {{\n").as_bytes())
        .unwrap();
    for s in struct_names.iter() {
        file.write_all(format!("\t{s}({s}),\n").as_bytes()).unwrap();
    }
    file.write_all(format!("}}\n").as_bytes()).unwrap();
}

fn define_type(field_list: &str) -> String {
    let mut final_string = "".to_string();

    let field_list_split: Vec<&str> = field_list.split(", ").collect();

    for field in field_list_split {
        final_string.push_str(&format!("\t{field},\n"))
    }

    final_string
}
