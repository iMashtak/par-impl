use par_syntax::ast::{Definition, Program};

pub fn format(program: &Program) -> String {
    let mut result = String::with_capacity(program.r);
    let indent = 0u64;
    format_program(program, &mut result, indent);
    result
}

fn format_program(program: &Program, result: &mut String, indent: u64) {
    let mut first = true;
    for definition in &program.defs {
        if !first {
            result.push('\n');
            first = false;
        }
        // format_definition(definition, result, indent);
    }
}
