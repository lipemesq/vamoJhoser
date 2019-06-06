
/*
 * Aqui só tem função de printar na tela acredite!
 */
use piston_window::{rectangle as retangulo, Context, G2d};
use piston_window::types::Color ;

const TAMANHO_BLOCO: f64 = 5.0;

pub fn para_posicao(posicao_jogo: i32) -> f64 {
    (posicao_jogo as f64) * TAMANHO_BLOCO
}

pub fn para_posicao_u32(game_coord: i32) -> u32 {
    para_posicao(game_coord) as u32
}

pub fn printa_bloco(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let posicao_x = para_posicao(x);
    let posicao_y = para_posicao(y);

    retangulo(
        color,
        [posicao_x, posicao_y, TAMANHO_BLOCO, TAMANHO_BLOCO],
        con.transform,
        g,
    );
}

pub fn printa_retangulo(
    cor: Color,
    x: i32,
    y: i32,
    largura: i32,
    altura: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = para_posicao(x);
    let y = para_posicao(y);

    retangulo(
        cor,
        [
            x,
            y,
            TAMANHO_BLOCO * (largura as f64),
            TAMANHO_BLOCO * (altura as f64),
        ],
        con.transform,
        g,
    );
}