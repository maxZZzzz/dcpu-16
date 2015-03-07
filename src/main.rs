enum Operators {
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

    PUSH_POP = 0x18,

    M_SP = 0x19,
    MW_SP = 0x1A,

    SP = 0x1B,
    PC = 0x1C,
    EX = 0x1D,
    MW = 0x1E,
    NW = 0x1F,
}

enum Opcodes {
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
   pc: u16,
   sp: u16,
   ex: u16,
   ia: u16,
}

struct EmulationData {
    cycle: u16
}

struct DCPU {
   register: RegisterSet,
   pointer: PointerRegisterSet,
   ram: [u16; 0x10000],

   emulationData: EmulationData,
}

impl DCPU {
    fn new() -> DCPU {
        DCPU {
            register: RegisterSet {
                a: 0,
                b: 0,
                c: 0,
                x: 0,
                y: 0,
                z: 0,
                i: 0,
                j: 0,
            },
            pointer: PointerRegisterSet {
                pc: 0,
                sp: 0,
                ex: 0,
                ia: 0,
            },
            ram: [0u16; 0x10000],
            emulationData: EmulationData {
                cycle: 0
            }
        }
    }

    fn fetch_word(&mut self) -> u16 {
        let result = self.ram[self.pointer.pc as usize];
        self.pointer.pc += 1;

        result
    }

    fn fetch_instruction(&mut self) -> (u16, u16, u16) {
        let word = self.fetch_word();

        let instruction = word & 0b11111;
        let param_b = (word >> 5) & 0b11111;
        let param_a = (word >> 5) & 0b111111;

        (instruction, param_a, param_b)
    }

    fn process_next_instruction(&mut self) {
        let (instruction, param_a, param_b) = self.fetch_instruction();
    }


    fn cycle(&mut self) {
        if (self.emulationData.cycle > 0) {
            self.emulationData.cycle -= 1;
        } else {
            self.process_next_instruction();
        }
    }
}

fn main() {
    let mut cpu = DCPU::new();

    cpu.cycle();
}
