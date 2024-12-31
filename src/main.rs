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
        let op_code = get_bits(instruction, 0, 6);
        let mut rd = None;
        let mut funct3 = None;
        let mut rs1 = None;
        let mut rs2 = None;
        let mut funct7 = None;
        let mut imm_11_0 = None;

        match op_code {
            // I-type (see 2.2 of ISA)
            19 => {
                rd = Some(get_bits(instruction, 7, 11));
                funct3 = Some(get_bits(instruction, 12, 14));
                rs1 = Some(get_bits(instruction, 15, 19));
                imm_11_0 = Some(get_bits(instruction, 20, 31));

                println!(
                    "{:012b} {:05b} {:03b} {:05b} {:07b}",
                    imm_11_0.unwrap().green(),
                    rs1.unwrap().bright_blue(),
                    funct3.unwrap().yellow(),
                    rd.unwrap().red(),
                    op_code.yellow()
                );
            }
            // R-Type (see 2.2 of ISA)
            51 => {
                rd = Some(get_bits(instruction, 7, 11));
                funct3 = Some(get_bits(instruction, 12, 14));
                rs1 = Some(get_bits(instruction, 15, 19));
                rs2 = Some(get_bits(instruction, 20, 24));
                funct7 = Some(get_bits(instruction, 25, 31));

                println!(
                    "{:07b} {:05b} {:05b} {:03b} {:05b} {:07b}",
                    funct7.unwrap().yellow(),
                    rs2.unwrap().bright_blue(),
                    rs1.unwrap().bright_blue(),
                    funct3.unwrap().yellow(),
                    rd.unwrap().red(),
                    op_code.yellow()
                );
            }
            _ => {
                todo!("Failed to interpret opcode: {}", op_code);
            }
        }

        match (op_code, funct3, funct7, rs1, rs2, rd, imm_11_0) {
            // ADDI
            (19, Some(0), _, Some(rs1_in), _, Some(rd_in), Some(imm_11_0)) => {
                self.general_registers[rd_in as usize] =
                    self.general_registers[rs1_in as usize].wrapping_add(imm_11_0 as u64);
            }
            (51, Some(0), Some(0), Some(rs1_in), Some(rs2_in), Some(rd_in), _) => {
                self.general_registers[rd_in as usize] = self.general_registers[rs1_in as usize]
                    .wrapping_add(self.general_registers[rs2_in as usize]);
            }
            v => {
                panic!("{:?}", v);
            }
        }
        println!(
            "PC:{} Reg: {:?}",
            self.program_counter, self.general_registers
        );
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
        if cpu.program_counter as usize >= cpu.program.len() {
            break;
        }
        cpu.step();
    }
}
