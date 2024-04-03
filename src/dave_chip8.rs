use std::io;
use std::time::{Duration, Instant};
use std::thread;
use crossterm::{
	event::{self, Event, KeyCode},
};
use rand;
use tui::{
	backend::Backend,
	style::{Color, Style},
	text::Span,
	widgets::{canvas::Canvas, Block, Borders},
	Frame,
	Terminal,
};

const MEM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const VX_REGISTERS: usize = 16;
const START_LOCATION: usize = 0x200;
const KEYBOARD_SIZE: usize = 16;
const FONT_SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0],
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xF0, 0x10, 0xF0, 0x80, 0xF0],
    [0xF0, 0x10, 0xF0, 0x10, 0xF0],
    [0x90, 0x90, 0xF0, 0x10, 0x10],
    [0xF0, 0x80, 0xF0, 0x10, 0xF0],
    [0xF0, 0x80, 0xF0, 0x90, 0xF0],
    [0xF0, 0x10, 0x20, 0x40, 0x40],
    [0xF0, 0x90, 0xF0, 0x90, 0xF0],
    [0xF0, 0x90, 0xF0, 0x10, 0xF0],
    [0xF0, 0x90, 0xF0, 0x90, 0x90],
    [0xE0, 0x90, 0xE0, 0x90, 0xE0],
    [0xF0, 0x80, 0x80, 0x80, 0xF0],
    [0xE0, 0x90, 0x90, 0x90, 0xE0],
    [0xF0, 0x80, 0xF0, 0x80, 0xF0],
    [0xF0, 0x80, 0xF0, 0x80, 0x80],
];
const SCREEN_HEIGHT: usize = 32;
const SCREEN_WIDTH: usize = 64;

#[allow(non_snake_case)]
pub struct Chip8 {
	memory: [u8; MEM_SIZE],
	Vx: [u8; VX_REGISTERS],
	I: u16,
	ST: u8,
	DT: u8,
	PC: usize,
	SP: usize,
	stack: [usize; STACK_SIZE],
	keyboard: [bool; KEYBOARD_SIZE], // true = pressed, false = not pressed
	monitor: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Chip8 {
	// Create New Chip8 Instance With Everything Set to 0
	fn new() -> Self {
		Self {
			memory: [0u8; MEM_SIZE],
			Vx: [0u8; VX_REGISTERS],
			I: 0,
			ST: 0,
			DT: 0,
			PC: START_LOCATION,
			SP: 0,
			stack: [0; STACK_SIZE],
			keyboard: [false; KEYBOARD_SIZE],
			monitor: [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
		}
	}

	// Inserts Inside Chip8 Memory (Starting From Address 0x200) the Program to Run
	pub fn start(program: &[u8]) -> Self {
		let mut chip_8 = Self::new();
		let mut counter_tmp = 0;

		for i in FONT_SPRITES {
			for j in i {
				chip_8.memory[counter_tmp] = j;
				counter_tmp += 1;
			}
		} // Load Fonts in Memory

		for i in program {
			chip_8.memory[chip_8.PC] = *i;
			chip_8.PC += 1;
		} // Load Program in Memory

		chip_8.PC = START_LOCATION; // Resets PC
		chip_8
	}

	// Removes 1 From the "Timer Register" and "Sound Timer"
	fn update(&mut self) {
		self.DT -= if self.DT > 0 { 1 } else {0};
        self.ST -= if self.ST > 0 { 1 } else {0};
	}

	// Runs Through One Opcode and Returns Run Opcode
	fn execute_next_opcode(&mut self) -> u16 {
		let opcode = u16::from_be_bytes([self.memory[self.PC], self.memory[self.PC + 1]]);

        // Divide Opcode Into Four 4 Bit Values
        let x = ((opcode >> 8) & 0xF) as usize; // Lower 4 Bits of First Byte
        let y = ((opcode >> 4) & 0xF) as usize; // Higher 4 Bits of Second Byte
        let n = (opcode & 0xF) as u8; // Lower 4 Bits of Second Byte
        let nnn = (opcode & 0xFFF) as u16; // Lower 12 Bits of Opcode
        let kk = (opcode & 0xFF) as u8; // Lower Bits 8 of Opcode
        
        match &opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => self._00E0(),
                0x00EE => self._00EE(),
                _ => self.no_opcode_found(),
            },
            0x1000 => self._1nnn(nnn as usize),
            0x2000 => self._2nnn(nnn as usize),
            0x3000 => self._3xkk(x, kk),
            0x4000 => self._4xkk(x, kk),
            0x5000 => self._5xy0(x, y),
            0x6000 => self._6xkk(x, kk),
            0x7000 => self._7xkk(x, kk),
            0x8000 => match opcode & 0xF {
                0x0000 => self._8xy0(x, y),
                0x0001 => self._8xy1(x, y),
                0x0002 => self._8xy2(x, y),
                0x0003 => self._8xy3(x, y),
                0x0004 => self._8xy4(x, y),
                0x0005 => self._8xy5(x, y),
                0x0006 => self._8xy6(x, y),
                0x0007 => self._8xy7(x, y),
                0x000E => self._8xyE(x, y),
                _ => self.no_opcode_found(),
            },
            0x9000 => self._9xy0(x, y),
            0xA000 => self._Annn(nnn),
            0xB000 => self._Bnnn(nnn),
            0xC000 => self._Cxkk(x, kk),
            0xD000 => self._Dxyn(x, y, n as usize),
            0xE000 => match opcode & 0xFF {
                0x009E => self._Ex9E(x),
                0x00A1 => self._ExA1(x),
                _ => self.no_opcode_found(),
            },
            0xF000 => match opcode & 0xFF {
                0x0007 => self._Fx07(x),
                0x000A => self._Fx0A(x),
                0x0015 => self._Fx15(x),
                0x0018 => self._Fx18(x),
                0x001E => self._Fx1E(x),
                0x0029 => self._Fx29(x),
                0x0033 => self._Fx33(x),
                0x0055 => self._Fx55(x),
                0x0065 => self._Fx65(x),
                _ => self.no_opcode_found(),
            },

            _ => self.no_opcode_found(),
        };
        opcode
    }

    // Sets to True the Desired Key and Returns Ok or Error
    pub fn set_key<'a>(&'a mut self,x: usize) -> Result<(),&'a str> {
        *(self.keyboard.get_mut(x).unwrap()) = true;
        Ok(())
    }

    // Sets All of the Keys to Those of the Array Passed in Args
    pub fn set_keys(&mut self,keys: &[bool]) {
        for i in 0..16{
            self.keyboard[i] = keys[i];
        }
    }

    fn get_pixel(&self,x: usize,y: usize) -> u8 {
        self.monitor[y % SCREEN_HEIGHT][x % SCREEN_WIDTH]
    }
}

// Opcodes for Chip8 (No Chip-48 Instructions)
#[allow(non_snake_case)]
impl Chip8 {
    fn no_opcode_found(&mut self) {
        self.PC += 2;
    }

    fn _00E0(&mut self) {
        self.monitor = [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT];
        self.PC += 2;
    }

    fn _00EE(&mut self) {
        self.PC = self.stack[self.SP];
        self.SP -= 1;
    }

    fn _1nnn(&mut self, addr: usize) {
        self.PC = addr;
    }

    fn _2nnn(&mut self, addr: usize) {
        self.PC += 2;
        self.SP += 1;
        self.stack[self.SP] = self.PC;
        self.PC = addr;
    }

    fn _3xkk(&mut self, Vx_reg: usize, kk: u8) {
        if self.Vx[Vx_reg] == kk {
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn _4xkk(&mut self, Vx_reg: usize, kk: u8) {
        if self.Vx[Vx_reg] != kk {
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn _5xy0(&mut self, Vx_reg: usize, Vy_reg: usize) {
        if self.Vx[Vx_reg] == self.Vx[Vy_reg] {
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn _6xkk(&mut self, Vx_reg: usize, kk: u8) {
        self.Vx[Vx_reg] = kk;
        self.PC += 2;
    }

    fn _7xkk(&mut self, Vx_reg: usize, kk: u8) {
        self.Vx[Vx_reg] = self.Vx[Vx_reg].wrapping_add(kk);
        self.PC += 2;
    }

    fn _8xy0(&mut self, Vx_reg: usize, Vy_reg: usize) {
        self.Vx[Vx_reg] = self.Vx[Vy_reg];
        self.PC += 2;
    }

    fn _8xy1(&mut self, Vx_reg: usize, Vy_reg: usize) {
        self.Vx[Vx_reg] |= self.Vx[Vy_reg];
        self.PC += 2;
    }

    fn _8xy2(&mut self, Vx_reg: usize, Vy_reg: usize) {
        self.Vx[Vx_reg] &= self.Vx[Vy_reg];
        self.PC += 2;
    }

    fn _8xy3(&mut self, Vx_reg: usize, Vy_reg: usize) {
        self.Vx[Vx_reg] ^= self.Vx[Vy_reg];
        self.PC += 2;
    }

    fn _8xy4(&mut self, Vx_reg: usize, Vy_reg: usize) {
        let tmp: u16 = self.Vx[Vx_reg] as u16 + self.Vx[Vy_reg] as u16;
        self.Vx[15] = if tmp > 255 { 1 } else { 0 };
        self.Vx[Vx_reg] = (tmp & 0xFF) as u8;
        self.PC += 2;
    }

    fn _8xy5(&mut self, Vx_reg: usize, Vy_reg: usize) {
        self.Vx[15] = if self.Vx[Vx_reg] > self.Vx[Vy_reg] {
            1
        } else {
            0
        };
        self.Vx[Vx_reg] -= self.Vx[Vy_reg];
        self.PC += 2;
    }

    fn _8xy6(&mut self, Vx_reg: usize, _Vy_reg: usize) {
        self.Vx[15] = if (self.Vx[Vx_reg] & 0x1) == 1 { 1 } else { 0 };
        self.Vx[Vx_reg] /= 2;
        self.PC += 2;
    }

    fn _8xy7(&mut self, Vx_reg: usize, Vy_reg: usize) {
        self.Vx[15] = if self.Vx[Vy_reg] > self.Vx[Vx_reg] {
            1
        } else {
            0
        };
        self.Vx[Vx_reg] = self.Vx[Vy_reg] - self.Vx[Vx_reg];
        self.PC += 2;
    }

    fn _8xyE(&mut self, Vx_reg: usize, _Vy_reg: usize) {
        let shifted_register = self.Vx[Vx_reg] << 1;
        self.Vx[15] = if (shifted_register & 0x1) == 1 { 1 } else { 0 };
        self.Vx[Vx_reg] *= 2;
        self.PC += 2;
    }

    fn _9xy0(&mut self, Vx_reg: usize, Vy_reg: usize) {
        if self.Vx[Vx_reg] != self.Vx[Vy_reg] {
            self.PC += 2
        }
        self.PC += 2;
    }

    fn _Annn(&mut self, nnn: u16) {
        self.I = nnn;
        self.PC += 2;
    }

    fn _Bnnn(&mut self, nnn: u16) {
        self.PC = (nnn as usize) + (self.Vx[0] as usize);
    }

    fn _Cxkk(&mut self, Vx_reg: usize, kk: u8) {
        self.Vx[Vx_reg] = kk & rand::random::<u8>();
        self.PC += 2;
    }

    fn _Dxyn(&mut self, Vx_reg: usize, Vy_reg: usize, bytes_to_read: usize) {
        let row = self.Vx[Vy_reg];
        let col = self.Vx[Vx_reg];
        self.Vx[0xF] = 0;

        for i in 0..bytes_to_read {
            let memory_pixel = self.memory[(self.I as usize) + i];

            for j in 0..8 {
                let bit = (memory_pixel >> j) & 0x1;
                let pixel_screen = self.monitor[(row as usize + i) % SCREEN_HEIGHT]
                    [(col as usize + 7 - j) % SCREEN_WIDTH];

                if bit == 1 && pixel_screen == 1 {
                    self.Vx[0xF] = 1;
                }

                self.monitor[(row as usize + i) % SCREEN_HEIGHT]
                    [(col as usize + 7 - j) % SCREEN_WIDTH] ^= bit;
            }
        }

        self.PC += 2;
    }

    fn _Ex9E(&mut self, Vx_reg: usize) {
        if self.keyboard[self.Vx[Vx_reg] as usize] == true {
            self.keyboard[self.Vx[Vx_reg] as usize] = false;
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn _ExA1(&mut self, Vx_reg: usize) {
        if self.keyboard[self.Vx[Vx_reg] as usize] == false {
            self.PC += 2;
        }
        self.PC += 2;
        self.keyboard[self.Vx[Vx_reg] as usize] = false;
    }

    fn _Fx07(&mut self, Vx_reg: usize) {
        self.Vx[Vx_reg] = self.DT;
        self.PC += 2;
    }

    fn _Fx0A(&mut self, Vx_reg: usize) {
        if let Some(k) = self.keyboard.iter().position(|x| *x == true){
            
            self.keyboard[k] = false;
            self.Vx[Vx_reg] = k as u8;
            self.PC += 2;
        }
    }

    fn _Fx15(&mut self, Vx_reg: usize) {
        self.DT = self.Vx[Vx_reg];
        self.PC += 2;
    }

    fn _Fx18(&mut self, Vx_reg: usize) {
        self.ST = self.Vx[Vx_reg];
        self.PC += 2;
    }

    fn _Fx1E(&mut self, Vx_reg: usize) {
        self.I = self.I.wrapping_add(self.Vx[Vx_reg] as u16);
        self.PC += 2;
    }

    fn _Fx29(&mut self, Vx_reg: usize) {
        self.I = (5 * self.Vx[Vx_reg]) as u16;
        self.PC += 2;
    }

    #[allow(unused_parens)]
    fn _Fx33(&mut self, Vx_reg: usize) {
        self.memory[self.I as usize] = (((self.Vx[Vx_reg] as i32) % 1000) / 100) as u8;
        self.memory[(self.I + 1) as usize] = (self.Vx[Vx_reg] % 100) / 10;
        self.memory[(self.I + 2) as usize] = (self.Vx[Vx_reg] % 10);
        self.PC += 2;
    }

    fn _Fx55(&mut self, Vx_reg: usize) {
        let mut tmp = self.I as usize;

        for i in 0..=Vx_reg {
            self.memory[tmp] = self.Vx[i];
            tmp += 1;
        }
        self.PC += 2;
    }

    fn _Fx65(&mut self, Vx_reg: usize) {
        let mut tmp = self.I as usize;

        for i in 0..=Vx_reg {
            self.Vx[i] = self.memory[tmp];
            tmp += 1;
        }
        self.PC += 2;
    }
}

fn ui<B>(
    f: &mut Frame<B>,
    chip8: &Chip8,
    opcodes_per_cycle: u8,
    timer_hz: u8,
    screen_height: usize,
    screen_width: usize,
) where
    B: Backend,
{
    let title = format!(
        "| David's Chip8 Emulator | Opcodes Per Cycle: {} | Timer Between CPU Cycles: {} Hz | Press (Esc) to Exit |",
        opcodes_per_cycle, timer_hz
    );
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title.to_owned()),
        )
        .paint(|ctx| {
            for y in 0..SCREEN_HEIGHT {
                for x in 0..SCREEN_WIDTH {
                    if chip8.get_pixel(x, y) == 1 {
                        ctx.print(
                            x as f64,
                            (screen_height - y) as f64,
                            Span::styled("█", Style::default().fg(Color::White)),
                        );
                    }
                }
            }
        })
        .x_bounds([0.0, screen_width as f64])
        .y_bounds([0.0, screen_height as f64]);

    f.render_widget(canvas, f.size());
}

pub fn run_dave_chip8_emulator<B>(terminal: &mut Terminal<B>, chip8: &mut Chip8) -> io::Result<()>
where
	B: Backend,
{
	let mut previous_instant = Instant::now();
	let mut opcodes_per_cycle: u8 = 8;
	let mut timer_hz: u8 = 60;
	let mut screen_height: usize = SCREEN_HEIGHT;
	let mut screen_width: usize = SCREEN_WIDTH;

	// Keyboard Events, Updating Timers, Executing Opcode, Drawing
	loop {
		// There is a little bit of "loss of time" since the method "from_millis" accepts u64 values only
        thread::sleep(Duration::from_millis(
        	((1.0 / timer_hz as f64) * 1000.0) as u64,
        ));
        chip8.update();

        for _i in 0..opcodes_per_cycle {
            if event::poll(Duration::from_micros(1))? {
                if let Event::Key(key) = event::read()? {
                    let _ = match key.code {
                        // Chip8 Commands
                        KeyCode::Char('w') => chip8.set_key(5),
                        KeyCode::Char('a') => chip8.set_key(7),
                        KeyCode::Char('s') => chip8.set_key(8),
                        KeyCode::Char('d') => chip8.set_key(9),
                        KeyCode::Char('q') => chip8.set_key(4),
                        KeyCode::Char('e') => chip8.set_key(6),
                        KeyCode::Char('1') => chip8.set_key(1),
                        KeyCode::Char('2') => chip8.set_key(2),
                        KeyCode::Char('3') => chip8.set_key(3),
                        KeyCode::Char('4') => chip8.set_key(12),
                        KeyCode::Char('x') => chip8.set_key(0),
                        KeyCode::Char('z') => chip8.set_key(10),
                        KeyCode::Char('c') => chip8.set_key(11),
                        KeyCode::Char('r') => chip8.set_key(13),
                        KeyCode::Char('f') => chip8.set_key(14),
                        KeyCode::Char('v') => chip8.set_key(15),
                        // New Emulator Commands
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Up => {
                            opcodes_per_cycle += if opcodes_per_cycle < u8::MAX { 1 } else { 0 };
                            Ok(())
                        }
                        KeyCode::Down => {
                            opcodes_per_cycle -= if opcodes_per_cycle > 1 { 1 } else { 0 };
                            Ok(())
                        }
                        KeyCode::Right => {
                            timer_hz += if timer_hz < u8::MAX { 1 } else { 0 };
                            Ok(())
                        }
                        KeyCode::Left => {
                            timer_hz -= if timer_hz > 1 { 1 } else { 0 };
                            Ok(())
                        }
                        KeyCode::Char('l') => {
                            screen_height = if screen_height == SCREEN_HEIGHT {
                                terminal.size()?.height as usize
                            } else {
                                SCREEN_HEIGHT
                            };
                            screen_width = if screen_width == SCREEN_WIDTH {
                                terminal.size()?.width as usize
                            } else {
                                SCREEN_WIDTH
                            };
                            Ok(())
                        }
                        _ => Ok(()),
                    };
                }
            }

        	if timer_hz > 105 {
        		return Ok(());
        	}

        	chip8.execute_next_opcode();

        	terminal
        		.draw(|f| {
        			ui(
        				f,
        				&chip8,
        				opcodes_per_cycle,
        				timer_hz,
        				screen_height,
        				screen_width,
        			)
        		})
        		.expect("It was not possible to draw any more");

        	if (Instant::now() - previous_instant).as_millis() >= 16 {
        		previous_instant = Instant::now();
        	}
        }
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn chip_8_testing_emulation() {
		use crate::dave_chip8::Chip8;
		use std::fs::File;
		use std::io::*;

		let mut file = File::options()
			.read(true)
			.create(false)
			.open("./dave_conf/var/daves_roms/TETRIS")
			.expect("Unable to Open File");

		let mut file_contents: Vec<u8> = Vec::new();
		println!("FILE CONTENTS 1: {:?}", file_contents);
		let data = file.read_to_end(&mut file_contents).expect("Unable to Read File");
		println!("FILE CONTENTS 2: {}", data);

		let mut chip_8 = Chip8::start(&file_contents[..]);

		while data > 0 {
			chip_8.execute_next_opcode();
			println!();
		}
	}
}
