#![allow(dead_code)]

extern crate websocket;

#[macro_use]
mod utils;
mod gameboy;
mod cpu;
mod cartridge;
mod memory;

use std::env;

use gameboy::GameBoy;
use cartridge::Cartridge;
use cpu::Cpu;
use memory::Memory;

fn main() {

    // Load a ROM file and return a Cartridge
    let rom = Cartridge::new(env::args().nth(1).unwrap()).unwrap();

    // Init Cpu registers
    let mut cpu = Cpu::new();

    // Init memory
    let mut mem = Memory::new();

    // Plug all emulated componants into the GameBoy
    let mut gb = GameBoy::new(&mut cpu, &rom, &mut mem);
    gb.debug();

    loop {
        cpu::opcodes::decode(&mut gb);
    }



}
