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

const MEM_SIZE: usize = 400;
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

pub struct Chip8 {
	memory: [u8; MEM_SIZE],
	vx: [u8; VX_REGISTERS],
	i: u16,
	st: u8,
	dt: u8,
	pc: usize,
	sp: usize,
	stack: [usize; STACK_SIZE],
	keyboard: [bool; KEYBOARD_SIZE], // true = pressed, false = not pressed
	monitor: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Chip8 {
	// Create New Chip8 Instance With Everything Set to 0
	fn new() -> Self {
		Self {
			memory: [0u8; MEM_SIZE],
			vx: [0u8; VX_REGISTERS],
			i: 0,
			st: 0,
			dt: 0,
			pc: START_LOCATION,
			sp: 0,
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
			chip_8.memory[chip_8.pc] = *i;
			chip_8.pc += 1;
		} // Load Program in Memory

		chip_8.pc = START_LOCATION; // Resets PC
		chip_8
	}

	// Removes 1 From the "Timer Register" and "Sound Timer"
	fn update(&mut self) {
		self.dt -= if self.dt > 0 { 1 } else { 0 };
		self.st -= if self.st > 0 { 1 } else { 0 };
	}

	// Runs Through One Opcode and Returns Run Opcode
	fn execute_next_opcode(&mut self) -> u16 {
		let opcode = u16::from_be_bytes([self.memory[self.pc], self.memory[self.pc + 1]]);

		// Divide Opcode Into Four 4-Bit Values
		let x = ((opcode >> 8) & 0xF) as usize; // Lower 4 Bits of First Byte
		let y = ((opcode >> 4) & 0xF) as usize; // Higher 4 Bits of Second Byte
		let n = (opcode & 0xF) as u8; // Lower 4 Bits of Second Byte
		let nnn = (opcode & 0xFFF) as u16; // Lower 12 Bits of Opcode
		let kk = (opcode & 0xFF) as u8; // Lower 8 Bits of Opcode

		match &opcode & 0xF000 {
			0x0000 => match opcode {
				0x00E0 => self._00_e0(),
				0x00EE => self._00_ee(),
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
                0x000E => self._8xy0(x, y),
                _ => self.no_opcode_found(),
            },
            0x9000 => self._9xy0(x, y),
            0xA000 => self._annn(nnn),
            0xB000 => self._bnnn(nnn),
            0xC000 => self._cxkk(x, kk),
            0xD000 => self._dxyn(x, y, n as usize),
            0xE000 => match opcode & 0xFF {
                0x009E => self._ex9_e(x),
                0x00A1 => self._ex_a1(x),
                _ => self.no_opcode_found(),
            },
            0xF000 => match opcode & 0xFF {
                0x0007 => self._fx07(x),
                0x000A => self._fx0_a(x),
                0x0015 => self._fx15(x),
                0x0018 => self._fx18(x),
                0x001E => self._fx15(x),
                0x0029 => self._fx29(x),
                0x0033 => self._fx33(x),
                0x0055 => self._fx55(x),
                0x0065 => self._fx65(x),
                _ => self.no_opcode_found(),
            },

            _ => self.no_opcode_found(),
        };
        
        opcode
    }

    // Sets to True the Desired Key and Returns Ok or Error
    fn set_key<'a>(&'a mut self, x: usize) -> Result<(), &'a str> {
    	*(self.keyboard.get_mut(x).unwrap()) = true;
    	Ok(())
    }

    // Sets All of the Keys to Those of the Array Passed in Args
    pub fn set_keys(&mut self, keys: &[bool]) {
    	for i in 0..16 {
    		self.keyboard[i] = keys[i];
    	}
    }

    fn get_pixel(&self, x: usize, y: usize) -> u8 {
    	self.monitor[y % SCREEN_HEIGHT][x % SCREEN_WIDTH]
    }
}

// Opcodes for Chip8 (No Chip-48 Instructions)
impl Chip8 {
	fn no_opcode_found(&mut self) {
		self.pc += 2;
	}

	fn _00_e0(&mut self) {
		self.pc = self.stack[self.sp];
		self.sp -= 1;
	}

	fn _00_ee(&mut self) {
		self.pc = self.stack[self.sp];
		self.sp -= 1;
	}

	fn _1nnn(&mut self, addr: usize) {
		self.pc = addr;
	}

	fn _2nnn(&mut self, addr: usize) {
		self.pc += 2;
		self.sp += 1;
		self.stack[self.sp] = self.pc;
		self.pc = addr;
	}

	fn _3xkk(&mut self, vx_reg: usize, kk: u8) {
		if self.vx[vx_reg] == kk {
			self.pc += 2;
		}
		self.pc += 2;
	}

	fn _4xkk(&mut self, vx_reg: usize, kk: u8) {
		if self.vx[vx_reg] != kk {
			self.pc += 2;
		}
		self.pc += 2;
	}

	fn _5xy0(&mut self, vx_reg: usize, vy_reg: usize) {
		if self.vx[vx_reg] == self.vx[vy_reg] {
			self.pc += 2;
		}
		self.pc += 2;
	}

	fn _6xkk(&mut self, vx_reg: usize, kk: u8) {
		self.vx[vx_reg] = kk;
		self.pc += 2;
	}

	fn _7xkk(&mut self, vx_reg: usize, kk: u8) {
        self.vx[vx_reg] = self.vx[vx_reg].wrapping_add(kk);
        self.pc += 2;
    }

    fn _8xy0(&mut self, vx_reg: usize, vy_reg: usize) {
        self.vx[vx_reg] = self.vx[vy_reg];
        self.pc += 2;
    }

    fn _8xy1(&mut self, vx_reg: usize, vy_reg: usize) {
        self.vx[vx_reg] |= self.vx[vy_reg];
        self.pc += 2;
    }

    fn _8xy2(&mut self, vx_reg: usize, vy_reg: usize) {
        self.vx[vx_reg] &= self.vx[vy_reg];
        self.pc += 2;
    }

    fn _8xy3(&mut self, vx_reg: usize, vy_reg: usize) {
        self.vx[vx_reg] ^= self.vx[vy_reg];
        self.pc += 2;
    }

    fn _8xy4(&mut self, vx_reg: usize, vy_reg: usize) {
        let tmp: u16 = self.vx[vx_reg] as u16 + self.vx[vy_reg] as u16;
        self.vx[15] = if tmp > 255 { 1 } else { 0 };
        self.vx[vx_reg] = (tmp & 0xFF) as u8;
        self.pc += 2;
    }

    fn _8xy5(&mut self, vx_reg: usize, vy_reg: usize) {
        self.vx[15] = if self.vx[vx_reg] > self.vx[vy_reg] {
            1
        } else {
            0
        };
        self.vx[vx_reg] -= self.vx[vy_reg];
        self.pc += 2;
    }

    fn _8xy6(&mut self, vx_reg: usize, _vy_reg: usize) {
        self.vx[15] = if (self.vx[vx_reg] & 0x1) == 1 { 1 } else { 0 };
        self.vx[vx_reg] /= 2;
        self.pc += 2;
    }

    fn _8xy7(&mut self, vx_reg: usize, vy_reg: usize) {
        self.vx[15] = if self.vx[vy_reg] > self.vx[vx_reg] {
            1
        } else {
            0
        };
        self.vx[vx_reg] = self.vx[vy_reg] - self.vx[vx_reg];
        self.pc += 2;
    }

    fn _8xy_e(&mut self, vx_reg: usize, _vy_reg: usize) {
        let shifted_register = self.vx[vx_reg] << 1;
        self.vx[15] = if (shifted_register & 0x1) == 1 { 1 } else { 0 };
        self.vx[vx_reg] *= 2;
        self.pc += 2;
    }

    fn _9xy0(&mut self, vx_reg: usize, vy_reg: usize) {
        if self.vx[vx_reg] != self.vx[vy_reg] {
            self.pc += 2
        }
        self.pc += 2;
    }

    fn _annn(&mut self, nnn: u16) {
        self.i = nnn;
        self.pc += 2;
    }

    fn _bnnn(&mut self, nnn: u16) {
        self.pc = (nnn as usize) + (self.vx[0] as usize);
    }

    fn _cxkk(&mut self, vx_reg: usize, kk: u8) {
        self.vx[vx_reg] = kk & rand::random::<u8>();
        self.pc += 2;
    }

    fn _dxyn(&mut self, vx_reg: usize, vy_reg: usize, bytes_to_read: usize) {
        let row = self.vx[vy_reg];
        let col = self.vx[vx_reg];
        self.vx[0xF] = 0;

        for i in 0..bytes_to_read {
            let memory_pixel = self.memory[(self.i as usize) + i];

            for j in 0..8 {
                let bit = (memory_pixel >> j) & 0x1;
                let pixel_screen = self.monitor[(row as usize + i) % SCREEN_HEIGHT]
                    [(col as usize + 7 - j) % SCREEN_WIDTH];

                if bit == 1 && pixel_screen == 1 {
                    self.vx[0xF] = 1;
                }

                self.monitor[(row as usize + i) % SCREEN_HEIGHT]
                    [(col as usize + 7 - j) % SCREEN_WIDTH] ^= bit;
            }
        }

        self.pc += 2;
    }

    fn _ex9_e(&mut self, vx_reg: usize) {
        if self.keyboard[self.vx[vx_reg] as usize] == true {
            self.keyboard[self.vx[vx_reg] as usize] = false;
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn _ex_a1(&mut self, vx_reg: usize) {
        if self.keyboard[self.vx[vx_reg] as usize] == false {
            self.pc += 2;
        }
        self.pc += 2;
        self.keyboard[self.vx[vx_reg] as usize] = false;
        /* for i in &mut self.keyboard{
            *i = false;
        } */
    }

    fn _fx07(&mut self, vx_reg: usize) {
        self.vx[vx_reg] = self.dt;
        self.pc += 2;
    }

    fn _fx0_a(&mut self, vx_reg: usize) {
        if let Some(k) = self.keyboard.iter().position(|x| *x == true){
            
            self.keyboard[k] = false;
            self.vx[vx_reg] = k as u8;
            self.pc += 2;
        }
    }

    fn _fx15(&mut self, vx_reg: usize) {
        self.dt = self.vx[vx_reg];
        self.pc += 2;
    }

    fn _fx18(&mut self, vx_reg: usize) {
        self.st = self.vx[vx_reg];
        self.pc += 2;
    }

    fn _fx1_e(&mut self, vx_reg: usize) {
        self.i = self.i.wrapping_add(self.vx[vx_reg] as u16);
        self.pc += 2;
    }

    fn _fx29(&mut self, vx_reg: usize) {
        self.i = (5 * self.vx[vx_reg]) as u16;
        self.pc += 2;
    }

    fn _fx33(&mut self, vx_reg: usize) {
        self.memory[self.i as usize] = (((self.vx[vx_reg] as i32) % 1000) / 100) as u8;
        self.memory[(self.i + 1) as usize] = (self.vx[vx_reg] % 100) / 10;
        self.memory[(self.i + 2) as usize] = self.vx[vx_reg] % 10;
        self.pc += 2;
    }

    fn _fx55(&mut self, vx_reg: usize) {
        let mut tmp = self.i as usize;

        for i in 0..=vx_reg {
            self.memory[tmp] = self.vx[i];
            tmp += 1;
        }
        self.pc += 2;
    }

    fn _fx65(&mut self, vx_reg: usize) {
        let mut tmp = self.i as usize;

        for i in 0..=vx_reg {
            self.vx[i] = self.memory[tmp];
            tmp += 1;
        }
        self.pc += 2;
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
		"Chip8|opcodes per cycle:{}|timer between cpu cycles:{} hz",
		opcodes_per_cycle,
		timer_hz,
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
							Span::styled("", Style::default().fg(Color::White)),
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
                        },
                        KeyCode::Down => {
                        	opcodes_per_cycle -= if opcodes_per_cycle > 1 { 1 } else { 0 };
                        	Ok(())
                        },
                        KeyCode::Right => {
                        	timer_hz += if timer_hz < u8::MAX { 1 } else { 0 };
                        	Ok(())
                        },
                        KeyCode::Left => {
                        	timer_hz -= if timer_hz > 1 { 1 } else { 0 };
                        	Ok(())
                        },
                        KeyCode::Char('l') => {
                        	screen_height = if screen_height == SCREEN_HEIGHT {
                        		terminal.size().unwrap().height as usize
                        	} else {
                        		SCREEN_HEIGHT
                        	};
                        	screen_width = if screen_width == SCREEN_WIDTH {
                        		terminal.size().unwrap().width as usize
                        	} else {
                        		SCREEN_WIDTH
                        	};
                        	Ok(())
                        },
                        _ => Ok(()),
        			};
        		}
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
			.open("./daves_roms/TETRIS")
			.unwrap();

		let mut file_contents: Vec<u8> = Vec::new();
		file.read_to_end(&mut file_contents).unwrap();

		let mut chip_8 = Chip8::start(&file_contents[..]);

		loop {
			chip_8.execute_next_opcode();
			println!();
		}
	}
}
