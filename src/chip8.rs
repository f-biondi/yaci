use std::fs;
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
    vmem: Vec<u8>,
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
            vmem: vec![0; 32 * 64],
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
            0x0000 =>
                match self.opcode & 0x000F {
                    0x0 => self.screen_clear(),
                    _ => self.subroutine_return(),
                },
            0x1000 => self.jump_to_address(),
            0x2000 => self.call_subroutine(),
            0x3000 => self.skip_if_vx(),
            0x4000 => self.skip_if_not_vx(),
            0x5000 => self.skip_if_vx_equals_vy(),
            0x6000 => self.set_vx(),
            _ => panic!("Invalid opcode"),
        }
    }

    /// OPCODE: 00E0
    ///
    /// The method clears the screen by setting every byte in the vmem to 0 and setting the 
    /// redraw flag to true, the program counter is then incremented by 2
    fn screen_clear(&mut self) {
        self.vmem = vec![0; 64*32];
        self.redraw = true; 
        self.pc += 2;
    }

    /// OPCODE: 00EE
    ///
    /// The method returns from a subroutine, pc is set to the last address in the stack (if the
    /// stack is empty the system halts), the pc is then incremented by 2
    fn subroutine_return(&mut self) {
        self.pc = match self.stack.pop_back() {
            Some(address) => address,
            None => process::exit(0x0),
        };
        self.pc += 2;
    }

    ///OPCODE: 1NNN
    ///
    /// The method jumps to the NNN address, pc is set to NNN
    fn jump_to_address(&mut self) {
        self.pc = self.opcode & 0x0FFF;
        println!("{:X}", self.pc);
    }

    ///OPCODE: 2NNN
    ///
    /// The method calls a subroutine at the address NNN by pushing pc to the stack and 
    /// setting pc equal to NNN
    fn call_subroutine(&mut self) {
        self.stack.push_back(self.pc);
        self.pc = self.opcode & 0x0FFF;
    }

    ///OPCODE: 3XNN
    ///
    /// The method skips the next instruction if v[X] is equal to NN
    fn skip_if_vx(&mut self) {
        if self.v[((self.opcode & 0x0F00) >> 8) as usize] == (self.opcode & 0x00FF) as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }

    ///OPCODE: 4XNN
    ///
    /// The method skips the next instruction if v[X] is not equal to NN
    fn skip_if_not_vx(&mut self) {
        if self.v[((self.opcode & 0x0F00) >> 8) as usize] != (self.opcode & 0x00FF) as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }

    ///OPCODE: 5XY0
    ///
    /// The method skips the next instruction if v[X] is equal to v[y]
    fn skip_if_vx_equals_vy(&mut self) {
        if self.v[((self.opcode & 0x0F00) >> 8) as usize] != self.v[((self.opcode & 0x00F0) >> 4) as usize] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    ///OPCODE: 6XNN
    ///
    /// The method set v[X] equal to NN
    fn set_vx(&mut self) {
        self.v[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
        self.pc += 2;
    }

    pub fn dump_vmem(&self) -> &[u8] {
        self.vmem.as_slice()
    }

    pub fn is_awaiting_redraw(&self) -> bool {
        self.redraw
    }

    pub fn fulfill_redraw(&mut self) {
        self.redraw = false;
    }
}
