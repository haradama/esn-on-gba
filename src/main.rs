#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use core::fmt::Write;
use embedded_alloc::Heap;
use fixed::types::I20F12;
use gba::prelude::*;

use reservoir_core::types::Scalar;
use reservoir_train::mackey_glass::{MackeyGlass, MackeyGlassParams};
use reservoir_train::{rmse, ESNBuilder};

const SCREEN_W: i32 = 240;
const SCREEN_H: i32 = 160;

type MyFixed = I20F12;

#[global_allocator]
static HEAP: Heap = Heap::empty();

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

#[no_mangle]
extern "C" fn main() -> ! {
    {
        use core::mem::MaybeUninit;
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

    if let Ok(mut l) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Debug) {
        writeln!(l, "=== GBA ESN START ===").ok();
        writeln!(l, "Heap initialized: {} bytes in EWRAM", 100 * 1024).ok();
    }

    let steps = (SCREEN_W + 20) as usize;
    let mut mg = MackeyGlass::new(MackeyGlassParams {
        a: 0.2,
        b: 0.1,
        n: 10,
        tau: 17,
        x0: 1.2,
        h: 1.0,
        steps,
        seed: Some(42),
        history: None,
    });

    let data_f64 = mg.generate();

    let inputs_fixed: Vec<Vec<MyFixed>> = data_f64[..data_f64.len() - 1]
        .iter()
        .map(|&v| {
            let val = MyFixed::from_f64_val(v);
            alloc::vec![val]
        })
        .collect();

    let targets_fixed: Vec<Vec<MyFixed>> = data_f64[1..]
        .iter()
        .map(|&v| {
            let val = MyFixed::from_f64_val(v);
            alloc::vec![val]
        })
        .collect();

    if let Ok(mut l) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Debug) {
        writeln!(l, "Data generated. Steps: {}", inputs_fixed.len()).ok();
    }

    let units = 20;
    let mut esn = ESNBuilder::<MyFixed>::new(1, 1)
        .units(units)
        .spectral_radius(MyFixed::from_num(0.9))
        .input_scaling(MyFixed::from_num(1.0))
        .leaking_rate(MyFixed::from_num(0.8))
        .seed(42)
        .build_lasso();

    if let Ok(mut l) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Debug) {
        writeln!(l, "ESN Built (Lasso). Units: {}", units).ok();
        writeln!(l, "Training start...").ok();
    }

    let washout = 20;
    let alpha = MyFixed::from_num(1e-4);
    let tol = MyFixed::from_num(1e-4);
    let max_iter = 1000;

    esn.fit_lasso(&inputs_fixed, &targets_fixed, alpha, max_iter, tol, washout);

    if let Ok(mut l) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Debug) {
        writeln!(l, "Training finished!").ok();
    }

    let mut preds_fixed = Vec::with_capacity(inputs_fixed.len());

    for input in &inputs_fixed {
        let out = esn.predict(input.as_slice());
        preds_fixed.push(out[0]);
    }

    let y_true_f64: Vec<f64> = targets_fixed.iter().map(|v| v[0].to_num()).collect();
    let preds_f64: Vec<f64> = preds_fixed.iter().map(|v| v.to_num()).collect();

    let error_score = rmse(&y_true_f64, &preds_f64);

    if let Ok(mut l) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Info) {
        writeln!(l, "-----------------------------").ok();
        writeln!(l, "Evaluation Result (Fixed Point)").ok();
        writeln!(l, "RMSE : {}", error_score).ok();
        writeln!(l, "-----------------------------").ok();
    }

    let scale_y = MyFixed::from_num(50);
    let offset_y = MyFixed::from_num(100);

    let y_true_fixed: Vec<MyFixed> = targets_fixed.iter().map(|v| v[0]).collect();

    for i in 0..preds_fixed.len() {
        if i >= SCREEN_W as usize {
            break;
        }

        let x = i as i32;

        let true_val = y_true_fixed[i];
        let y_true_pos = (offset_y - true_val * scale_y).to_num::<i32>();

        if y_true_pos >= 0 && y_true_pos < SCREEN_H {
            video3_draw_pixel(x, y_true_pos, Color::GREEN);
        }

        let pred_val = preds_fixed[i];
        let y_pred_pos = (offset_y - pred_val * scale_y).to_num::<i32>();

        if y_pred_pos >= 0 && y_pred_pos < SCREEN_H {
            video3_draw_pixel(x, y_pred_pos, Color::RED);
        }
    }

    loop {
        VBlankIntrWait();
    }
}

fn video3_draw_pixel(x: i32, y: i32, color: Color) {
    let offset = (y * SCREEN_W + x) as usize;
    unsafe {
        let vram = 0x0600_0000 as *mut u16;
        vram.add(offset).write_volatile(color.0);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut l) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(l, "PANIC: {info}").ok();
    }
    video3_clear_to(Color::RED);
    loop {}
}

fn video3_clear_to(color: Color) {
    let vram = 0x0600_0000 as *mut u16;
    for i in 0..(SCREEN_W * SCREEN_H) {
        unsafe { vram.add(i.try_into().unwrap()).write_volatile(color.0) };
    }
}
