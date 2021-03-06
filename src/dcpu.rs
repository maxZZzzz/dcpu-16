extern crate core;

#[allow(non_camel_case_types)]

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

impl Opcode {
    pub fn from_u16(opcode : u16) -> Option<Self> {
        match opcode {
            0x01 => Some(Opcode::SET),
            0x02 => Some(Opcode::ADD),
            0x03 => Some(Opcode::SUB),
            0x04 => Some(Opcode::MUL),
            0x05 => Some(Opcode::MLI),
            0x06 => Some(Opcode::DIV),
            0x07 => Some(Opcode::DVI),
            0x08 => Some(Opcode::MOD),
            0x09 => Some(Opcode::MDI),
            0x0A => Some(Opcode::AND),
            0x0B => Some(Opcode::BOR),
            0x0C => Some(Opcode::XOR),
            0x0D => Some(Opcode::SHR),
            0x0E => Some(Opcode::ASR),
            0x0F => Some(Opcode::SHL),
            0x10 => Some(Opcode::IFB),
            0x11 => Some(Opcode::IFC),
            0x12 => Some(Opcode::IFE),
            0x13 => Some(Opcode::IFN),
            0x14 => Some(Opcode::IFG),
            0x15 => Some(Opcode::IFA),
            0x16 => Some(Opcode::IFL),
            0x17 => Some(Opcode::IFU),
            0x1A => Some(Opcode::ADX),
            0x1B => Some(Opcode::SBX),
            0x1E => Some(Opcode::STI),
            0x1F => Some(Opcode::STD),
            _ => None
        }
    }
}

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

    fn process_next_instruction(&mut self) -> bool {
        let (instruction, param_a, param_b) = self.fetch_instruction();

        debug!("current instruction is {} : {}, {}", instruction, param_a, param_b);

        let opcode = Opcode::from_u16(instruction);

        match opcode {
            Option::None => return false,

            _ => match opcode.unwrap() {
                Opcode::SET => self.ex_op_set(param_a, param_b),
                Opcode::ADD => self.ex_op_add(param_a, param_b),
                Opcode::SUB => self.ex_op_sub(param_a, param_b),
                Opcode::MUL => self.ex_op_mul(param_a, param_b),
                Opcode::MLI => self.ex_op_mli(param_a, param_b),
                Opcode::DIV => self.ex_op_div(param_a, param_b),
                Opcode::DVI => self.ex_op_dvi(param_a, param_b),
                Opcode::MOD => self.ex_op_mod(param_a, param_b),
                Opcode::MDI => self.ex_op_mdi(param_a, param_b),
                Opcode::AND => self.ex_op_and(param_a, param_b),
                Opcode::BOR => self.ex_op_bor(param_a, param_b),
                Opcode::XOR => self.ex_op_xor(param_a, param_b),
                Opcode::SHR => self.ex_op_shr(param_a, param_b),
                Opcode::ASR => self.ex_op_asr(param_a, param_b),
                Opcode::SHL => self.ex_op_shl(param_a, param_b),
                Opcode::IFB => self.ex_op_ifb(param_a, param_b),
                Opcode::IFC => self.ex_op_ifc(param_a, param_b),
                Opcode::IFE => self.ex_op_ife(param_a, param_b),
                Opcode::IFN => self.ex_op_ifn(param_a, param_b),
                Opcode::IFG => self.ex_op_ifg(param_a, param_b),
                Opcode::IFA => self.ex_op_ifa(param_a, param_b),
                Opcode::IFL => self.ex_op_ifl(param_a, param_b),
                Opcode::IFU => self.ex_op_ifu(param_a, param_b),
                Opcode::ADX => self.ex_op_adx(param_a, param_b),
                Opcode::SBX => self.ex_op_sbx(param_a, param_b),
                Opcode::STI => self.ex_op_sti(param_a, param_b),
                Opcode::STD => self.ex_op_std(param_a, param_b),
            },
        }

        true
    }

    fn ex_op_set(&mut self, param_a : u16, param_b : u16) {
        debug!("ex_op_set");
        let value_a = self.get_by_operator(param_a);

        self.set_by_operator(param_b, value_a);
    }

    fn ex_op_add(&mut self, param_a : u16, param_b : u16) {
        debug!("ex_op_set");
        let value_a = self.get_by_operator(param_a) as u32;
        let value_b = self.get_by_operator(param_b) as u32;

        let result = value_b + value_a;
        let ex = ((result & 0xFFFF0000) > 0) as u16;

        self.set_by_operator(Operator::EX as u16, ex);
        self.set_by_operator(param_b, (result & 0xFFFF) as u16);
    }

    //FIXME: that is probably wrong
    fn ex_op_sub(&mut self, param_a : u16, param_b : u16) {
        debug!("ex_op_sub");
        let value_a = self.get_by_operator(param_a) as u32;
        let value_b = self.get_by_operator(param_b) as u32;

        let result = value_b - value_a;
        let ex = ((result & 0xFFFF0000) > 0) as u16;

        self.special_register[2] = ex;

        self.set_by_operator(param_b, (result & 0xFFFF) as u16);
    }

    fn ex_op_mul(&mut self, param_a : u16, param_b : u16) {
        debug!("ex_op_mul");
        let value_a = self.get_by_operator(param_a) as u32;
        let value_b = self.get_by_operator(param_b) as u32;

        let result = value_b * value_a;
        let ex = ((result >> 16) & 0xFFFF) as u16;

        self.special_register[2] = ex;
        self.set_by_operator(param_b, result as u16);
    }

    fn ex_op_mli(&mut self, param_a : u16, param_b : u16) {
        debug!("ex_op_mli");
        let value_a = self.get_by_operator(param_a) as i32;
        let value_b = self.get_by_operator(param_b) as i32;

        let result = value_b * value_a;
        let ex = ((result >> 16) & 0xFFFF) as u16;

        self.special_register[2] = ex;
        self.set_by_operator(param_b, (result & 0xFFFF) as u16);
    }

    fn ex_op_div(&mut self, param_a : u16, param_b : u16) {
        debug!("ex_op_div");
        let value_a = self.get_by_operator(param_a) as u32;
        let value_b = self.get_by_operator(param_b) as u32;

        let result = value_b / value_a;
        let ex = (((value_b << 16) / value_a) & 0xFFFF) as u16;

        self.special_register[2] = ex;
        self.set_by_operator(param_b, (result & 0xFFFF) as u16);
    }

    fn ex_op_dvi(&mut self, param_a : u16, param_b : u16) {
        debug!("ex_op_dvi");
        let value_a = self.get_by_operator(param_a) as u32;
        let value_b = self.get_by_operator(param_b) as u32;

        let result = value_b / value_a;
        let ex = (((value_b << 16) / value_a) & 0xFFFF) as u16;

        self.special_register[2] = ex;
        self.set_by_operator(param_b, (result & 0xFFFF) as u16);
    }
    

    fn ex_op_mod (&mut self, param_a : u16, param_b : u16) {
        let value_a = self.get_by_operator(param_a);
        let value_b = self.get_by_operator(param_a);

        let result = value_b % value_a;

        self.set_by_operator(param_b, result);
    }

    fn ex_op_mdi (&mut self, param_a : u16, param_b : u16) {
        let value_a = self.get_by_operator(param_a) as i16;
        let value_b = self.get_by_operator(param_a) as i16;

        let result = value_b % value_a;

        self.set_by_operator(param_b, result as u16);
    }

    fn ex_op_and (&mut self, param_a : u16, param_b : u16) {
        let value_a = self.get_by_operator(param_a);
        let value_b = self.get_by_operator(param_a);

        let result = value_b & value_a;

        self.set_by_operator(param_b, result);
    }

    fn ex_op_bor (&mut self, param_a : u16, param_b : u16) {
        let value_a = self.get_by_operator(param_a);
        let value_b = self.get_by_operator(param_a);

        let result = value_b | value_a;

        self.set_by_operator(param_b, result);
    }

    fn ex_op_xor (&mut self, param_a : u16, param_b : u16) {
        let value_a = self.get_by_operator(param_a);
        let value_b = self.get_by_operator(param_a);

        let result = value_b ^ value_a;

        self.set_by_operator(param_b, result);
    }

    fn ex_op_shr (&mut self, param_a : u16, param_b : u16) {
        let value_a = self.get_by_operator(param_a) as u32;
        let value_b = self.get_by_operator(param_a) as u32;

        let result = value_b >> value_a;
        let ex = ((value_b << 16) >> value_a) & 0xFFFF;

        self.set_by_operator(Operator::EX as u16, ex as u16);
        self.set_by_operator(param_b, result as u16);
    }

    fn ex_op_asr (&mut self, param_a : u16, param_b : u16) {
        // FIXME: the sign may have to be exapnded here properly
        let value_a = self.get_by_operator(param_a) as i32;
        let value_b = self.get_by_operator(param_a) as i32;

        let result = value_b >> value_a;
        let ex = ((value_b << 16) >> value_a) & 0xFFFF;

        self.set_by_operator(Operator::EX as u16, ex as u16);
        self.set_by_operator(param_b, result as u16 as u16);
    }

    fn ex_op_shl (&mut self, param_a : u16, param_b : u16) {
        let value_a = self.get_by_operator(param_a) as u32;
        let value_b = self.get_by_operator(param_a) as u32;

        let ca = (value_b << value_a);
        let ex = (ca >> 16) & 0xFFFF;
        let result = ca & 0xFFFF;

        self.set_by_operator(Operator::EX as u16, ex as u16);
        self.set_by_operator(param_b, result as u16);
    }

    fn ex_op_ifb (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_ifc (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_ife (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_ifn (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_ifg (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_ifa (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_ifl (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_ifu (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_adx (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_sbx (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_sti (&mut self, param_a : u16, param_b : u16) {
    }

    fn ex_op_std (&mut self, param_a : u16, param_b : u16) {
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

    pub fn print_cpu_status(&self) {
        println!("RA = {}", self.register[0]);
        println!("RB = {}", self.register[1]);
        println!("RC = {}", self.register[2]);
        println!("RX = {}", self.register[3]);
        println!("RY = {}", self.register[4]);
        println!("RZ = {}", self.register[5]);
        println!("RI = {}", self.register[6]);
        println!("RJ = {}", self.register[7]);

        println!("");

        println!("SP = {}", self.special_register[0]);
        println!("PC = {}", self.special_register[1]);
        println!("EX = {}", self.special_register[2]);
        println!("IA = {}", self.special_register[3]);

        println!("CYCLES = {}", self.cycle);
    }
}

fn test_set_for_operator<F>(op : Operator, test : F) where F : Fn(&mut DCPU) {
    let mut cpu = DCPU::new();
    let mut pc = 0;

    // SET RA
    cpu.ram[pc] = DCPU::create_instruction(Opcode::SET as u16, Operator::NW as u16, op as u16);
    pc += 1;
    cpu.ram[pc] = 0xFFFF;
    pc += 1;
    cpu.ram[pc] = 0xFFFF;
    cpu.cycle();

    cpu.print_cpu_status();

    test(&mut cpu);
}

#[test]
fn test_op_set() {

    test_set_for_operator(
        Operator::RA,
        |cpu| assert!(
            0xFFFF == cpu.register[0],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::RB,
        |cpu| assert!(
            0xFFFF == cpu.register[1],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::RC,
        |cpu| assert!(
            0xFFFF == cpu.register[2],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::RX,
        |cpu| assert!(
            0xFFFF == cpu.register[3],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::RY,
        |cpu| assert!(
            0xFFFF == cpu.register[4],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::RZ,
        |cpu| assert!(
            0xFFFF == cpu.register[5],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::RI,
        |cpu| assert!(
            0xFFFF == cpu.register[6],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::RJ,
        |cpu| assert!(
            0xFFFF == cpu.register[7],
            "assert failed"
        )
    );

    // ==== PTR SET ====


    test_set_for_operator(
        Operator::M_RA,
        |cpu| assert!(
            0xFFFF == cpu.ram[0],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::M_RB,
        |cpu| assert!(
            0xFFFF == cpu.ram[0],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::M_RC,
        |cpu| assert!(
            0xFFFF == cpu.ram[0],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::M_RX,
        |cpu| assert!(
            0xFFFF == cpu.ram[0],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::M_RY,
        |cpu| assert!(
            0xFFFF == cpu.ram[0],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::M_RZ,
        |cpu| assert!(
            0xFFFF == cpu.ram[0],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::M_RI,
        |cpu| assert!(
            0xFFFF == cpu.ram[0],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::M_RJ,
        |cpu| assert!(
            0xFFFF == cpu.ram[0],
            "assert failed"
        )
    );

    // ==== OFFSET PTR SET ====

    test_set_for_operator(
        Operator::MW_RA,
        |cpu| assert!(
            0xFFFF == cpu.ram[0xFFFF],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::MW_RB,
        |cpu| assert!(
            0xFFFF == cpu.ram[0xFFFF],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::MW_RC,
        |cpu| assert!(
            0xFFFF == cpu.ram[0xFFFF],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::MW_RX,
        |cpu| assert!(
            0xFFFF == cpu.ram[0xFFFF],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::MW_RY,
        |cpu| assert!(
            0xFFFF == cpu.ram[0xFFFF],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::MW_RZ,
        |cpu| assert!(
            0xFFFF == cpu.ram[0xFFFF],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::MW_RI,
        |cpu| assert!(
            0xFFFF == cpu.ram[0xFFFF],
            "assert failed"
        )
    );

    test_set_for_operator(
        Operator::MW_RJ,
        |cpu| assert!(
            0xFFFF == cpu.ram[0xFFFF],
            "assert failed"
        )
    );
}

#[test]
fn test_cpu() {
    let mut cpu = DCPU::new();
    let mut pc = 0;

    // SET RB
//    cpu.ram[pc] = DCPU::create_instruction(Opcode::SET as u16, Operator::NW as u16, Operator::RB as u16);
//    pc += 1;
//    cpu.ram[pc] = 100;
//    pc += 1;
//    cpu.cycle();
//    println!("RA = {}, RB = {}", cpu.register[0], cpu.register[1]);
//    assert!(
//        100 == cpu.register[0] && 
//        100 == cpu.register[1],
//        "RA = {}, RB = {}", 
//        cpu.register[0],
//        cpu.register[1]
//    );
//
//    // ADD RB RA
//    cpu.ram[pc] = DCPU::create_instruction(Opcode::ADD as u16, Operator::RA as u16, Operator::RB as u16);
//    pc += 1;
//    cpu.cycle();
//    println!("RA = {}, RB = {}", cpu.register[0], cpu.register[1]);
//    assert!(
//        100 == cpu.register[0] && 
//        200 == cpu.register[1],
//        "RA = {}, RB = {}", 
//        cpu.register[0],
//        cpu.register[1]
//    );
//
//    // SUB RB RA
//    cpu.ram[pc] = DCPU::create_instruction(Opcode::SUB as u16, Operator::RA as u16, Operator::RB as u16);
//    pc += 1;
//    cpu.cycle();
//    println!("RA = {}, RB = {}", cpu.register[0], cpu.register[1]);
//    assert!(
//        100 == cpu.register[0] && 
//        100 == cpu.register[1],
//        "RA = {}, RB = {}", 
//        cpu.register[0],
//        cpu.register[1]
//    );
}
