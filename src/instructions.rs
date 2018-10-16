
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
    fn new(name: &str) -> Self {
        Register([false; 5])
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
   },
   S {
   },
   SB {
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
        match self {
            R => {},
            I => {},
            S => {},
            SB => {},
            U => {},
            UJ => {},
        }
        [false; 32]
    }
}

pub fn convert_bit_array_to_byte_array(bit_array: [bool; 32]) -> [u8; 4] {
    
    [0; 4]
}
