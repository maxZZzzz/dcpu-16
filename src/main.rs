#![feature(core)]

use dcpu::DCPU;
use dcpu::Operator;
use dcpu::Opcode;

#[macro_use]
extern crate log;

mod dcpu;

fn main() {
    let mut cpu = DCPU::new();

    cpu.ram[0] = DCPU::create_instruction(Opcode::SET as u16, Operator::NW as u16, Operator::RA as u16);
    cpu.ram[1] = 100;

    println!("RA = {}", cpu.register[0]);

    cpu.cycle();

    println!("RA = {}", cpu.register[0]);
}
