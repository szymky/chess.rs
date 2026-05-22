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
#[derive(Debug)]
enum PieceType {
    Queen,
    King,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

#[derive(Debug)]
enum PieceColor {
    Black,
    White,
}

impl PieceColor {
    fn to_id(&self) -> usize {
        match self {
            PieceColor::Black => 0,
            PieceColor::White => 1,
        }
    }
}

#[derive(Debug)]
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
}

#[derive(Debug)]
struct Piece<'a> {
    piece_type: PieceType,
    texture: PieceTexture<'a>,
    selected: bool,
}

impl<'a> Piece<'a> {
    fn new(
        piece_type: PieceType,
        color: PieceColor,
        position: (u8, u8),
        piece_spritesheet: &'a Texture2D,
    ) -> Self {
        let piece_width = piece_spritesheet.width() / 6.0;
        let piece_height = piece_spritesheet.height() / 2.0;

        let tex_rec_x: f32 = match piece_type {
            PieceType::Queen => 0,
            PieceType::King => 1,
            PieceType::Rook => 2,
            PieceType::Knight => 3,
            PieceType::Bishop => 4,
            PieceType::Pawn => 5,
        } as f32
            * piece_width;

        let tex_rec_y: f32 = color.to_id() as f32 * piece_height;

        let texture_pos_x = position.0 as f32 * (screen_width() / BOARD_WIDTH as f32);
        let texture_pos_y = position.1 as f32 * (screen_height() / BOARD_HEIGHT as f32);

        let piece_texture: PieceTexture<'a> = PieceTexture::new(
            Rect::new(tex_rec_x, tex_rec_y, piece_width, piece_height),
            Vec2::new(texture_pos_x, texture_pos_y),
            piece_spritesheet,
        );

        Self {
            piece_type,
            texture: piece_texture,
            selected: false,
        }
    }

    fn mouse_over(&self) -> bool {
        let (mx, my) = mouse_position();

        mx >= self.texture.position.x
            && mx <= self.texture.position.x + self.texture.texture_rect.w
            && my >= self.texture.position.y
            && my <= self.texture.position.y + self.texture.texture_rect.h
    }
}

#[macroquad::main(conf)]
async fn main() {
    let size = screen_width() / BOARD_WIDTH as f32;

    let pieces: Texture2D = load_texture("ChessPiecesArray.png").await.unwrap();

    pieces.set_filter(FilterMode::Nearest);

    let mut piece_vec: Vec<Piece> = Vec::new();

    for i in 0..BOARD_WIDTH {
        let piece = Piece::new(
            PieceType::Pawn,
            if i % 2 == 0 {
                PieceColor::White
            } else {
                PieceColor::Black
            },
            (i as u8, 1),
            &pieces,
        );

        piece_vec.push(piece);
    }

    loop {
        clear_background(BLACK);

        let (mx, my) = mouse_position();

        for i in 0..BOARD_HEIGHT * BOARD_WIDTH {
            let x = (i % BOARD_WIDTH) as f32;
            let y = (i / BOARD_HEIGHT) as f32;

            let px = x * size;
            let py = y * size;

            let is_light = (x + y) % 2.0 == 0.0;

            let color = if is_light { LIGHT_SQUARE } else { DARK_SQUARE };

            draw_rectangle(px, py, size, size, color);
        }

        for piece in piece_vec.iter_mut() {
            if is_mouse_button_down(MouseButton::Left) && piece.mouse_over() {
                piece.texture.position.x = mx - (piece.texture.texture_rect.w / 2.0);
                piece.texture.position.y = my - (piece.texture.texture_rect.h / 2.0);
            }

            piece.texture.draw();
        }

        next_frame().await
    }
}
