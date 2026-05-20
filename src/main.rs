use macroquad::prelude::*;

mod consts;

use consts::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Chess"),
        window_resizable: false,
        window_width: 600,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let size = screen_width() / BOARD_WIDTH as f32;

    loop {
        clear_background(BLACK);

        // let (mx, my) = mouse_position();

        for i in 0..BOARD_HEIGHT * BOARD_WIDTH {
            let x = (i % BOARD_WIDTH) as f32;
            let y = (i / BOARD_HEIGHT) as f32;

            let px = x * size;
            let py = y * size;

            //let is_hovering = mx < px + size && mx > px && my < py + size && my > py;

            let is_light = (x + y) % 2.0 == 0.0;

            let color = if is_light { LIGHT_SQUARE } else { DARK_SQUARE };

            draw_rectangle(px, py, size, size, color);
        }

        next_frame().await
    }
}
