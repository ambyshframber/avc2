use wrapping_arithmetic::wrappit;

use crate::memory::Mem;

const WST_START: u16 = 0x0100;
const RST_START: u16 = 0x0200;

pub struct Processor {
    mem: Mem,
    wsp: u8,
    rsp: u8,
    // 7 6 5 4 3 2 1 0
    //             o c
    st: u8,
    pc: u16,
}

impl Processor {
    pub fn new(rom: &[u8]) -> Processor {
        let mem = Mem::new_from_rom(rom);
        Processor {
            mem,
            wsp: 0xff, rsp: 0xff, st: 0,
            pc: 0x0300,
        }
    }

    pub fn execute_once(&mut self) {
        let instr = self.mem.get(self.pc);
        self.execute(instr)
    }

    #[wrappit]
    fn execute(&mut self, instr: u8) {
        println!("EXEC {:02x}", instr);
        println!("WSP AT {:04x}", self.wsp);
        println!("RSP AT {:04x}", self.rsp);
        let k = instr & 0x80 != 0; // keep
        let r = instr & 0x40 != 0; // return stack
        let d = instr & 0x20 != 0; // double width
        let op = instr & 0b11111;

        match op { // instruction decode
            0 => { // lit and extras
                if k { // LIT
                    self.pc += 1;
                    let [hb, lb] = self.mem.get_16(self.pc).to_be_bytes(); // get a 16, and only push lb if in 16 bit mode
                    if d {
                        self.push(lb, r); // lb
                        self.pc += 1
                    }
                    self.push(hb, r)
                }
                else {
                    match instr {
                        0x20 => self.st |= 1, // SEC
                        0x40 => self.st &= !1, // CLC
                        0x60 => self.push(0, false), // EXT
                        _ => {} // NOP
                    }
                }
            }
            1 => {} // nop
            2 => {}
            
            3..=7 | 0xd => { // stack primitives and STH
                if instr == 0x83 { // RTI
                    self.st = self.pop(false);
                    self.pc = self.pop_16(true)
                }
                else {
                    if d {
                        let c = self.pop_16(r);
                        let b = self.pop_16(r);
                        let a = self.pop_16(r);
                        match op {
                            3 => { // POP
                                self.push_16(b, r); self.push_16(a, r);
                            }
                            4 => { // SWP
                                self.push_16(b, r); self.push_16(c, r); self.push_16(a, r);
                            }
                            5 => { // ROT
                                self.push_16(c, r); self.push_16(a, r); self.push_16(b, r);
                            }
                            6 => { // DUP
                                self.push_16(c, r); self.push_16(c, r); self.push_16(b, r); self.push_16(a, r);
                            }
                            7 => { // OVR
                                self.push_16(b, r); self.push_16(c, r); self.push_16(b, r); self.push_16(a, r);
                            }
                            0xd => { // STH
                                self.push_16(a, r); self.push_16(b, r); self.push_16(c, !r);
                            }
                            _ => unreachable!()
                        }
                    }
                    else {
                        let c = self.pop(r);
                        let b = self.pop(r);
                        let a = self.pop(r);
                        match op {
                            3 => { // POP
                                self.push(b, r); self.push(a, r);
                            }
                            4 => { // SWP
                                self.push(b, r); self.push(c, r); self.push(a, r);
                            }
                            5 => { // ROT
                                self.push(c, r); self.push(a, r); self.push(b, r);
                            }
                            6 => { // DUP
                                self.push(c, r); self.push(c, r); self.push(b, r); self.push(a, r);
                            }
                            7 => { // OVR
                                self.push(b, r); self.push(c, r); self.push(b, r); self.push(a, r);
                            }
                            0xd => { // STH
                                self.push(a, r); self.push(b, r); self.push(c, !r);
                            }
                            _ => unreachable!()
                        }
                    }
                }
            }

            8 | 9 => { // EQU and GTH
                if d {
                    let (a, b) = if k {
                        let a = self.pick_16(0, r);
                        let b = self.pick_16(2, r);
                        (a, b)
                    }
                    else {
                        let a = self.pop_16(r);
                        let b = self.pop_16(r);
                        (a, b)
                    };
                    if op == 8 {
                        self.push((a == b) as u8, r) 
                    }
                    else {
                        self.push((a as i16 > b as i16) as u8, r) 
                    }
                }
                else {
                    let (a, b) = if k {
                        let a = self.pick(0, r);
                        let b = self.pick(1, r);
                        (a, b)
                    }
                    else {
                        let a = self.pop(r);
                        let b = self.pop(r);
                        (a, b)
                    };
                    if op == 8 {
                        self.push((a == b) as u8, r) 
                    }
                    else {
                        self.push((b as i8 > a as i8) as u8, r) 
                    }
                }
            }

            0xa..=0xc => { // jumps
                if d {
                    let addr = self.pop_16(r);
                    
                    let will_jump = match op {
                        0xb => {
                            let cond = if k {
                                self.pick(1, r)
                            }
                            else {
                                self.pop(r)
                            };
                            cond != 0 // jump not zero
                        }
                        _  => true
                    };
                    if op == 0xc {
                        self.push_16(self.pc, !r)
                    }
                    
                    if will_jump {
                        self.pc = addr
                    }
                }
                else { // rel jumps
                    let mut ofs = if k {
                        self.pick(0, r)
                    }
                    else {
                        self.pop(r)
                    };
                    
                    let will_jump = match op {
                        0xb => {
                            let cond = if k {
                                self.pick(1, r)
                            }
                            else {
                                self.pop(r)
                            };
                            cond != 0 // jump not zero
                        }
                        _  => true
                    };
                    if op == 0xc {
                        self.push_16(self.pc, !r)
                    }
                    
                    let dest = self.get_pc_offset(ofs); 
                    if will_jump {
                        self.pc = dest
                    }
                }
            }

            0x10..=0x15 => { // memory accessing
                let addr = match op {
                    0x10..=0x11 => { // zpg
                        self.pop(r) as u16
                    }
                    0x12..=0x13 => { // rel
                        let ofs = self.pop(r);
                        self.get_pc_offset(ofs)
                    }
                    _ => { // absolute
                        self.pop_16(r)
                    }
                };
                // all even values are load
                if op & 0b1 == 0 { // load
                    let [hb, lb] = self.mem.get_16(addr).to_be_bytes();
                    if d {
                        self.push(lb, r)
                    }
                    self.push(hb, r)
                }
                else { // store
                    if d {
                        let v = self.pop_16(r);
                        self.mem.set_16(addr, v)
                    }
                    else {
                        let v = self.pop(r);
                        self.mem.set(addr, v)
                    }
                }
            }

            0x16..=0x17 => { // PIC and PUT
                let ofs = self.pop(r);
                if d {
                    if op == 0x16 { // PIC
                        let v = self.pick_16(ofs, r);
                        self.push_16(v, r)
                    }
                    else { // PUT
                        let v = self.pop_16(r);
                        self.put_16(v, ofs, r)
                    }
                }
                else {
                    if op == 0x16 { // PIC
                        let v = self.pick(ofs, r);
                        self.push(v, r)
                    }
                    else { // PUT
                        let v = self.pop(r);
                        self.put(v, ofs, r)
                    }
                }
            }

            0x18..=0x1e => { // arithmetic
                if d {
                    let (a, b) = if k {
                        let a = self.pick_16(0, r);
                        let b = self.pick_16(2, r);
                        (a, b)
                    }
                    else {
                        let a = self.pop_16(r);
                        let b = self.pop_16(r);
                        (a, b)
                    };
                    let x = match op {
                        0x18 => { // ADC
                            let c = self.st & 1; // get carry flag
                            let x = a + b;
                            if a > x { // test for overflow
                                self.st |= 1 // set carry
                            }
                            else {
                                self.st &= !1 // clear carry
                            }
                            x + c.into() // add carry at the end
                        }
                        0x19 => { // SBC
                            let c = !self.st & 1; // get borrow flag
                            let x = b - a;
                            if a < x { // test for underflow
                                self.st &= !1 // clear carry = set borrow
                            }
                            else {
                                self.st |= 1 // set carry = clear borrow
                            }
                            x - c.into()
                        }
                        0x1a => a * b, // MUL
                        0x1b => { // DVM
                            self.push_16(b / a, r);
                            b % a
                        }
                        0x1c => a & b, // AND
                        0x1d => a | b, // IOR
                        0x1e => a ^ b, // XOR
                        _ => unreachable!()
                    };
                    self.push_16(x, r)
                }
                else {
                    let (a, b) = if k {
                        let a = self.pick(0, r);
                        let b = self.pick(1, r);
                        (a, b)
                    }
                    else {
                        let a = self.pop(r);
                        let b = self.pop(r);
                        (a, b)
                    };
                    let x = match op {
                        0x18 => { // ADC
                            let c = self.st & 1; // get carry flag
                            let x = a + b;
                            if a > x { // test for overflow
                                self.st |= 1 // set carry
                            }
                            else {
                                self.st &= !1 // clear carry
                            }
                            x + c // add carry at the end
                        }
                        0x19 => { // SBC
                            let c = !self.st & 1; // get borrow flag
                            let x = b - a;
                            if a < x { // test for underflow
                                self.st &= !1 // clear carry = set borrow
                            }
                            else {
                                self.st |= 1 // set carry = clear borrow
                            }
                            x - c
                        }
                        0x1a => a * b, // MUL
                        0x1b => { // DVM
                            self.push(b / a, r);
                            b % a
                        }
                        0x1c => a & b, // AND
                        0x1d => a | b, // IOR
                        0x1e => a ^ b, // XOR
                        _ => unreachable!()
                    };
                    self.push(x, r)
                }
            }

            0x1f => { // SFT
                let sft_amt = self.pop(r);
                let ls = (sft_amt & 0xf0) >> 4;
                let rs = sft_amt & 0x0f;
                if d {
                    let mut v = self.pop_16(r);
                    v <<= ls;
                    v >>= rs;
                    self.push_16(v, r)
                }
                else {
                    let mut v = self.pop(r);
                    v <<= ls;
                    v >>= rs;
                    self.push(v, r)
                }
            }

            _ => {} // nop
        }

        self.pc += 1;
    }

    fn get_pc_offset(&self, ofs: u8) -> u16 {
        let ofs = ofs as i8;
        let neg = ofs < 0;
        if neg {
            (ofs * -1) as u16
        }
        else {
            ofs as u16
        }
    }

    // internal stack manipulation
    // because the stack is essentially backwards (grows down)
    // when pushing a u16, you push lb first
    // so that when reading it, the endianness is right
    // when popping a u16, pop hb first

    #[wrappit]
    fn push(&mut self, val: u8, is_rst: bool) {
        println!("PUSHING {:02x}", val);
        let idx = if is_rst {
            let idx = (self.rsp as u16) + RST_START;
            self.rsp -= 1;
            idx
        }
        else {
            let idx = (self.wsp as u16) + WST_START;
            self.wsp -= 1;
            idx
        };
        self.mem.set(idx, val);
    }
    #[wrappit]
    fn pop(&mut self, is_rst: bool) -> u8 {
        let idx = if is_rst {
            self.rsp += 1;
            (self.rsp as u16) + RST_START
        }
        else {
            self.wsp += 1;
            (self.wsp as u16) + WST_START
        };
        self.mem.get(idx)
    }
    fn push_16(&mut self, val: u16, is_rst: bool) {
        let [hb, lb] = val.to_be_bytes();
        self.push(lb, is_rst);
        self.push(hb, is_rst);
    }
    fn pop_16(&mut self, is_rst: bool) -> u16 {
        let hb = self.pop(is_rst);
        let lb = self.pop(is_rst);
        u16::from_be_bytes([hb, lb])
    }

    /// x_ofs ... x_1 x_0 -- val ... x_1 x_0
    #[wrappit]
    fn put(&mut self, val: u8, ofs: u8, is_rst: bool) {
        let idx = if is_rst {
            ((self.rsp + ofs + 1) as u16) + RST_START
        }
        else {
            ((self.wsp + ofs + 1) as u16) + WST_START
        };
        self.mem.set(idx, val)
    }
    #[wrappit]
    fn pick(&mut self, ofs: u8, is_rst: bool) -> u8 {
        let idx = if is_rst {
            ((self.rsp + ofs + 1) as u16) + RST_START
        }
        else {
            ((self.wsp + ofs + 1) as u16) + WST_START
        };
        self.mem.get(idx)
    }
    /// hb is at ofs, lb is at ofs + 1
    #[wrappit]
    fn put_16(&mut self, val: u16, ofs: u8, is_rst: bool) {
        let [hb, lb] = val.to_be_bytes();
        self.put(hb, ofs, is_rst);
        self.put(lb, ofs + 1, is_rst)
    }
    #[wrappit]
    fn pick_16(&mut self, ofs: u8, is_rst: bool) -> u16 {
        let hb = self.pick(ofs, is_rst);
        let lb = self.pick(ofs + 1, is_rst);
        u16::from_be_bytes([hb, lb])
    }
}
