use std::fs;

#[allow(dead_code)]
pub struct Chip8 {
    opcode: u16,
    v: Vec<u8>,
    i: u16,
    pc: usize,
    stack: Vec<u16>,
    sp: usize,
    ram: Vec<u8>,
    vram: Vec<u8>,
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
            stack: vec![0; 16],
            sp: 0,
            ram: vec![0; 4096],
            vram: vec![0; 32 * 64],
            redraw: false,
            keys: vec![0; 16],
            delay_t: 0,
            sound_t: 0,
        }
    }

    /// The method loads the default font from ram[0] to ram[80] and
    /// the game rom from ram[512] onwards.

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
            self.ram[i] = char_table[i]
        } 

        let rom = fs::read(rom).expect("Cannot read rom file!");
        for i in 0..rom.len() {
            self.ram[0x200 + i] = rom[i];
        }
    }

    pub fn clock(&mut self) {
        self.opcode = (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc+1] as u16);
        println!("{:X}", self.opcode);
    }

    pub fn dump_vram(&self) -> &[u8] {
        self.vram.as_slice()
    }

    pub fn is_awaiting_redraw(&self) -> bool {
        self.redraw
    }

    pub fn fulfill_redraw(&mut self) {
        self.redraw = false;
    }
}
