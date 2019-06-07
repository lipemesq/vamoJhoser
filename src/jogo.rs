use piston_window::*;
use piston_window::types::Color;

//use rand::{thread_rng, Rng};

use crate::tron::{Direcao, Moto};
use crate::printa::printa_retangulo;

const BORDA: Color = [0.00, 0.00, 0.00, 1.0];
const FIM_DE_JOGO: Color = [0.90, 0.00, 0.00, 0.5];

const INTERVALO_DE_MOVIMENTO: f64 = 0.005;
const REINICIA: f64 = 1.0;

/*
pub struct pacote {
    dir: Key,
    //algo a mais?
}
*/

/*
*struct para jogo contem as informaçãoes
*Player1, Player2, dimensões da tela, e se algum
*dos jogadores bateu 
*/
pub struct Jogo {
    moto_player1: Moto,
    moto_player2: Moto,
    largura:i32,
    altura:i32,
    bateu: bool,
    tempo_de_espera: f64,
}
/*
*implementação de jogo
*/
impl Jogo {

    /*
    *Construtor de jogo
    */
    pub fn new (largura: i32, altura: i32) ->Jogo { 
            Jogo {
            moto_player1: Moto::new(5,75),
            moto_player2: Moto::new(145,75),
            tempo_de_espera: 0.0,
            largura,
            altura,
            bateu: false,
        }
    }

    /*
    *Função que verifica se alguma tecla foi precionada
    */
    pub fn tecla_precionada(&mut self, key:Key, player:i32) {
        if player == 1 {
            if self.bateu {
                return;
            }
            let dir = match key {
                Key::Up => Some(Direcao::Up),
                Key::Down => Some(Direcao::Down),
                Key::Left => Some(Direcao::Left),
                Key::Right => Some(Direcao::Right),
                _ => Some(self.moto_player1.direcao_da_moto())
            };
            if dir.unwrap() == self.moto_player1.direcao_da_moto().oposto() {
                return;
            }
            self.atualiza_moto(dir,player);

        } else {
            if self.bateu {
                return;
            }
            let dir = match key {
                Key::Up => Some(Direcao::Up),
                Key::Down => Some(Direcao::Down),
                Key::Left => Some(Direcao::Left),
                Key::Right => Some(Direcao::Right),
                _ => Some(self.moto_player2.direcao_da_moto())
            };
            if dir.unwrap() == self.moto_player2.direcao_da_moto().oposto() {
                return;
            }
            self.atualiza_moto(dir,player);
        }
    }

    /*
    *Escreve na tela as ultimas atualização
    */
    pub fn printa(&self, con: &Context, g:&mut G2d, player:i32) {
        if player == 1 {
            self.moto_player1.printa(con, g);

            printa_retangulo(BORDA, 0, 0, self.largura, 1, con, g);
            printa_retangulo(BORDA, 0, self.altura - 1, self.largura, 1, con, g);
            printa_retangulo(BORDA, 0, 0, 1, self.altura, con, g);
            printa_retangulo(BORDA, self.largura - 1, 0, 1, self.altura, con, g);

            if self.bateu {
                printa_retangulo(FIM_DE_JOGO, 0, 0, self.largura, self.altura, con, g);
            }
        } else {
            self.moto_player2.printa(con, g);
            printa_retangulo(BORDA, 0, 0, self.largura, 1, con, g);
            printa_retangulo(BORDA, 0, self.altura - 1, self.largura, 1, con, g);
            printa_retangulo(BORDA, 0, 0, 1, self.altura, con, g);
            printa_retangulo(BORDA, self.largura - 1, 0, 1, self.altura, con, g);
            if self.bateu {
                printa_retangulo(FIM_DE_JOGO, 0, 0, self.largura, self.altura, con, g);
            }
        }

    }
    
    //verifica se o jogo acabou e reinicia em caso positivo
    pub fn atualiza(&mut self, delta_time: f64) {
        self.tempo_de_espera += delta_time;

        if self.bateu {
            if self.tempo_de_espera > REINICIA {
                self.reinicia();
            }
            return;
        }

        if self.tempo_de_espera > INTERVALO_DE_MOVIMENTO {
            self.atualiza_moto(None);
        }
    }

    /*verica se houve uma colisao entre as motos ou 
     *entre a borda da janela implementado para um player
     *
     *Fodeu essa porra aqui é uma bosta
     */
    fn checa_se_bateu(&self, dir_player1: Option<Direcao>, dir_player2: Option<Direcao>, player:i32 ) -> bool {
        if player == 1 {
            let (proximo_x, proximo_y) = self.moto_player1.proximo_movimento(dir_player1);
            if self.moto_player1.ve_se_bateu(proximo_x,proximo_y) {
                return false;
            }
            proximo_x > 0 && proximo_y > 0 && proximo_x < self.largura-1 && proximo_y < self.altura-1
        } else {
            let (proximo_x, proximo_y) = self.moto_player2.proximo_movimento(dir_player2);
            if self.moto_player2.ve_se_bateu(proximo_x,proximo_y) {
                return false;
            }
            proximo_x > 0 && proximo_y > 0 && proximo_x < self.largura-1 && proximo_y < self.altura-1
        }
    }

    /*
    *Recebe como parametro uma direção e atualiza direção da moto
    */
    fn atualiza_moto(&mut self,dir: Option<Direcao>, player:i32) {
        if player == 1 {
            if self.checa_se_bateu(dir) {
                self.moto_player1.mover(dir);
            } else {
                self.bateu = true;
            }
            self.tempo_de_espera = 0.0;
        } else {
            if self.checa_se_bateu(dir) {
                self.moto_player2.mover(dir);
            } else {
                self.bateu = true;
            }
            self.tempo_de_espera = 0.0;
        }
    }

    //reinicia o jogo
    fn reinicia (&mut self) {
        self.moto = Moto::new(5,75);
        self.tempo_de_espera = 0.0;
        self.bateu = false;
    }
}
