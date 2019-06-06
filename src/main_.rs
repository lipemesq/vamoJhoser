extern crate piston_window;
extern crate rand;

mod printa;
mod tron;
mod jogo;

use piston_window::*;
use piston_window::types::Color;

use jogo::Jogo;
use printa::para_posicao_u32;

const PRETO: Color = [0.5, 0.5, 0.5, 1.0];

fn inicia_jogo() {
    let (largura, altura) = (150, 150);

    let mut janela: PistonWindow = WindowSettings::new(
        "Tron",[
        para_posicao_u32(largura),
        para_posicao_u32(altura)
        ])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut jogo = Jogo::new(largura, altura);
    while let Some(event) = janela.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            jogo.tecla_precionada(key);
        }
        janela.draw_2d(&event, |c, g| {
            clear(PRETO, g);
            jogo.printa(&c, g);
        });

        event.update(|arg| {
            jogo.atualiza(arg.dt);
        });
    }
}

fn main() {
    inicia_jogo();
}
