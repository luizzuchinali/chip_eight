// Memory layout
// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
// 0x200-0xFFF - Program ROM and work RAM

struct Chip8 {
    memory: [u8; 0x1000],
    v_registers: [u8; 0x10],
    //0x000 to 0xFFF
    index: u16,
    program_counter: u16,

    screen_state: [u8; 64 * 32],

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 0x10],
    stack_pointer: u16,

    keypad: [u8; 0x10],
}

impl Chip8 {
    fn new(program: &[u8]) -> Self {
        let mut memory = [0u8; 0x1000];
        let sub_memory = &mut memory[0x200..];
        sub_memory.copy_from_slice(&program);

        Self {
            memory,
            v_registers: [0u8; 0x10],
            index: 0x0,
            program_counter: 0x200,
            screen_state: [1u8; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0u16; 0x10],
            stack_pointer: 0x0,
            keypad: [0u8; 0x10],
        }
    }
    fn cycle(&mut self) {
        let hi = self.memory[self.program_counter as usize] as u16;
        let lo = self.memory[(self.program_counter + 1) as usize] as u16;
        self.program_counter += 2;

        let opcode = hi << 8 | lo;
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => self.cls(),       // 00E0 - CLS
                0x00EE => self.ret(),       // 00EE - RET
                _ => self.sys(opcode), // 0nnn - SYS nnn
            },

            0x1000 => println!("1NNN"),
            0x2000 => println!("2NNN"),
            0x3000 => println!("3XNN"),
            0x4000 => println!("4XNN"),
            0x5000 => println!("5XY0"),
            0x6000 => println!("6XNN"),
            0x7000 => println!("7XNN"),
            0xA000 => println!("ANNN"),
            0xB000 => println!("BNNN"),
            0xC000 => println!("CXNN"),
            0xD000 => println!("DXYN"),

            _ => match opcode {
                _ => todo!(),
            }
        }
    }

    fn cls(&mut self) {
        self.screen_state.fill(0);
    }

    fn ret(&self) {
        todo!()
    }

    fn sys(&self, opcode: u16) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Chip8;

    #[test]
    fn cls_should_fill_screen_state_with_zeroes() {
        let mut chip8 = Chip8::new(&[0x00, 0xE0]);

        chip8.cycle();

        assert!(chip8.screen_state.iter().all(|&x| x == 0));
    }
}