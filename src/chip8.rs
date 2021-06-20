use std::fs;
use rand::Rng;
use std::process;
use std::collections::VecDeque;

#[allow(dead_code)]
pub struct Chip8 {
    opcode: u16,
    v: Vec<u8>,
    i: u16,
    pc: u16,
    stack: VecDeque<u16>,
    mem: Vec<u8>,
    vmem: Vec<Vec<u8>>,
    redraw: bool,
    keys: Vec<u8>,
    delay_t: u8,
    sound_t: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            opcode: 0,
            v: vec![0; 16],
            i: 0,
            pc: 0x200, //rom program starts at 0x200 memory address (512 in decimal)
            stack: VecDeque::with_capacity(16),
            mem: vec![0; 4096],
            vmem: vec![vec![0; 64]; 32],
            redraw: false,
            keys: vec![0; 16],
            delay_t: 0,
            sound_t: 0,
        }
    }

    /// The method loads the default font from mem[0] to mem[80] and
    /// the game rom from mem[512] onwards.
    pub fn load_rom(&mut self, rom: &String) {
        let char_table = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        for i in 0..char_table.len() {
            self.mem[i] = char_table[i]
        } 

        let rom = fs::read(rom).expect("Cannot read rom file!");
        for i in 0..rom.len() {
            self.mem[0x200 + i] = rom[i];
        }

    }

    /// The method represent a cpu clock, it fetches the 2 byte opcode at pc
    /// mem location and calls the decode_opcode() method to decode it, the
    /// method then proceds decreasing the sound and delay timer by 1 if they have a value
    /// greater than 0, if the sound timer have a value equal to one a beep is
    /// reproduced
    pub fn clock(&mut self) {
        self.opcode = (self.mem[self.pc as usize] as u16) << 8 | (self.mem[(self.pc+1) as usize] as u16);

        self.decode_opcode(); 

        if self.delay_t > 0 {
            self.delay_t -= 1;
        }

        if self.sound_t > 0 {
            if self.sound_t == 1 {
                println!("beep");
            }
            self.sound_t -= 1;
        }
    }

    fn decode_opcode(&mut self) {
        println!("{:X}", self.opcode);
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x000F {
                0x0 => self.screen_clear(),
                _ => self.subroutine_return(),
            },
            0x1000 => self.jump_to_address(),
            0x2000 => self.call_subroutine(),
            0x3000 => self.skip_if_vx(),
            0x4000 => self.skip_if_not_vx(),
            0x5000 => self.skip_if_vx_equals_vy(),
            0x6000 => self.set_vx(),
            0x7000 => self.add_vx_no_carry(),
            0x8000 => match self.opcode & 0x000F {
                0x0 => self.set_vx_to_vy(), 
                0x1 => self.set_vx_or_vy(), 
                0x2 => self.set_vx_and_vy(), 
                0x3 => self.set_vx_xor_vy(), 
                0x4 => self.add_vy_to_vx_carry(), 
                0x5 => self.subtract_vy_from_vx_borrow(), 
                0x6 => self.vx_store_least_and_right_shift(), 
                0x7 => self.set_vx_to_vy_minus_vx(), 
                _ => self.vx_store_most_and_left_shift(), 
            },
            0x9000 => self.skip_if_vx_not_equals_vy(),
            0xA000 => self.set_i(),
            0xB000 => self.jump_to_v0_plus_nnn(),
            0xC000 => self.rand_xor(),
            0xD000 => self.draw(),
            _ => loop {},
        }
    }

    /// OPCODE: 00E0
    ///
    /// The method clears the screen by setting every byte in the vmem to 0 and setting the 
    /// redraw flag to true, the program counter is then incremented by 2
    fn screen_clear(&mut self) {
        self.vmem = vec![vec![0; 64]; 32];
        self.redraw = true; 
        self.pc += 2;
    }

    /// OPCODE: 00EE
    ///
    /// The method returns from a subroutine, pc is set to the last address in the stack (if the
    /// stack is empty the system halts), pc is then incremented by 2
    fn subroutine_return(&mut self) {
        self.pc = match self.stack.pop_back() {
            Some(address) => address,
            None => process::exit(0x0),
        };
        self.pc += 2;
    }

    /// OPCODE: 1NNN
    ///
    /// The method jumps to the NNN address, pc is set to NNN
    fn jump_to_address(&mut self) {
        self.pc = self.opcode & 0x0FFF;
    }

    /// OPCODE: 2NNN
    ///
    /// The method calls a subroutine at the address NNN by pushing pc to the stack and 
    /// setting pc equal to NNN
    fn call_subroutine(&mut self) {
        self.stack.push_back(self.pc);
        self.pc = self.opcode & 0x0FFF;
    }

    /// OPCODE: 3XNN
    ///
    /// The method skips the next instruction if v[X] is equal to NN
    fn skip_if_vx(&mut self) {
        if self.v[((self.opcode & 0x0F00) >> 8) as usize] == (self.opcode & 0x00FF) as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }

    /// OPCODE: 4XNN
    ///
    /// The method skips the next instruction if v[X] is not equal to NN
    fn skip_if_not_vx(&mut self) {
        if self.v[((self.opcode & 0x0F00) >> 8) as usize] != (self.opcode & 0x00FF) as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }

    /// OPCODE: 5XY0
    ///
    /// The method skips the next instruction if v[X] is equal to v[y]
    fn skip_if_vx_equals_vy(&mut self) {
        if self.v[((self.opcode & 0x0F00) >> 8) as usize] == self.v[((self.opcode & 0x00F0) >> 4) as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    /// OPCODE: 6XNN
    ///
    /// The method sets v[X] equal to NN
    fn set_vx(&mut self) {
        self.v[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
        self.pc += 2;
    }

    /// OPCODE: 7XNN
    ///
    /// The method adds NN to v[X] without setting the carry flag
    fn add_vx_no_carry(&mut self) {
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        self.v[x] = self.v[x].wrapping_add((self.opcode & 0x00FF) as u8);
        self.pc += 2;
    }

    /// OPCODE: 8XY0
    ///
    /// The method sets v[X] equal to v[Y]
    fn set_vx_to_vy(&mut self) {
        self.v[((self.opcode & 0x0F00) >> 8) as usize] = self.v[((self.opcode & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }

    /// OPCODE: 8XY1
    ///
    /// The method sets v[X] equal to v[X] or v[Y] (bitwise)
    fn set_vx_or_vy(&mut self) {
        self.v[((self.opcode & 0x0F00) >> 8) as usize] |= self.v[((self.opcode & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }

    /// OPCODE: 8XY2
    ///
    /// The method sets v[X] equal to v[X] and v[Y] (bitwise)
    fn set_vx_and_vy(&mut self) {
        self.v[((self.opcode & 0x0F00) >> 8) as usize] &= self.v[((self.opcode & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }

    /// OPCODE: 8XY3
    ///
    /// The method sets v[X] equal to v[X] xor v[Y] (bitwise)
    fn set_vx_xor_vy(&mut self) {
        self.v[((self.opcode & 0x0F00) >> 8) as usize] ^= self.v[((self.opcode & 0x00F0) >> 4) as usize];
        self.pc += 2;
    }

    /// OPCODE: 8XY4
    ///
    /// The method adds v[Y] to v[X] and sets V[0xF] if the operation results in a carry
    fn add_vy_to_vx_carry(&mut self) {
        let x : usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y : usize = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.v[x] > (0xFF - self.v[y]) {
            self.v[0xF] = 1;
        }
        else {
            self.v[0xF] = 0;
        }

        self.v[x] = self.v[x].wrapping_add(self.v[y]);
        self.pc += 2;
    }

    /// OPCODE: 8XY5
    ///
    /// The method subtracts v[Y] from v[X] and sets V[0xF] if the operation does not result in a borrow
    fn subtract_vy_from_vx_borrow(&mut self) {
        let x : usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y : usize = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.v[x] < self.v[y] {
            self.v[0xF] = 0;
        }
        else {
            self.v[0xF] = 1;
        }

        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.pc += 2;
    }

    /// OPCODE: 8XY6
    ///
    /// The method sets v[0xF] equal to the least significant bit of v[X] and shifts v[X] by one
    /// bit to the right 
    fn vx_store_least_and_right_shift(&mut self) {
        let x : usize = ((self.opcode & 0x0F00) >> 8) as usize;

        self.v[0xF] = self.v[x] & 0x01;
        self.v[x] >>= 1;

        self.pc += 2;
    }

    /// OPCODE: 8XY7
    ///
    /// The method sets v[X] equal to v[Y] - V[X] and sets V[0xF] if the operation does not
    /// result in a borrow
    fn set_vx_to_vy_minus_vx(&mut self) {
        let x : usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y : usize = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.v[y] < self.v[x] {
            self.v[0xF] = 0;
        }
        else {
            self.v[0xF] = 1;
        }

        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.pc += 2;
    }

    /// OPCODE: 8XYE
    ///
    /// The method sets v[0xF] equal to the most significant bit of v[X] and shifts v[X] by one
    /// bit to the left 
    fn vx_store_most_and_left_shift(&mut self) {
        let x : usize = ((self.opcode & 0x0F00) >> 8) as usize;

        self.v[0xF] = (self.v[x] & 0x80) >> 7;
        self.v[x] <<= 1;

        self.pc += 2;
    }

    /// OPCODE: 9XY0
    ///
    /// The method skips the next instruction if v[X] is not equal to v[y]
    fn skip_if_vx_not_equals_vy(&mut self) {
        if self.v[((self.opcode & 0x0F00) >> 8) as usize] != self.v[((self.opcode & 0x00F0) >> 4) as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    /// OPCODE: ANNN
    ///
    /// The method sets i equal to the address NNN
    fn set_i(&mut self) {
        self.i = self.opcode & 0x0FFF;
        self.pc += 2;
    }

    /// OPCODE: BNNN
    ///
    /// The method jumps to v[0] plus NNN address
    fn jump_to_v0_plus_nnn(&mut self) {
        self.pc = self.v[0] as u16 + (self.opcode & 0x0FFF) as u16;
    }

    /// OPCODE: CXNN
    ///
    /// The method sets v[X] equal to a random number xor NN
    fn rand_xor(&mut self) {
        let mut rng = rand::thread_rng();
        let x = ((self.opcode & 0x0F00) >> 8) as usize;
        let rand_u8 : u8 = rng.gen();
        self.v[x] = rand_u8 ^ (self.opcode & 0x00FF) as u8;
        self.pc += 2;
    }

    /// OPCODE: DXYN
    ///
    /// The method draws a sprite at the coordinates v[X], v[Y], the sprite has a width of 8 pixels
    /// and an height of N+1 pixels, sprite data is read from i address onwards V[0xF] is set if a
    /// pixels is unset during the operation.   
    fn draw(&mut self) {
        let x : usize = ((self.opcode & 0x0F00) >> 8) as usize;
        let y : usize = ((self.opcode & 0x00F0) >> 4) as usize;
        let height : u8 = (self.opcode & 0x000F) as u8;
        self.v[0xF] = 0;
        self.redraw = true;
       
        for l in 0..height {
            let line_pixels = self.mem[(self.i + l as u16) as usize];
            for c in 0u8..8u8 {
                let pixel = (line_pixels & (0x80 / u8::pow(2, c as u32))) >> 7 - c;
                let screen_y = ((self.v[y] + l) % 32) as usize;
                let screen_x = ((self.v[x] + c) % 64) as usize;
                if pixel == 1 {
                    if self.vmem[screen_y][screen_x] == 1 {
                        self.v[0xF] = 1;
                    }
                    self.vmem[screen_y][screen_x] ^= pixel;
                }
            }
        }

        self.pc += 2;
    }

    pub fn dump_vmem_line(&self, line_number: usize) -> &[u8] {
        self.vmem[line_number].as_slice()
    }

    pub fn is_awaiting_redraw(&self) -> bool {
        self.redraw
    }

    pub fn fulfill_redraw(&mut self) {
        self.redraw = false;
    }
}
