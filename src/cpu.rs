use crate::ops;
use std::collections::HashMap;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub memory: [u8; 0xffff],
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    Indirect,
    Accumulator,
    Relative,
    Implied,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xffff],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.program_counter = self.mem_read_u16(0xfffc);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
        self.mem_write_u16(0xfffc, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter), 
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            },
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            },
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            },
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            },
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            _ => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    fn update_processor_status(&mut self, condition: bool, flag: u8) {
        if condition {
            self.status |= flag;
        } else {
            self.status &= !flag;
        }
    }

    fn update_status_z_n(&mut self, value: u8) {
        self.update_processor_status(value == 0, 0b0000_0010);
        self.update_processor_status(value & 0b1000_0000 != 0,  0b1000_0000);
    }

    // add with carry
    fn _adc() {}

    // logical and
    fn _and() {}

    // arithmetic shift left
    fn _asl() {}

    // branches
    fn _bcc() {}
    fn _bcs() {}
    fn _beq() {}
    fn _bit() {}
    fn _bmi() {}
    fn _bne() {}
    fn _bpl() {}

    // break
    fn _brk() {}

    // clear carry/decimal/int_dsbl/overflow flags
    fn _clc() {}
    fn _cld() {}
    fn _cli() {}
    fn _clv() {}

    // compare mem, x/y
    fn _cmp() {}
    fn _cpx() {}
    fn _cpy() {}

    // dec mem, reg x/y
    fn _dec() {}
    fn _dex() {}
    fn _dey() {}

    // exclusive or
    fn _eor() {}

    // inc mem, reg x/y
    fn _inc() {}
    fn inx(&mut self, _mode: &AddressingMode) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_status_z_n(self.register_x);
    }
    fn _iny() {}

    // jmp
    fn _jmp() {}
    // jmp subroutine
    fn _jsr() {}

    // load reg a/x/y
    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = value;
        self.update_status_z_n(value);
    }
    fn ldx(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_x = value;
        self.update_status_z_n(value);
    }
    fn ldy(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_y = value;
        self.update_status_z_n(value);
    }

    // logical shift right
    fn _lsr() {}

    // no op
    fn _nop() {}

    // logical inclusive or
    fn _ora() {}

    // push/pop reg a/p
    fn _pha() {}
    fn _php() {}
    fn _pla() {}
    fn _plp() {}

    // rotate left/right
    fn _rol() {}
    fn _ror() {}

    // return from interrrupt/subroutine
    fn _rti() {}
    fn _rts() {}

    // subtract with carry
    fn _sbc() {}

    // set carry/decimal/int_dsbl flags
    fn _sec() {}
    fn _sed() {}
    fn _sei() {}

    // store reg a/x/y
    fn _sta() {}
    fn _stx() {}
    fn _sty() {}

    fn tax(&mut self, _mode: &AddressingMode) {
        self.register_x = self.register_a;
        self.update_status_z_n(self.register_x);
    }
    fn _tay() {}
    fn _tsx() {}
    fn _txa() {}
    fn _txs() {}
    fn _tya() {}

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static ops::OpCode> = *ops::OPCODES_MAP;

        loop {
            let opcode = self.mem_read(self.program_counter);
            let op = opcodes.get(&opcode).unwrap();
            self.program_counter += 1;

            match opcode {
                0x00 => { // BRK
                    return;
                },
                0xe8 => { // INX
                    self.inx(&op.mode);
                },

                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => { // LDA
                    self.lda(&op.mode);
                },

                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => { // LDX
                    self.ldx(&op.mode)
                }

                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => {
                    self.ldy(&op.mode)
                }

                0xaa => {
                    self.tax(&op.mode);
                },
                _ => todo!("opcode {:#02x}", opcode)
            }

            self.program_counter += op.cycles as u16 - 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa0_ldy_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa0_ldy_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
        assert!(cpu.register_y == 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa0_ldy_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x80, 0x00]);
        assert_eq!(cpu.register_y, 0x80);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xa1_lda_indirect_x_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x14, 0x77);
        cpu.mem_write(0x15, 0x7f);
        cpu.mem_write(0x7f77, 0x05);
        cpu.load_and_run(vec![
            0xa2, 0x10,
            0xa1, 0x04,
            0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa2_ldx_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa2_ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.register_x == 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa2_ldx_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x80, 0x00]);
        assert_eq!(cpu.register_x, 0x80);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xa4_ldy_zeropage_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xF0, 0x05);
        cpu.load_and_run(vec![0xa4, 0xF0, 0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa5_lda_zeropage_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xF0, 0x05);
        cpu.load_and_run(vec![0xa5, 0xF0, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa6_ldx_zeropage_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0xF0, 0x05);
        cpu.load_and_run(vec![0xa6, 0xF0, 0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.register_a == 0);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x80, 0x00]);
        assert_eq!(cpu.register_a, 0x80);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xaa_tax_transfer_num() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xaa, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.register_x, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xaa_tax_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xaa, 0x00]);
        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x80, 0xaa, 0x00]);
        assert_eq!(cpu.register_a, 0x80);
        assert_eq!(cpu.register_x, 0x80);
        assert!(cpu.status & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0xac_lda_absolute_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x7000, 0x05);
        cpu.load_and_run(vec![
            0xac, 0x00, 0x70,
            0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xad_lda_absolute_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x7000, 0x05);
        cpu.load_and_run(vec![
            0xad, 0x00, 0x70,
            0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xae_ldx_absolute_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x7000, 0x05);
        cpu.load_and_run(vec![
            0xae, 0x00, 0x70,
            0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xb1_lda_indirect_y_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x04, 0x77);
        cpu.mem_write(0x05, 0x7f);
        cpu.mem_write(0x7f77, 0x05);
        cpu.load_and_run(vec![
            0xa2, 0x10,
            0xb1, 0x04,
            0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xb4_ldy_zeropage_x_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x008F, 0x05);
        cpu.load_and_run(vec![
            0xa2, 0x0F,
            0xb4, 0x80,
            0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xb5_lda_zeropage_x_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x008F, 0x05);
        cpu.load_and_run(vec![
            0xa2, 0x0F,
            0xb5, 0x80,
            0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xb6_ldx_zeropage_y_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x008F, 0x05);
        cpu.load_and_run(vec![
            0xa0, 0x0F,
            0xb6, 0x80,
            0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xb9_lda_absolute_y_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x800F, 0x05);
        cpu.load_and_run(vec![
            0xa0, 0x0F,
            0xb9, 0x00, 0x80,
            0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xbc_lda_absolute_x_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x800F, 0x05);
        cpu.load_and_run(vec![
            0xa2, 0x0F,
            0xbc, 0x00, 0x80,
            0x00]);
        assert_eq!(cpu.register_y, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xbd_lda_absolute_x_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x800F, 0x05);
        cpu.load_and_run(vec![
            0xa2, 0x0F,
            0xbd, 0x00, 0x80,
            0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xbe_ldx_absolute_y_load_data() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x800F, 0x05);
        cpu.load_and_run(vec![
            0xa0, 0x0F,
            0xbe, 0x00, 0x80,
            0x00]);
        assert_eq!(cpu.register_x, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }
}
