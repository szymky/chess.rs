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

#[allow(dead_code)]
enum Piece {
    Queen,
    King,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

struct PieceTexture<'a> {
    texture_rect: Rect,
    position: Vec2,

    texture: &'a Texture2D,
}

impl<'a> PieceTexture<'a> {
    fn new(texture_rect: Rect, position: Vec2, texture: &'a Texture2D) -> Self {
        Self {
            texture_rect,
            position,
            texture,
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.texture_rect),
                ..Default::default()
            },
        );
    }

    fn mouse_over(&self) -> bool {
        let (mx, my) = mouse_position();

        mx >= self.position.x
            && mx <= self.position.x + self.texture_rect.w
            && my >= self.position.y
            && my <= self.position.y + self.texture_rect.h
    }
}

#[macroquad::main(conf)]
async fn main() {
    let size = screen_width() / BOARD_WIDTH as f32;

    let pieces: Texture2D = load_texture("ChessPiecesArray.png").await.unwrap();

    pieces.set_filter(FilterMode::Nearest);

    let piece_height = pieces.height() / 2.0;
    let piece_width = pieces.width() / 6.0;

    let mut piece_vec: Vec<PieceTexture> = Vec::new();

    for i in 0..6 {
        piece_vec.push(PieceTexture::new(
            Rect::new(i as f32 * piece_width, 0., piece_width, piece_height),
            Vec2::new(i as f32 * piece_width, 0.0),
            &pieces,
        ));
    }

    loop {
        clear_background(BLACK);

        let (mx, my) = mouse_position();

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

        for piece in piece_vec.iter_mut() {
            if is_mouse_button_down(MouseButton::Left) && piece.mouse_over() {
                piece.position.x = mx - (piece.texture_rect.w / 2.0);
                piece.position.y = my - (piece.texture_rect.h / 2.0);
            }

            piece.draw();
        }

        next_frame().await
    }
}
