use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct TreeType
{
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate(output_dir: String) -> io::Result<()>
{
    define_ast(
        &output_dir,
        &"Expr".to_string(),
        &["error", "tokens", "object"],
        &[
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal  : Option<Object> value".to_string(),
            "Unary    : Token operator, Box<Expr> right".to_string(),
        ],
    )?;

    define_ast(
        &output_dir,
        &"Stmt".to_string(),
        &["error", "expr"],
        &[
            "Expression : Expr expression".to_string(),
            "Print      : Expr expression".to_string(),
        ],
    )?;

    Ok(())
}

fn define_ast(
    output_dir: &String,
    base_name: &String,
    imports: &[&str],
    types: &[String],
) -> io::Result<()>
{
    let path = PathBuf::from(output_dir).join(format!("{}.rs", base_name.to_lowercase()));
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    // Write imports to file
    for import in imports
    {
        writeln!(file, "use crate::{import}::*;")?;
    }

    for ttype in types
    {
        let (base_class_name, args) = ttype.split_once(':').unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let arg_split = args.split(',');
        let mut fields = Vec::new();

        for arg in arg_split
        {
            let (t2type, name) = arg.trim().split_once(' ').unwrap();
            fields.push(format!("{}: {}", name.trim(), t2type.trim()));
        }

        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    writeln!(file, "\npub enum {base_name} {{\n")?;
    for t in &tree_types
    {
        writeln!(file, "    {}({}),", t.base_class_name, t.class_name)?;
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "impl {base_name} {{\n")?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, {}_visitor: &dyn {}Visitor<T>) -> Result<T, LoxError>{{",
        base_name.to_lowercase(),
        base_name,
    )?;
    writeln!(file, "        match self {{")?;

    for t in &tree_types
    {
        writeln!(
            file,
            "            {}::{}(v) => v.accept({}_visitor),",
            base_name,
            t.base_class_name,
            base_name.to_lowercase()
        )?;
    }

    writeln!(file, "        }}\n    }}\n}}\n")?;

    for t in &tree_types
    {
        writeln!(file, "pub struct {} {{", t.class_name)?;
        for f in &t.fields
        {
            writeln!(file, "    pub {f},")?;
        }

        writeln!(file, "}}\n")?;
    }

    writeln!(file, "pub trait {base_name}Visitor<T> {{")?;
    for t in &tree_types
    {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, LoxError>;",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name,
        )?;
    }
    writeln!(file, "}}\n")?;

    for t in &tree_types
    {
        writeln!(file, "impl {} {{", t.class_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {base_name}Visitor<T>) -> Result<T, \
             LoxError>{{"
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}\n}}\n")?;
    }
    Ok(())
}
