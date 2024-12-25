use super::asm_tree::{
    AFunctionDefinitionNode, AInstructionNode, AOperandNode, AProgramNode, ARegisterNode,
    AUnaryOperatorNode,
};

#[tracing::instrument(skip_all)]
pub fn emit_program(a_program: AProgramNode, output: &mut String) {
    let AProgramNode::Program(a_function) = a_program;
    emit_function(a_function, output);
    output.push_str("   .section .note.GNU-stack,\"\",@progbits\n");
}

fn emit_prologue(output: &mut String) {
    output.push_str(&format!("    pushq %rbp\n"));
    output.push_str(&format!("    movq %rsp, %rbp\n"));
}

fn emit_epilogue(output: &mut String) {
    output.push_str(&format!("   movq %rbp, %rsp\n"));
    output.push_str(&format!("   popq %rbp\n"));
}

fn emit_function(a_function: AFunctionDefinitionNode, output: &mut String) {
    let AFunctionDefinitionNode::Function(name, instructions) = a_function;
    output.push_str(&format!("   .globl {name}\n"));
    output.push_str(&format!("{name}:\n"));
    emit_prologue(output);
    for a_instruction in instructions {
        emit_instructions(a_instruction, output);
    }
}

fn emit_instructions(a_instruction: AInstructionNode, output: &mut String) {
    match a_instruction {
        AInstructionNode::Mov(src, dst) => {
            let src = direct_emit_operand(src);
            let dst = direct_emit_operand(dst);
            output.push_str(&format!("   movl    {src}, {dst}"));
        }
        AInstructionNode::Ret => {
            emit_epilogue(output);
            output.push_str("   ret");
        }
        AInstructionNode::Unary(operator, operand) => {
            let operand = direct_emit_operand(operand);
            let operator = direct_emit_operator(operator);
            output.push_str(&format!("   {operator}    {operand}"));
        }
        AInstructionNode::AllocateStack(size) => {
            output.push_str(&format!("  subq    ${size}, %rsp"));
        }
    }
    output.push_str("\n");
}

fn direct_emit_operand(a_operand: AOperandNode) -> String {
    match a_operand {
        AOperandNode::Imm(c) => format!("${c}"),
        AOperandNode::Reg(reg) => match reg {
            ARegisterNode::AX => format!("%eax"),
            ARegisterNode::R10 => format!("%r10d"),
        },
        AOperandNode::Stack(addr) => format!("{addr}(%rbp)"),
        _ => panic!("invalid operand found in emitter stage"),
    }
}

fn direct_emit_operator(a_operator: AUnaryOperatorNode) -> String {
    match a_operator {
        AUnaryOperatorNode::Neg => format!("negl"),
        AUnaryOperatorNode::Not => format!("notl"),
    }
}
