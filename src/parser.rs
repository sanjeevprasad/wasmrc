use crate::modules::InstructionType;
use InstructionType::*;
impl From<u8> for InstructionType {
  fn from(value: u8) -> Self {
    match value {
      0x00 => Unreachable,
      0x01 => Nop,
      0x02 => {
        let BlockType(ty) = parser.parse()?;
        let Expr(body) = parser.parse()?;
        Block { ty, body }
      }
      0x03 => {
        let BlockType(ty) = parser.parse()?;
        let Expr(body) = parser.parse()?;
        Loop { ty, body }
      }
      0x04 => {
        let BlockType(ty) = parser.parse()?;

        let mut then_body = vec![];
        let has_else = loop {
          match parser.input {
            [0x05, ..] => break true,
            [0x0b, ..] => break false,
            _ => then_body.push(parser.parse()?),
          }
        };
        parser.eat(1);

        let else_body = if has_else {
          let Expr(insns) = parser.parse()?;
          insns
        } else {
          vec![]
        };

        If {
          ty,
          then_body,
          else_body,
        }
      }
      0x0c => Br(parser.parse()?),
      0x0d => BrIf(parser.parse()?),
      0x0e => BrTable {
        labels: parser.parse_vec()?.into_vec()?,
        default_label: parser.parse()?,
      },
      0x0f => Return,
      0x10 => Call(parser.parse()?),
      0x11 => {
        let insn = CallIndirect(parser.parse()?);
        parser.parse_flag(0x00, "reserved byte in call_indirect")?;
        insn
      }
      // Parametric instructions
      // https://webassembly.github.io/spec/core/binary/instructions.html#parametric-instructions
      0x1a => Drop,
      0x1b => Select,
      // Variable instructions
      // https://webassembly.github.io/spec/core/binary/instructions.html#variable-instructions
      0x20 => LocalGet(parser.parse()?),
      0x21 => LocalSet(parser.parse()?),
      0x22 => LocalTee(parser.parse()?),
      0x23 => GlobalGet(parser.parse()?),
      0x24 => GlobalSet(parser.parse()?),
      // Memory instructions
      // https://webassembly.github.io/spec/core/binary/instructions.html#memory-instructions
      0x28 => I32Load(parser.parse()?),
      0x29 => I64Load(parser.parse()?),
      0x2a => F32Load(parser.parse()?),
      0x2b => F64Load(parser.parse()?),
      0x2c => I32Load8S(parser.parse()?),
      0x2d => I32Load8U(parser.parse()?),
      0x2e => I32Load16S(parser.parse()?),
      0x2f => I32Load16U(parser.parse()?),
      0x30 => I64Load8S(parser.parse()?),
      0x31 => I64Load8U(parser.parse()?),
      0x32 => I64Load16S(parser.parse()?),
      0x33 => I64Load16U(parser.parse()?),
      0x34 => I64Load32S(parser.parse()?),
      0x35 => I64Load32U(parser.parse()?),
      0x36 => I32Store(parser.parse()?),
      0x37 => I64Store(parser.parse()?),
      0x38 => F32Store(parser.parse()?),
      0x39 => F64Store(parser.parse()?),
      0x3a => I32Store8(parser.parse()?),
      0x3b => I32Store16(parser.parse()?),
      0x3c => I64Store8(parser.parse()?),
      0x3d => I64Store16(parser.parse()?),
      0x3e => I64Store32(parser.parse()?),
      0x3f => {
        parser.parse_flag(0x00, "reserved byte in memory.size")?;
        MemorySize
      }
      0x40 => {
        parser.parse_flag(0x00, "reserved byte in memory.grow")?;
        MemoryGrow
      }
      // Numeric instructions
      // constants
      0x41 => I32Const(parser.parse_int()?),
      0x42 => I64Const(parser.parse_int()?),
      0x43 => F32Const(parser.parse()?),
      0x44 => F64Const(parser.parse()?),
      // test and relation operators
      0x45 => I32Eqz,
      0x46 => I32Eq,
      0x47 => I32Ne,
      0x48 => I32LtS,
      0x49 => I32LtU,
      0x4a => I32GtS,
      0x4b => I32GtU,
      0x4c => I32LeS,
      0x4d => I32LeU,
      0x4e => I32GeS,
      0x4f => I32GeU,
      0x50 => I64Eqz,
      0x51 => I64Eq,
      0x52 => I64Ne,
      0x53 => I64LtS,
      0x54 => I64LtU,
      0x55 => I64GtS,
      0x56 => I64GtU,
      0x57 => I64LeS,
      0x58 => I64LeU,
      0x59 => I64GeS,
      0x5a => I64GeU,
      0x5b => F32Eq,
      0x5c => F32Ne,
      0x5d => F32Lt,
      0x5e => F32Gt,
      0x5f => F32Le,
      0x60 => F32Ge,
      0x61 => F64Eq,
      0x62 => F64Ne,
      0x63 => F64Lt,
      0x64 => F64Gt,
      0x65 => F64Le,
      0x66 => F64Ge,
      // i32 operations
      0x67 => I32Clz,
      0x68 => I32Ctz,
      0x69 => I32Popcnt,
      0x6a => I32Add,
      0x6b => I32Sub,
      0x6c => I32Mul,
      0x6d => I32DivS,
      0x6e => I32DivU,
      0x6f => I32RemS,
      0x70 => I32RemU,
      0x71 => I32And,
      0x72 => I32Or,
      0x73 => I32Xor,
      0x74 => I32Shl,
      0x75 => I32ShrS,
      0x76 => I32ShrU,
      0x77 => I32Rotl,
      0x78 => I32Rotr,
      // i64 operations
      0x79 => I64Clz,
      0x7a => I64Ctz,
      0x7b => I64Popcnt,
      0x7c => I64Add,
      0x7d => I64Sub,
      0x7e => I64Mul,
      0x7f => I64DivS,
      0x80 => I64DivU,
      0x81 => I64RemS,
      0x82 => I64RemU,
      0x83 => I64And,
      0x84 => I64Or,
      0x85 => I64Xor,
      0x86 => I64Shl,
      0x87 => I64ShrS,
      0x88 => I64ShrU,
      0x89 => I64Rotl,
      0x8a => I64Rotr,
      // f32 operations
      0x8b => F32Abs,
      0x8c => F32Neg,
      0x8d => F32Ceil,
      0x8e => F32Floor,
      0x8f => F32Trunc,
      0x90 => F32Nearest,
      0x91 => F32Sqrt,
      0x92 => F32Add,
      0x93 => F32Sub,
      0x94 => F32Mul,
      0x95 => F32Div,
      0x96 => F32Min,
      0x97 => F32Max,
      0x98 => F32Copysign,
      // f64 operations
      0x99 => F64Abs,
      0x9a => F64Neg,
      0x9b => F64Ceil,
      0x9c => F64Floor,
      0x9d => F64Trunc,
      0x9e => F64Nearest,
      0x9f => F64Sqrt,
      0xa0 => F64Add,
      0xa1 => F64Sub,
      0xa2 => F64Mul,
      0xa3 => F64Div,
      0xa4 => F64Min,
      0xa5 => F64Max,
      0xa6 => F64Copysign,
      // conversions
      0xa7 => I32WrapI64,
      0xa8 => I32TruncF32S,
      0xa9 => I32TruncF32U,
      0xaa => I32TruncF64S,
      0xab => I32TruncF64U,
      0xac => I64ExtendI32S,
      0xad => I64ExtendI32U,
      0xae => I64TruncF32S,
      0xaf => I64TruncF32U,
      0xb0 => I64TruncF64S,
      0xb1 => I64TruncF64U,
      0xb2 => F32ConvertI32S,
      0xb3 => F32ConvertI32U,
      0xb4 => F32ConvertI64S,
      0xb5 => F32ConvertI64U,
      0xb6 => F32DemoteF64,
      0xb7 => F64ConvertI32S,
      0xb8 => F64ConvertI32U,
      0xb9 => F64ConvertI64S,
      0xba => F64ConvertI64U,
      0xbb => F64PromoteF32,
      0xbc => I32ReinterpretF32,
      0xbd => I64ReinterpretF64,
      0xbe => F32ReinterpretI32,
      0xbf => F64ReinterpretI64,
      // https://webassembly.github.io/spec/core/binary/instructions.html#numeric-instructions
      b => return Err(parser.unexpected_byte([], b, "instruction")),
    }
  }
}
