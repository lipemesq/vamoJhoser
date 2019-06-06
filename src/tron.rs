use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::printa::printa_bloco;

const MOTO1: Color = [0.00, 0.80, 0.00, 1.0];
//const MOTO2: Color = [0.00, 0.80, 0.00, 1.0];

const MAXTAM: usize = 500;

#[derive(Copy, Clone, PartialEq)]
pub enum Direcao {
    Up,
    Down,
    Left,
    Right,
}

impl Direcao {
    /*
     *Definicao para oposto de uma direção
     */
    pub fn oposto(&self) -> Direcao {
        match *self {
            Direcao::Up => Direcao::Down,
            Direcao::Down => Direcao::Up,
            Direcao::Left => Direcao::Right,
            Direcao::Right => Direcao::Left,        }
    }
}

/*
 *estrutura para representar os jogadores 
 *para biblioteca importada
 */
struct Bloco {
    x:i32,
    y:i32,
}

/*
 *struct Para Moto
 */
pub struct Moto {
    direcao: Direcao,
    rastro: LinkedList<Bloco>
}

impl Moto {
    
    /*
     *Construtor de moto
     */
    pub fn new(x: i32, y: i32) -> Moto {
        let mut rastro: LinkedList<Bloco> = LinkedList::new();
        rastro.push_back(Bloco {x, y});
        rastro.push_back(Bloco{
            x:x + 1,
            y,
        });
        
        Moto {
            direcao: Direcao::Right,
            rastro,
        }
    }

    /*
     *printa moto. é provavel que tenha que ser modificado
     */
    pub fn printa(&self, con: &Context, g: &mut G2d) {
        for bloco in &self.rastro {
            printa_bloco(MOTO1, bloco.x, bloco.y, con, g);
        }
    }

    /*
     *atualiza posição da moto com base no ultimo bloco adicionado
     *a lista 
     */
    pub fn posicao_moto(&self) -> (i32,i32) {
        let moto = self.rastro.front().unwrap();
        (moto.x, moto.y)
    }

    /*
     *recebe uma direção e atualiza direção ou 
     *mantem ultima caso nenhuma tecla seja apertada
     */
    pub fn mover(&mut self, dir: Option<Direcao>) {
        match dir {
            Some(d) => self.direcao = d,
            None => (),
        }

        let (x_anterior, y_anterior): (i32, i32) = self.posicao_moto();

        let novo_bloco = match self.direcao {
            Direcao::Up => Bloco {
                x: x_anterior,
                y: y_anterior - 1,
            },
            Direcao::Down => Bloco {
                x: x_anterior,
                y: y_anterior + 1,
            },
            Direcao::Left => Bloco {
                x: x_anterior - 1,
                y: y_anterior,
            },
            Direcao::Right => Bloco {
                x: x_anterior + 1,
                y: y_anterior,
            },
        };
        self.rastro.push_front(novo_bloco);
        
        if self.rastro.len() > MAXTAM {
            self.rastro.pop_back();
        }
    }

    /*
     *Retorna direção atual da moto
     */
    pub fn direcao_da_moto(&self) -> Direcao {
        self.direcao
    }

    /*
     * Verifica qual foi a tecla apertada para fazer o proximo movimento
     */
    pub fn proximo_movimento(&self, dir: Option<Direcao>) -> (i32, i32) {
        let (moto_x, moto_y): (i32, i32) = self.posicao_moto();

        let mut direcao_movimento = self.direcao;
        match dir {
            Some(d) => direcao_movimento = d,
            None => {}
        }

        match direcao_movimento {
            Direcao::Up => (moto_x, moto_y - 1),
            Direcao::Down => (moto_x, moto_y + 1),
            Direcao::Left => (moto_x - 1, moto_y),
            Direcao::Right => (moto_x + 1, moto_y),
        }
    }

    /*
     *verifica se a moto se chocou com o proprio rastro 
     *creio que isso da para remover ou melhorar
     */
    pub fn ve_se_bateu(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for bloco in &self.rastro {
            if x == bloco.x && y == bloco.y {
                return true;
            }

            ch += 1;
            if ch == self.rastro.len() - 1 {
                break;
            }
        }
        return false;
    }
}
