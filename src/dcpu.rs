
use self::core::num::FromPrimitive;

extern crate core;

#[allow(non_camel_case_types)]

#[derive(FromPrimitive)]
#[repr(u16)]
pub enum Operator {
    RA = 0x00,
    RB = 0x01,
    RC = 0x02,
    RX = 0x03,
    RY = 0x04,
    RZ = 0x05,
    RI = 0x06,
    RJ = 0x07,

    M_RA = 0x08,
    M_RB = 0x09,
    M_RC = 0x0A,
    M_RX = 0x0B,
    M_RY = 0x0C,
    M_RZ = 0x0D,
    M_RI = 0x0E,
    M_RJ = 0x0F,

    MW_RA = 0x10,
    MW_RB = 0x11,
    MW_RC = 0x12,
    MW_RX = 0x13,
    MW_RY = 0x14,
    MW_RZ = 0x15,
    MW_RI = 0x16,
    MW_RJ = 0x17,

    MC_SP = 0x18,
    M_SP = 0x19,
    MW_SP = 0x1A,

    SP = 0x1B,
    PC = 0x1C,
    EX = 0x1D,

    MW = 0x1E,
    NW = 0x1F,
}

#[derive(FromPrimitive)]
#[repr(u16)]
pub enum Opcode {
    SET = 0x01,
    ADD = 0x02,
    SUB = 0x03,
    MUL = 0x04,
    MLI = 0x05,
    DIV = 0x06,
    DVI = 0x07,
    MOD = 0x08,
    MDI = 0x09,
    AND = 0x0A,
    BOR = 0x0B,
    XOR = 0x0C,
    SHR = 0x0D,
    ASR = 0x0E,
    SHL = 0x0F,
    IFB = 0x10,
    IFC = 0x11,
    IFE = 0x12,
    IFN = 0x13,
    IFG = 0x14,
    IFA = 0x15,
    IFL = 0x16,
    IFU = 0x17,
    ADX = 0x1A,
    SBX = 0x1B,
    STI = 0x1E,
    STD = 0x1F,
}

#[derive(FromPrimitive)]
#[repr(u16)]
pub enum SpecialOpcode {
    JSR = 0x01,
    INT = 0x08,
    IAG = 0x09,
    IAS = 0x0A,
    RFI = 0x0B,
    IAQ = 0x0C,
    HWN = 0x10,
    HWQ = 0x11,
    HWI = 0x12,
}

/*
struct RegisterSet {
    a: u16,
    b: u16,
    c: u16,
    x: u16,
    y: u16,
    z: u16,
    i: u16,
    j: u16,
}

struct PointerRegisterSet {
   sp: u16,
   pc: u16,
   ex: u16,
   ia: u16,
}
*/

pub struct DCPU {
   pub register: [u16; 8],
   pub special_register: [u16; 4],
   pub ram: [u16; 0x10000],

   pub cycle: u16,
}

impl DCPU {
    pub fn new() -> DCPU {
        DCPU {
            register: [0u16; 8],
            special_register: [0u16; 4],
            ram: [0u16; 0x10000],
            cycle: 0,
        }
    }

    fn get_by_operator(&mut self, operator : u16) -> u16 {

        debug!("get_by_operaotr {}", operator);

        if operator < 0x08 {
            return self.register[operator as usize];
        } else if operator < 0x10 {
            let index = operator & 0b111;
            let ptr = self.register[index as usize];

            return self.ram[ptr as usize];
        } else if operator < 0x18 {
            let index = operator & 0b111;
            let word = self.fetch_word();
            let ptr = self.register[index as usize] + word;

            return self.ram[ptr as usize];
        } else if operator == 0x18 {
            let sp = self.special_register[0];
            self.special_register[0] = sp - 1;

            return self.ram[sp as usize];
        } else if operator == 0x19 {
            let sp = self.special_register[0];

            return self.ram[sp as usize];
        } else if operator == 0x1a {
            let word = self.fetch_word();
            let ptr = self.special_register[0] + word;

            return self.ram[ptr as usize];
        } else if operator == 0x1b {
            return self.special_register[0];
        } else if operator == 0x1c {
            return self.special_register[1];
        } else if operator == 0x1d {
            return self.special_register[2];
        } else if operator == 0x1e {
            let ptr = self.fetch_word();

            return self.ram[ptr as usize];
        } else if operator == 0x1f {
            return self.fetch_word();
        } 

        return operator
    }

    fn set_by_operator(&mut self, operator : u16, value : u16) {

        debug!("set_by_operaotr {} = {}", operator, value);

        if operator < 0x08 {
            self.register[operator as usize] = value;
        } else if operator < 0x10 {
            let index = operator & 0b111;
            let ptr = self.register[index as usize];

            self.ram[ptr as usize] = value;
        } else if operator < 0x18 {
            let index = operator & 0b111;
            let word = self.fetch_word();
            let ptr = self.register[index as usize] + word;

            self.ram[ptr as usize] = value;
        } else if operator == 0x18 {
            let sp = self.special_register[0];
            self.special_register[0] = sp + 1;

            self.ram[sp as usize] = value;
        } else if operator == 0x19 {
            let sp = self.special_register[0];

            self.ram[sp as usize] = value;
        } else if operator == 0x1a {
            let word = self.fetch_word();
            let ptr = self.special_register[0] + word;

            self.ram[ptr as usize] = value;
        } else if operator == 0x1b {
            self.special_register[0] = value;
        } else if operator == 0x1c {
            self.special_register[1] = value;
        } else if operator == 0x1d {
            self.special_register[2] = value;
        } else if operator == 0x1e {
            let ptr = self.fetch_word();

            self.ram[ptr as usize] = value;
        } else if operator == 0x1f {
            let ptr = self.special_register[1] + 1;

            self.ram[ptr as usize] = value;
        } 
    }

    fn fetch_word(&mut self) -> u16 {
        let pc = self.special_register[1];
        let result = self.ram[pc as usize];
        self.special_register[1] = pc + 1;

        result
    }

    fn fetch_instruction(&mut self) -> (u16, u16, u16) {
        let word = self.fetch_word();

        debug!("fetched word {}", word);

        let instruction = word & 0b11111;
        let param_b = (word >> 5) & 0b11111;
        let param_a = (word >> (5 + 6)) & 0b111111;

        (instruction, param_a, param_b)
    }

    fn ex_op_set(&mut self, param_a : u16, param_b : u16) {
        debug!("ex_op_set");
        let value_a = self.get_by_operator(param_a);

        self.set_by_operator(param_b, value_a);
    }

    fn process_next_instruction(&mut self) -> bool {
        let (instruction, param_a, param_b) = self.fetch_instruction();

        debug!("current instruction is {} : {}, {}", instruction, param_a, param_b);

        let opcode = Opcode::from_u16(instruction);

        match opcode {
            Option::None => return false,

            _ => match opcode.unwrap() {
                Opcode::SET => self.ex_op_set(param_a, param_b),
                _ => return false,
            },
        }

        true
    }


    pub fn cycle(&mut self) {
        if self.cycle > 0 {
            self.cycle -= 1;
        } else {
            self.process_next_instruction();
        }
    }

    pub fn create_instruction(opcode : u16, param_a : u16, param_b : u16) -> u16 {
        debug!("creating instruction for {} : {}, {}", opcode, param_a, param_b);

        (opcode & 0b11111)
        + ((param_b & 0b11111) << 5) 
        + ((param_a & 0b111111) << (5 + 6))
    }
}

#[test]
fn test_cup() {
    let mut cpu = DCPU::new();

    cpu.ram[0] = DCPU::create_instruction(Opcode::SET as u16, Operator::NW as u16, Operator::RA as u16);
    cpu.ram[1] = 100;

    cpu.cycle();

    assert!(cpu.register[0] == 100, "expected 100 got {}", cpu.register[0]);
}
