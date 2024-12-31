use owo_colors::OwoColorize;
use std::{fs, usize};

const MAX_MEMORY: u64 = 1024 * 1024 * 128;

#[derive(Debug, Clone, Default)]
struct RV64I {
    /// x0-x31
    /// x0 is the Zero register
    ///
    general_registers: [u64; 32],
    /// PC
    program_counter: u64,
    program: Vec<u8>,
}

impl RV64I {
    pub fn new(program: Vec<u8>) -> Self {
        let mut registers = [0; 32];
        registers[2] = MAX_MEMORY;

        Self {
            general_registers: registers,
            program,
            ..Default::default()
        }
    }

    pub fn step(&mut self) {
        let instr = self.fetch();

        self.program_counter += 4;

        self.general_registers[0] = 0;
        self.execute(instr);
    }

    pub fn fetch(&self) -> u32 {
        let a = [
            self.program[self.program_counter as usize],
            self.program[self.program_counter as usize + 1],
            self.program[self.program_counter as usize + 2],
            self.program[self.program_counter as usize + 3],
        ];
        let (prefix, result, suffix) = unsafe { a.align_to::<u32>() };

        if !prefix.is_empty() || !suffix.is_empty() {
            panic!("Vec<u8> is not properly aligned for u32");
        }
        result[0]
    }

    pub fn execute(&mut self, instruction: u32) {
        println!("{:032b}", instruction);
        let op_code = get_bits(instruction, 0, 6);

        let rd = get_bits(instruction, 7, 11);
        let rs1 = get_bits(instruction, 15, 19);
        let rs2 = get_bits(instruction, 20, 24);

        let funct3 = get_bits(instruction, 12, 14);
        let funct7 = get_bits(instruction, 25, 31);

        let imm_11_0 = get_bits(instruction, 20, 31);

        // TODO: sw, li, lui

        match op_code {
            // I-type (see 2.2 of ISA)
            0b0010011 => {
                println!(
                    "{:012b} {:05b} {:03b} {:05b} {:07b}",
                    imm_11_0.green(),
                    rs1.bright_blue(),
                    funct3.yellow(),
                    rd.red(),
                    op_code.yellow()
                );
                self.handle_i_type(op_code, rd, funct3, rs1, imm_11_0);
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

                // self.handle_s_type(op_code, funct3, rs1, rs2, imm);
            }
            // B-Type (see 2.2 of ISA)
            0b1100011 => {
                let imm_11 = get_bits(instruction, 7, 7);
                let imm_1_4 = get_bits(instruction, 8, 11);
                let imm_12 = get_bits(instruction, 31, 31);
                let imm_10_5 = get_bits(instruction, 25, 30);
                // self.handle_b_type(op_code, funct3, rs1, rs2, imm);
            }
            _ => {
                todo!("Failed to interpret opcode: {}", op_code);
            }
        }

        println!(
            "PC:{} Reg: {:?}",
            self.program_counter, self.general_registers
        );
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
        match (funct3, funct7) {
            (0, 0) => {
                self.general_registers[rd as usize] = self.general_registers[rs1 as usize]
                    .wrapping_add(self.general_registers[rs2 as usize]);
            }
            _ => {
                todo!()
            }
        }
    }
    fn handle_i_type(&mut self, opcode: u32, rd: u32, funct3: u32, rs1: u32, imm: u32) {
        match funct3 {
            0 => {
                self.general_registers[rd as usize] =
                    self.general_registers[rs1 as usize].wrapping_add(imm as u64);
            }
            _ => {
                todo!()
            }
        }
    }
    fn handle_s_type(&mut self, opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: u32) {}
    fn handle_b_type(&mut self, opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: u32) {}
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
        if cpu.program_counter as usize >= cpu.program.len() {
            break;
        }
        cpu.step();
    }
}
