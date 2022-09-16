pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
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
    fn inx(&mut self) {
        self.register_x += 1;
        self.update_status_z_n(self.register_x);
    }
    fn _iny() {}

    // jmp
    fn _jmp() {}
    // jmp subroutine
    fn _jsr() {}

    // load reg a/x/y
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_status_z_n(self.register_a);
    }
    fn _ldx(){}
    fn _ldy(){}

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

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_status_z_n(self.register_x);
    }
    fn _tay() {}
    fn _tsx() {}
    fn _txa() {}
    fn _txs() {}
    fn _tya() {}

    pub fn run(&mut self, program: Vec<u8>) {
        self.program_counter = 0;
        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                0x00 => { // BRK
                    return;
                },
                0xa9 => { // LDA
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param);
                },
                0xaa => self.tax(),
                0xe8 => self.inx(),
                _ => todo!("opcode {:#02x}", opcode)

            }
        }
    }
}
