use core::fmt::Write;
use gba::{
    mgba::{MgbaBufferedLogger, MgbaMessageLevel},
    video::Color,
};

pub const SCREEN_W: i32 = 240;
pub const SCREEN_H: i32 = 160;

const DIGITS: [[u8; 8]; 10] = [
    [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00],
    [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00],
    [0x3C, 0x66, 0x06, 0x0C, 0x30, 0x60, 0x7E, 0x00],
    [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00],
    [0x0C, 0x1C, 0x3C, 0x6C, 0x7E, 0x0C, 0x1E, 0x00],
    [0x7E, 0x60, 0x7C, 0x06, 0x06, 0x66, 0x3C, 0x00],
    [0x1C, 0x30, 0x60, 0x7C, 0x66, 0x66, 0x3C, 0x00],
    [0x7E, 0x06, 0x0C, 0x18, 0x30, 0x30, 0x30, 0x00],
    [0x3C, 0x66, 0x66, 0x3C, 0x66, 0x66, 0x3C, 0x00],
    [0x3C, 0x66, 0x66, 0x3E, 0x06, 0x0C, 0x38, 0x00],
];

// A
const GLYPH_A: [u8; 8] = [
    0x18,
    0x24,
    0x42,
    0x42,
    0x7E,
    0x42,
    0x42,
    0x00,
];

// B
const GLYPH_B: [u8; 8] = [
    0x7C,
    0x42,
    0x42,
    0x7C,
    0x42,
    0x42,
    0x7C,
    0x00,
];

// C
const GLYPH_C: [u8; 8] = [
    0x3C,
    0x42,
    0x40,
    0x40,
    0x40,
    0x42,
    0x3C,
    0x00,
];

// D
const GLYPH_D: [u8; 8] = [
    0x78,
    0x44,
    0x42,
    0x42,
    0x42,
    0x44,
    0x78,
    0x00,
];

// E
const GLYPH_E: [u8; 8] = [
    0x7E,
    0x40,
    0x40,
    0x7C,
    0x40,
    0x40,
    0x7E,
    0x00,
];

// F
const GLYPH_F: [u8; 8] = [
    0x7E,
    0x40,
    0x40,
    0x7C,
    0x40,
    0x40,
    0x40,
    0x00,
];

// G
const GLYPH_G: [u8; 8] = [
    0x3C,
    0x42,
    0x40,
    0x4E,
    0x42,
    0x42,
    0x3C,
    0x00,
];

// H
const GLYPH_H: [u8; 8] = [
    0x42,
    0x42,
    0x42,
    0x7E,
    0x42,
    0x42,
    0x42,
    0x00,
];

// I
const GLYPH_I: [u8; 8] = [
    0x3C,
    0x18,
    0x18,
    0x18,
    0x18,
    0x18,
    0x3C,
    0x00,
];

// J
const GLYPH_J: [u8; 8] = [
    0x1E,
    0x08,
    0x08,
    0x08,
    0x48,
    0x48,
    0x30,
    0x00,
];

// K
const GLYPH_K: [u8; 8] = [
    0x42,
    0x44,
    0x48,
    0x70,
    0x48,
    0x44,
    0x42,
    0x00,
];

// L
const GLYPH_L: [u8; 8] = [
    0x40,
    0x40,
    0x40,
    0x40,
    0x40,
    0x40,
    0x7E,
    0x00,
];

// M
const GLYPH_M: [u8; 8] = [
    0x42,
    0x66,
    0x5A,
    0x42,
    0x42,
    0x42,
    0x42,
    0x00,
];

// N
const GLYPH_N: [u8; 8] = [
    0x42,
    0x62,
    0x52,
    0x4A,
    0x46,
    0x42,
    0x42,
    0x00,
];

// O
const GLYPH_O: [u8; 8] = [
    0x3C,
    0x42,
    0x42,
    0x42,
    0x42,
    0x42,
    0x3C,
    0x00,
];

// P
const GLYPH_P: [u8; 8] = [
    0x7C,
    0x42,
    0x42,
    0x7C,
    0x40,
    0x40,
    0x40,
    0x00,
];

// Q
const GLYPH_Q: [u8; 8] = [
    0x3C,
    0x42,
    0x42,
    0x42,
    0x4A,
    0x44,
    0x3A,
    0x00,
];

// R
const GLYPH_R: [u8; 8] = [
    0x7C,
    0x42,
    0x42,
    0x7C,
    0x48,
    0x44,
    0x42,
    0x00,
];

// S
const GLYPH_S: [u8; 8] = [
    0x3E,
    0x40,
    0x40,
    0x3C,
    0x02,
    0x02,
    0x7C,
    0x00,
];

// T
const GLYPH_T: [u8; 8] = [
    0x7E,
    0x18,
    0x18,
    0x18,
    0x18,
    0x18,
    0x18,
    0x00,
];

// U
const GLYPH_U: [u8; 8] = [
    0x42,
    0x42,
    0x42,
    0x42,
    0x42,
    0x42,
    0x3C,
    0x00,
];

// V
const GLYPH_V: [u8; 8] = [
    0x42,
    0x42,
    0x42,
    0x24,
    0x24,
    0x18,
    0x18,
    0x00,
];

// W
const GLYPH_W: [u8; 8] = [
    0x42,
    0x42,
    0x5A,
    0x5A,
    0x66,
    0x66,
    0x42,
    0x00,
];

// X
const GLYPH_X: [u8; 8] = [
    0x42,
    0x24,
    0x18,
    0x18,
    0x18,
    0x24,
    0x42,
    0x00,
];

// Y
const GLYPH_Y: [u8; 8] = [
    0x42,
    0x24,
    0x18,
    0x18,
    0x18,
    0x18,
    0x18,
    0x00,
];

// Z
const GLYPH_Z: [u8; 8] = [
    0x7E,
    0x04,
    0x08,
    0x18,
    0x20,
    0x40,
    0x7E,
    0x00,
];

const GLYPH_SPACE: [u8; 8] = [0x00; 8];

fn draw_char(ch: u8, x: i32, y: i32, color: Color) {
    let glyph: &[u8; 8] = if (b'0'..=b'9').contains(&ch) {
        let idx = (ch - b'0') as usize;
        &DIGITS[idx]
    } else {
        match ch {
            b'A' => &GLYPH_A,
            b'B' => &GLYPH_B,
            b'C' => &GLYPH_C,
            b'D' => &GLYPH_D,
            b'E' => &GLYPH_E,
            b'F' => &GLYPH_F,
            b'G' => &GLYPH_G,
            b'H' => &GLYPH_H,
            b'I' => &GLYPH_I,
            b'J' => &GLYPH_J,
            b'K' => &GLYPH_K,
            b'L' => &GLYPH_L,
            b'M' => &GLYPH_M,
            b'N' => &GLYPH_N,
            b'O' => &GLYPH_O,
            b'P' => &GLYPH_P,
            b'Q' => &GLYPH_Q,
            b'R' => &GLYPH_R,
            b'S' => &GLYPH_S,
            b'T' => &GLYPH_T,
            b'U' => &GLYPH_U,
            b'V' => &GLYPH_V,
            b'W' => &GLYPH_W,
            b'X' => &GLYPH_X,
            b'Y' => &GLYPH_Y,
            b'Z' => &GLYPH_Z,
            b' ' => &GLYPH_SPACE,
            _ => return,
        }
    };

    for (row, bits) in glyph.iter().enumerate() {
        for col in 0..8 {
            if bits & (0x80 >> col) != 0 {
                let px = x + col as i32;
                let py = y + row as i32;
                if px >= 0 && px < SCREEN_W && py >= 0 && py < SCREEN_H {
                    video3_draw_pixel(px, py, color);
                }
            }
        }
    }
}


pub fn draw_text(text: &str, x: i32, y: i32, color: Color) {
    let mut cx = x;
    for b in text.bytes() {
        draw_char(b, cx, y, color);
        cx += 8;
    }
}

pub fn video3_draw_pixel(x: i32, y: i32, color: Color) {
    let offset = (y * SCREEN_W + x) as usize;
    unsafe {
        let vram = 0x0600_0000 as *mut u16;
        vram.add(offset).write_volatile(color.0);
    }
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut l) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(l, "PANIC: {info}").ok();
    }
    video3_clear_to(Color::RED);
    loop {}
}

pub fn video3_clear_to(color: Color) {
    let vram = 0x0600_0000 as *mut u16;
    for i in 0..(SCREEN_W * SCREEN_H) {
        unsafe { vram.add(i as usize).write_volatile(color.0) };
    }
}

#[no_mangle]
unsafe extern "C" fn _critical_section_1_0_acquire() -> u8 {
    let ime = gba::mmio::IME.read();
    gba::mmio::IME.write(false);
    if ime {
        1
    } else {
        0
    }
}

#[no_mangle]
unsafe extern "C" fn _critical_section_1_0_release(restore_state: u8) {
    if restore_state != 0 {
        gba::mmio::IME.write(true);
    }
}
