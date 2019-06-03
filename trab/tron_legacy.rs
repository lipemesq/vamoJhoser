const TAMANHO_MAPA: usize = 60;

fn cria_raid() 
{
    let mut mapa: [[char; TAMANHO_MAPA]; TAMANHO_MAPA] = [[' ';TAMANHO_MAPA];TAMANHO_MAPA];

    for i in 0..TAMANHO_MAPA {
        mapa[0][i] = '#';
        mapa[i][TAMANHO_MAPA-1] = '#';
        mapa[i][0] = '#';
        mapa[TAMANHO_MAPA-1][i] = '#';
    }


    for i in 0..TAMANHO_MAPA {
        for j in 0..TAMANHO_MAPA {
            print!("{}[",'\x1B');
            print!("{}m",40+1 );
            print!("{} ",mapa[i][j]);
        }
        println!();
    }
}

fn main() {

    //while true {
        //std::system::command("reset");
        cria_raid();
        print!("{}[",'\x1B');
        print!("0m")
    //}
    
}