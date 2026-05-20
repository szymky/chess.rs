use macroquad::prelude::*;

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
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
