#![no_std]
#![no_main]

extern crate alloc;

use core::mem::MaybeUninit;
use embedded_alloc::Heap;
use esn_on_gba::utils::{self, SCREEN_H, SCREEN_W};
use gba::prelude::*;

use reservoir_datasets::fixed_datasets::{
    FixedI16, FixedI32, MACKEY_GLASS_I16, MACKEY_GLASS_I32, MACKEY_GLASS_LEN,
};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[no_mangle]
extern "C" fn main() -> ! {
    {
        const HEAP_SIZE: usize = 100 * 1024; // 100KB

        #[link_section = ".ewram"]
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

        unsafe {
            let ptr = core::ptr::addr_of_mut!(HEAP_MEM);
            HEAP.init(ptr as usize, HEAP_SIZE)
        }
    }

    DISPCNT.write(
        DisplayControl::new()
            .with_video_mode(VideoMode::_3)
            .with_show_bg2(true),
    );
    video3_clear_to(Color::BLACK);

    let max_steps = SCREEN_W as usize;
    let steps = core::cmp::min(MACKEY_GLASS_LEN, max_steps);

    let data_i32: &[FixedI32] = &MACKEY_GLASS_I32[..steps];
    let data_i16: &[FixedI16] = &MACKEY_GLASS_I16[..steps];

    let half_h = SCREEN_H / 2;
    let center_top: i32 = half_h;
    let center_bottom: i32 = SCREEN_H;

    let scale: i32 = 40;

    let center_top_i16 = FixedI16::from_num(center_top);
    let center_bottom_i32 = FixedI32::from_num(center_bottom);

    let scale_i16 = FixedI16::from_num(scale);
    let scale_i32 = FixedI32::from_num(scale);

    utils::draw_text("MACKEY GLASS I16F16", 4, 4, Color::RED);
    utils::draw_text("MACKEY GLASS I32F32", 4, half_h + 4, Color::GREEN);

    for i in 0..steps {
        let x = i as i32;
        let v16 = data_i16[i];
        let y16 = (center_top_i16 - v16 * scale_i16).to_num::<i32>();

        if y16 >= 0 && y16 < half_h {
            utils::video3_draw_pixel(x, y16, Color::RED);
        }

        let v32 = data_i32[i];
        let y32 = (center_bottom_i32 - v32 * scale_i32).to_num::<i32>();

        if y32 >= half_h && y32 < SCREEN_H {
            utils::video3_draw_pixel(x, y32, Color::GREEN);
        }
    }

    loop {
        VBlankIntrWait();
    }
}
