use masm_parser::{MidenProgram, EmptyProgram};

fn main() {
    let pow_program = MidenProgram::parse_from_file("./masm/constants.masm").unwrap();
    pow_program.print_masm();
}
