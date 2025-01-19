use owo_colors::OwoColorize;
use std::{fs, u64, usize};

mod dram;
use dram::{Bus, Memory, DRAM_BASE, MAX_MEMORY};

#[derive(Debug, Clone, Default)]
struct RV64I {
    /// x0-x31
    /// x0 is the Zero register
    ///
    general_registers: [u64; 32],
    /// PC
    program_counter: u64,
    bus: Bus,
    stalled: bool,
}

impl RV64I {
    pub fn new(program: Vec<u8>) -> Self {
        let mut registers = [0; 32];
        registers[2] = MAX_MEMORY;
        let memory = Memory::new(program);
        let bus = Bus::new(memory);

        Self {
            general_registers: registers,
            bus,
            stalled: false,
            program_counter: DRAM_BASE,
            ..Default::default()
        }
    }

    pub fn step(&mut self) {
        let instr = match self.fetch() {
            Ok(0) => {
                self.stalled = true;
                dbg!(self.stalled);
                return;
            }
            Ok(v) => v,
            _ => {
                self.stalled = true;
                dbg!(self.stalled);
                return;
            }
        };

        self.program_counter += 4;

        self.general_registers[0] = 0;
        if let Err(_) = self.execute(instr) {
            self.stalled = true;
            dbg!(self.stalled);
            return;
        }
    }

    pub fn fetch(&self) -> Result<u32, ()> {
        match self.bus.load(self.program_counter, 32) {
            Ok(v) => Ok(v as u32),
            Err(v) => Err(v),
        }
    }

    pub fn execute(&mut self, instruction: u32) -> Result<(), ()> {
        println!("{:032b}", instruction);
        let op_code = get_bits(instruction, 0, 6);

        let rd = get_bits(instruction, 7, 11);
        let rs1 = get_bits(instruction, 15, 19);
        let rs2 = get_bits(instruction, 20, 24);

        let funct3 = get_bits(instruction, 12, 14);
        let funct7 = get_bits(instruction, 25, 31);

        let imm_11_0 = get_bits(instruction, 20, 31);

        match op_code {
            // I-type (see 2.2 of ISA)
            0b0010011 | 0b0000011 => {
                let imm = if imm_11_0 >> 11 == 1 {
                    imm_11_0 as u64 | (u64::MAX << 11)
                } else {
                    imm_11_0 as u64
                };
                println!(
                    "{:032b} {:05b} {:03b} {:05b} {:07b}",
                    imm.green(),
                    rs1.bright_blue(),
                    funct3.yellow(),
                    rd.red(),
                    op_code.yellow()
                );
                self.handle_i_type(op_code, rd, funct3, rs1, imm);
            }
            // R-Type (see 2.2 of ISA)
            0b0110011 => {
                println!(
                    "{:07b} {:05b} {:05b} {:03b} {:05b} {:07b}",
                    funct7.yellow(),
                    rs2.bright_blue(),
                    rs1.bright_blue(),
                    funct3.yellow(),
                    rd.red(),
                    op_code.yellow()
                );
                self.handle_r_type(op_code, rd, funct3, rs1, rs2, funct7);
            }
            // S-Type (see 2.2 of ISA)
            0b0100011 => {
                let imm_4_0 = get_bits(instruction, 7, 11);
                let imm_11_5 = get_bits(instruction, 25, 31);

                let imm = imm_11_5 << 5 | imm_4_0;

                let imm = if imm >> 11 == 1 {
                    imm as u64 | (u64::MAX << 11)
                } else {
                    imm as u64
                };

                self.handle_s_type(op_code, funct3, rs1, rs2, imm)?;
            }
            // B-Type (see 2.2 of ISA)
            0b1100011 => {
                let imm_11 = get_bits(instruction, 7, 7);
                let imm_1_4 = get_bits(instruction, 8, 11);
                let imm_12 = get_bits(instruction, 31, 31);
                let imm_10_5 = get_bits(instruction, 25, 30);

                // TODO: Verify
                let imm = imm_1_4 << 1 | imm_10_5 << 5 | imm_11 << 11 | imm_12 << 12;
                let imm = if imm >> 12 == 1 {
                    imm as u64 | (u64::MAX << 12)
                } else {
                    imm as u64
                };

                todo!("Validate B-type Immidiate Value");

                self.handle_b_type(op_code, funct3, rs1, rs2, imm);
            }
            _ => {
                todo!("Failed to interpret opcode: {}", op_code);
            }
        }

        println!(
            "PC:{} Reg: {:?}",
            self.program_counter, self.general_registers
        );
        Ok(())
    }

    fn handle_r_type(
        &mut self,
        opcode: u32,
        rd: u32,
        funct3: u32,
        rs1: u32,
        rs2: u32,
        funct7: u32,
    ) {
        match (opcode, funct3, funct7) {
            (0b0110011, 0, 0) => {
                self.general_registers[rd as usize] = self.general_registers[rs1 as usize]
                    .wrapping_add(self.general_registers[rs2 as usize]);
            }
            _ => {
                todo!("OP:{:08b} F3:{:04b}", opcode, funct3);
            }
        }
    }
    fn handle_i_type(&mut self, opcode: u32, rd: u32, funct3: u32, rs1: u32, imm: u64) {
        match (opcode, funct3) {
            (0b0000011, 0b000) => {
                // lb
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                let val = self
                    .bus
                    .load(addr, 8)
                    .expect("Update Code path for I instr");
                self.general_registers[rd as usize] = val as i8 as i64 as u64;
            }
            (0b0000011, 0b001) => {
                // lh
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                let val = self
                    .bus
                    .load(addr, 16)
                    .expect("Update Code path for I instr");
                self.general_registers[rd as usize] = val as i16 as i64 as u64;
            }
            (0b0000011, 0b010) => {
                // lw
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                let val = self
                    .bus
                    .load(addr, 32)
                    .expect("Update Code path for I instr");
                self.general_registers[rd as usize] = val as i32 as i64 as u64;
            }
            (0b0000011, 0b011) => {
                // ld
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                let val = self
                    .bus
                    .load(addr, 64)
                    .expect("Update Code path for I instr");
                self.general_registers[rd as usize] = val;
            }
            (0b0000011, 0b100) => {
                // lbu
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                let val = self
                    .bus
                    .load(addr, 8)
                    .expect("Update Code path for I instr");
                self.general_registers[rd as usize] = val;
            }
            (0b0000011, 0b101) => {
                // lhu
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                let val = self
                    .bus
                    .load(addr, 16)
                    .expect("Update Code path for I instr");
                self.general_registers[rd as usize] = val;
            }
            (0b0000011, 0b110) => {
                // lhu
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                let val = self
                    .bus
                    .load(addr, 32)
                    .expect("Update Code path for I instr");
                self.general_registers[rd as usize] = val;
            }
            (0b0010011, 0b000) => {
                // ADDI (also noop when rs1, rd, imm are zero)
                self.general_registers[rd as usize] =
                    self.general_registers[rs1 as usize].wrapping_add(imm);
            }
            _ => {
                todo!("OP:{:08b} F3:{:04b}", opcode, funct3);
            }
        }
    }
    fn handle_s_type(
        &mut self,
        opcode: u32,
        funct3: u32,
        rs1: u32,
        rs2: u32,
        imm: u64,
    ) -> Result<(), ()> {
        match (opcode, funct3) {
            (0b0100011, 0b000) => {
                // sb
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                self.bus
                    .store(addr, 8, self.general_registers[rs2 as usize])?;
            }
            (0b0100011, 0b001) => {
                // sh
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                self.bus
                    .store(addr, 16, self.general_registers[rs2 as usize])?;
            }
            (0b0100011, 0b010) => {
                // sw
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                self.bus
                    .store(addr, 32, self.general_registers[rs2 as usize])?;
            }
            (0b0100011, 0b011) => {
                // sd
                let addr = self.general_registers[rs1 as usize].wrapping_add(imm);
                self.bus
                    .store(addr, 64, self.general_registers[rs2 as usize])?;
            }
            _ => {
                todo!("OP:{:08b} F3:{:04b}", opcode, funct3);
            }
        }
        Ok(())
    }
    fn handle_b_type(&mut self, opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: u64) {
        match (opcode, funct3) {
            _ => {
                todo!("OP:{:08b} F3:{:04b}", opcode, funct3);
            }
        }
    }
}

fn get_bits(value: u32, start: u32, end: u32) -> u32 {
    let num_bits = end - start + 1;
    let mask = (1 << num_bits) - 1;
    (value >> start) & mask
}

fn main() {
    let file = fs::read("./test.bin").unwrap();
    // let file = vec_u8_to_vec_u32(file);
    let mut cpu = RV64I::new(file);

    loop {
        if cpu.stalled {
            break;
        }
        cpu.step();
    }
}
