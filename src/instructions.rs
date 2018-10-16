
pub struct InstructionLiterals {
    opcode: [bool; 7],
    funct3: Option<[bool; 3]>,
    funct7: Option<[bool; 7]>
}

impl InstructionLiterals {
    fn new(name: &str) -> Self {
        let opcode = [false; 7];
        let funct3 = None;
        let funct7 = None;

        InstructionLiterals {
            opcode,
            funct3,
            funct7
        }
    }
}

pub struct Register([bool; 5]);

impl Register {
    pub fn new(name: &str) -> Self {
        let mut output = Register([false; 5]);
        let num = name[1..].parse();
        match num {
            Ok(x) => {
                let x: u8 = x;
                for i in 0..5 {
                    output.0[4 - i] = (x >> i) % 2 != 0;
                }
                output

            }
            Err(_) => { panic!(); }
        }
    }
}


pub enum Instruction {
    R {
        opcode: [bool; 7],
        rd: Register,
        funct3: [bool; 3],
        rs1: Register,
        rs2: Register,
        funct7: [bool; 7]
   },
   I {
        opcode: [bool; 7],
        rd: Register,
        funct3: [bool; 3],
        rs1: Register,
        imm: [bool; 11],

   },
   S {
        opcode: [bool; 7],
        funct3: [bool; 3],
        rs1: Register,
        rs2: Register,
        imm: [bool; 11],
   },
   SB {
        opcode: [bool; 7],
        funct3: [bool; 3],
        rs1: Register,
        rs2: Register,
        imm: [bool; 12],
   },
   U {
       opcode: [bool; 7],
       rd: Register,
       imm: [bool; 20]
   },
   UJ {
       opcode: [bool; 7],
       rd: Register,
       imm: [bool; 20]
   }
}

impl Instruction {
    pub fn to_word(&self) -> [bool; 32] {
        let mut output = [false; 32];
        match self {
            Instruction::R{opcode, rd, funct3, rs1, rs2, funct7} => {
                let mut offset = 0;
                for x in 0..7 {
                    output[offset] = funct7[x];
                    offset += 1;
                }

                for x in 0..5 {
                    output[offset] = rs2.0[x];
                    offset += 1;
                }
                
                for x in 0..5 {
                    output[offset] = rs1.0[x];
                    offset += 1;
                }
                
                for x in 0..3 {
                    output[offset] = funct3[x];
                    offset += 1;
                }
                
                for x in 0..5 {
                    output[offset] = rd.0[x];
                    offset += 1;
                }
                
                for x in 0..7 {
                    output[offset] = opcode[x];
                    offset += 1;
                }
            },
            Instruction::I{opcode, rd, funct3, rs1, imm} => {},
            Instruction::S{opcode, funct3, rs1, rs2, imm} => {},
            Instruction::SB{opcode, funct3, rs1, rs2, imm} => {},
            Instruction::U{opcode, rd, imm} => {},
            Instruction::UJ{opcode, rd, imm} => {},
        }
        output
    }
}

pub fn convert_bit_array_to_byte_array(bit_array: [bool; 32]) -> [u8; 4] {
    let mut output = [0; 4];
    
    for i in 0..32 {
        if bit_array[i] {
            output[i/8] += 1 << (7 - (i % 8));
        }
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn generate_register_x10() {
        let x10 = Register::new("x10");
        assert_eq!(x10.0, [false, true, false, true, false])
    }

    #[test]
    fn generate_register_x11() {
        let x10 = Register::new("x11");
        assert_eq!(x10.0, [false, true, false, true, true])
    }

    #[test]
    fn test_convert_bit_array_to_byte_array() {
        let input = [false; 32];
        assert_eq!(convert_bit_array_to_byte_array(input), [0x0; 4]);
        
        let input = [true; 32];
        assert_eq!(convert_bit_array_to_byte_array(input), [0xff; 4]);
        let input = [true, false, true, false, false, true, true, false,
                     false, true, true, false, true, false, true, true,
                     false, false, false, true, false, false, false, true,
                     true, true, true, false, true, true, true, true];
        let output = [0xa6, 0x6b, 0x11, 0xef];
        assert_eq!(convert_bit_array_to_byte_array(input), output);
    }

    #[test] 
    fn generate_an_op_code() {
        let x10 = Register::new("x10");
        let x11 = Register::new("x11");
        let x12 = Register::new("x12");
        let instruction = Instruction::R {
            opcode: [false, true, true, false, false, true, true],
            rd: x10,
            rs1: x11,
            rs2: x12,
            funct3: [false, false, false],
            funct7: [false, false, false, false, false, false, false]
        };
        let word = convert_bit_array_to_byte_array(instruction.to_word());
        assert_eq!(word, [0x01, 0x45, 0x85, 0x33]);
    }
}
