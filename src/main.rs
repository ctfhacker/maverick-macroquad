use macroquad::*;
use boardgames_macroquad::row::Row;
use boardgames_macroquad::piece::Piece;
use boardgames_macroquad::assets::ASSETS;
use boardgames_macroquad::Resizeable;
use std::collections::HashMap;

#[repr(u32)]
enum Asset {
    ActionCard,
    MeleeTarget,
    // Monster(&'static str),
}

#[macroquad::main("Maverick")]
async fn main() {
    let mut res = HashMap::new();
    res.insert(Asset::ActionCard as u32, load_texture("static/action.png").await);
    res.insert(Asset::MeleeTarget as u32, load_texture("static/melee.png").await);

    let assets = ASSETS.get_or_init(||  res );

    let mut character_piece = Piece::new(Asset::ActionCard as u32);
    let child_piece = Piece::new(Asset::MeleeTarget as u32);
    character_piece.add_child(Piece::new(Asset::MeleeTarget as u32), 
                              vec2(0.0, 0.20), vec2(-0.5, 0.0));
    character_piece.add_child(Piece::new(Asset::MeleeTarget as u32), 
                              vec2(1.0, 0.20), vec2(-0.5, 0.0));
    character_piece.add_child(Piece::new(Asset::MeleeTarget as u32), 
                              vec2(0.0, 0.40), vec2(-0.5, 0.0));
    character_piece.add_child(Piece::new(Asset::MeleeTarget as u32), 
                              vec2(1.0, 0.40), vec2(-0.5, 0.0));

    let mut row = Row::new();
    row.spacing(20.0);
    for _ in 0..8 {
        row.add(character_piece.clone());
    }

    loop {
        clear_background(BLACK);

        row.draw(vec2(0.0, 0.0));

        character_piece.draw(vec2(0.0, row.height()), 1.0);

        next_frame().await
    }
}
