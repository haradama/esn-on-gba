#![no_std]
#![no_main]

use gba::prelude::*;
use nalgebra::{SMatrix, SVector};

use reservoir_infer::esn::StaticESN;
use reservoir_infer::readout::static_readout::StaticReadout as DenseStaticReadout;
use reservoir_infer::reservoir::static_reservoir::StaticReservoir as DenseStaticReservoir;

use esn_on_gba::utils::{self, draw_text, video3_clear_to, SCREEN_H, SCREEN_W};

include!(concat!(env!("OUT_DIR"), "/esn_generated.rs"));

#[no_mangle]
extern "C" fn main() -> ! {
    DISPCNT.write(
        DisplayControl::new()
            .with_video_mode(VideoMode::_3)
            .with_show_bg2(true),
    );
    video3_clear_to(Color::BLACK);

    draw_text("MACKEY GLASS", 4, 4, Color::WHITE);
    draw_text("TRUTH", 4, 16, Color::GREEN);
    draw_text("PRED", 4, 26, Color::RED);

    let w_in: SMatrix<S, N, IN> = SMatrix::from_row_slice(&W_IN_DATA);
    let w: SMatrix<S, N, N> = SMatrix::from_row_slice(&W_DATA);
    let reservoir: DenseStaticReservoir<S, IN, N, EXT> =
        DenseStaticReservoir::create(w_in, w, LEAKING_RATE);

    let w_out: SMatrix<S, OUT, EXT> = SMatrix::from_row_slice(&W_OUT_DATA);
    let readout: DenseStaticReadout<S, EXT, OUT> = DenseStaticReadout::create(w_out);

    let mut esn: StaticESN<S, _, _> = StaticESN::new(reservoir, readout);

    let steps = core::cmp::min(EVAL_STEPS, SCREEN_W as usize);

    let mut vmin: S = EVAL_TRUTH[0];
    let mut vmax: S = EVAL_TRUTH[0];
    for i in 1..steps {
        let v = EVAL_TRUTH[i];
        if v < vmin {
            vmin = v;
        }
        if v > vmax {
            vmax = v;
        }
    }

    let mut range: S = vmax - vmin;
    if range.abs() < 1e-6 {
        range = 1e-6;
    }

    let mid: S = (vmax + vmin) * 0.5;
    let center_y: S = (SCREEN_H as S) * 0.5;
    let scale: S = (SCREEN_H as S) * 0.45 / range;

    for i in 0..steps {
        let u = EVAL_U[i];
        let truth = EVAL_TRUTH[i];

        let x_in: SVector<S, IN> = SVector::<S, IN>::from_row_slice(&[u]);
        let y: SVector<S, OUT> = esn.predict::<IN, OUT, EXT>(&x_in);
        let pred = y[0];

        let px = i as i32;

        let y_truth = (center_y - (truth - mid) * scale) as i32;
        let y_pred = (center_y - (pred - mid) * scale) as i32;

        if (0..SCREEN_H).contains(&y_truth) {
            utils::video3_draw_pixel(px, y_truth, Color::GREEN);
        }
        if (0..SCREEN_H).contains(&y_pred) {
            utils::video3_draw_pixel(px, y_pred, Color::RED);
        }
    }

    loop {
        VBlankIntrWait();
    }
}
