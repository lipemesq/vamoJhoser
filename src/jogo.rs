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
    moto: Moto,
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
            moto: Moto::new(5,75),
            tempo_de_espera: 0.0,
            largura,
            altura,
            bateu: false,
        }
    }

    /*
    *Função que verifica se alguma tecla foi precionada
    */
    pub fn tecla_precionada(&mut self, key:Key) {
        if self.bateu {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direcao::Up),
            Key::Down => Some(Direcao::Down),
            Key::Left => Some(Direcao::Left),
            Key::Right => Some(Direcao::Right),
            _ => Some(self.moto.direcao_da_moto())
        };

        if dir.unwrap() == self.moto.direcao_da_moto().oposto() {
            return;
        }

        self.atualiza_moto(dir);
    }

    /*
    *Escreve na tela as ultimas atualização
    */
    pub fn printa(&self, con: &Context, g:&mut G2d) {
        self.moto.printa(con, g);

        printa_retangulo(BORDA, 0, 0, self.largura, 1, con, g);
        printa_retangulo(BORDA, 0, self.altura - 1, self.largura, 1, con, g);
        printa_retangulo(BORDA, 0, 0, 1, self.altura, con, g);
        printa_retangulo(BORDA, self.largura - 1, 0, 1, self.altura, con, g);

        if self.bateu {
            printa_retangulo(FIM_DE_JOGO, 0, 0, self.largura, self.altura, con, g);
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
     */
    fn checa_se_bateu(&self, dir: Option<Direcao>) -> bool {
        let (proximo_x, proximo_y) = self.moto.proximo_movimento(dir);

        if self.moto.ve_se_bateu(proximo_x,proximo_y) {
            return false;
        }

        proximo_x > 0 && proximo_y > 0 && proximo_x < self.largura-1 && proximo_y < self.altura-1
    }

    /*
    *Recebe como parametro uma direção e atualiza direção da moto
    */
    fn atualiza_moto(&mut self,dir: Option<Direcao>) {
        if self.checa_se_bateu(dir) {
            self.moto.mover(dir);
        } else {
            self.bateu = true;
        }
        self.tempo_de_espera = 0.0;
    }

    //reinicia o jogo
    fn reinicia (&mut self) {
        self.moto = Moto::new(5,75);
        self.tempo_de_espera = 0.0;
        self.bateu = false;
    }
}
