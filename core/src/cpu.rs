// Memory layout
// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
// 0x200-0xFFF - Program ROM and work RAM

use std::collections::VecDeque;

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