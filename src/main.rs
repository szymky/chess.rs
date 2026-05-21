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

enum Piece {
    Queen,
    King,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[macroquad::main(conf)]
async fn main() {
    let size = screen_width() / BOARD_WIDTH as f32;

    let pieces: Texture2D = load_texture("ChessPiecesArray.png").await.unwrap();

    pieces.set_filter(FilterMode::Nearest);

    let piece_height = pieces.height() / 2.0;
    let piece_width = pieces.width() / 6.0;

    let mut piece_vec: Vec<Rect> = Vec::new();

    for i in 0..6 {
        piece_vec.push(Rect::new(
            i as f32 * piece_width,
            0.,
            piece_width,
            piece_height,
        ));
    }

    loop {
        clear_background(BLACK);

        // let (mx, my) = mouse_position();
        //
        //

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

        for (i, &piece) in piece_vec.iter().enumerate() {
            draw_texture_ex(
                &pieces,
                piece_width * i as f32,
                piece_height * i as f32,
                WHITE,
                DrawTextureParams {
                    source: Some(piece),
                    ..Default::default()
                },
            );
        }

        next_frame().await
    }
}
