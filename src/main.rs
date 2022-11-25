use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;
use std::{f32::consts::PI, thread, time::Duration};

use macroquad::prelude::*;

const WINDOW_SIZE: Vec2 = Vec2::from_array([1000., 1000.]);
const WINDOW_MIDDLE: Vec2 = Vec2::from_array([WINDOW_SIZE.x / 2., WINDOW_SIZE.y / 2.]);
const SCALING_FACTOR: f32 = WINDOW_SIZE.x / 2.1;
const TIME_DELAY: Duration = Duration::from_millis(25);

const BACKGROUND_COLOR: Color = WHITE;
const FOREGROUND_COLOR: Color = BLACK;
const TEXT_COLOR: Color = BLACK;

fn window_config() -> Conf {
    Conf {
        window_title: "Times Table Shapes".to_owned(),
        window_width: WINDOW_SIZE.x.round() as i32,
        window_height: WINDOW_SIZE.y.round() as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn create_coors(num_points: usize) -> Vec<Vec2> {
    (0..num_points)
        .map(|n| Vec2::from_angle(PI + (2. * PI * (n as f32 / num_points as f32))))
        .map(|v| v * SCALING_FACTOR + WINDOW_MIDDLE)
        .collect()
}

#[macroquad::main(window_config)]
async fn main() {
    let mut factor_increment = dec!(0.01);
    let mut factor = dec!(2.);

    let mut num_points: usize = 200;
    let points_increment: usize = 5;
    let mut increasing = true;

    let mut point_coors = create_coors(num_points);

    let mut paused = false;

    loop {
        if is_key_pressed(KeyCode::Q) {
            factor_increment /= dec!(10.);
        }

        if is_key_pressed(KeyCode::E) {
            factor_increment *= dec!(10.);
        }

        if is_key_down(KeyCode::W) {
            num_points += points_increment;
            point_coors = create_coors(num_points);
        }

        if is_key_down(KeyCode::S) {
            if num_points > points_increment {
                num_points -= points_increment;
                point_coors = create_coors(num_points);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if {
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                increasing = true;
                true
            } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                increasing = false;
                true
            } else {
                false
            }
        } || !paused
        {
            point_coors = create_coors(num_points);

            if increasing {
                factor = factor + factor_increment
            } else {
                factor = factor - factor_increment;
            }
        }

        // Drawing
        clear_background(BACKGROUND_COLOR);

        draw_text(
            format!("Factor: {} (A and D)", factor).as_str(),
            10.,
            20.,
            20.,
            TEXT_COLOR,
        );
        draw_text(
            format!("Increment: {} (Q and E)", factor_increment).as_str(),
            10.,
            40.,
            20.,
            TEXT_COLOR,
        );
        draw_text(
            format!("Number of Points: {} (S and W)", num_points).as_str(),
            10.,
            60.,
            20.,
            TEXT_COLOR,
        );
        draw_text(
            format!("Pause with SPACE.").as_str(),
            10.,
            80.,
            20.,
            TEXT_COLOR,
        );

        draw_circle_lines(
            WINDOW_MIDDLE.x,
            WINDOW_MIDDLE.y,
            SCALING_FACTOR,
            1.,
            FOREGROUND_COLOR,
        );
        for (n, v) in point_coors.iter().enumerate() {
            draw_circle(v.x, v.y, 3., FOREGROUND_COLOR);
            draw_text(format!("{}", n).as_str(), v.x, v.y, 20., FOREGROUND_COLOR);
        }
        for n in 0..num_points {
            let p1 = point_coors[n];
            let p2 =
                point_coors[(n as f64 * factor.to_f64().unwrap()).round() as usize % num_points];
            draw_line(p1.x, p1.y, p2.x, p2.y, 2., FOREGROUND_COLOR);
        }

        thread::sleep(TIME_DELAY);
        next_frame().await
    }
}
