use crate::structs::AstNode;

static C_HEADER: &'static str = "\
# include <stdio.h>

int main() {
    setbuf(stdout, NULL);
    unsigned char tape[10000] = {0};
    unsigned char *ptr = tape+500;
";
static C_FOOTER: &'static str = "return 0;}";

static C_INCREASE: &'static str = "    ++*ptr;\n";
static C_DECREASE: &'static str = "    --*ptr;\n";
static C_MOVERIGT: &'static str = "    ++ptr;\n";
static C_MOVELEFT: &'static str = "    --ptr;\n";
static C_OUTPUTCH: &'static str = "    putchar(*ptr);\n";
static C_INPUTCHR: &'static str = "    *ptr = getchar();\n";
static C_LOOPSTRT: &'static str = "    while (*ptr) {\n";
static C_LOOPENDS: &'static str = "    }\n";
static C_EXITEXIT: &'static str = "    exit(*ptr);\n";

fn ast_to_c_ins(c_code: &mut String, ast: &Vec<AstNode>) {
    for node in ast {
        match node {
            AstNode::INC => c_code.push_str(&C_INCREASE),
            AstNode::DEC => c_code.push_str(&C_DECREASE),
            AstNode::MRT => c_code.push_str(&C_MOVERIGT),
            AstNode::MLT => c_code.push_str(&C_MOVELEFT),
            AstNode::OUT => c_code.push_str(&C_OUTPUTCH),
            AstNode::INP => c_code.push_str(&C_INPUTCHR),
            AstNode::END => c_code.push_str(&C_EXITEXIT),
            AstNode::LOP(subloop) => {
                c_code.push_str(&C_LOOPSTRT);
                ast_to_c_ins(c_code, subloop);
                c_code.push_str(&C_LOOPENDS);
            }
            _ => (),
        }
    }
}


pub fn compile_ast_c(ast: &Vec<AstNode>) -> String {
    let mut c_code = String::new();
    c_code.push_str(&C_HEADER);

    ast_to_c_ins(&mut c_code, ast);

    c_code.push_str(&C_FOOTER);
    c_code
}