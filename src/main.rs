use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::time::Duration;
use std::thread;



const LARGURA: usize = 60;
const ALTURA: usize = 30;

fn borda(i:usize, j:usize)->bool {
	if (i==0)||(i==LARGURA-1)||(j==0)||(j==ALTURA-1) {
		return true;
	}
	return false;
}

fn cria_raid() 
{
    let mapa: [[char; LARGURA]; ALTURA] = [[' ';LARGURA];ALTURA];

    for i in 0..ALTURA {
        for j in 0..LARGURA {
        	if borda(i,j) {
        		print!("#");
        	}
        	else{
	        	print!("{}",mapa[i][j]);
        	}
        }
        println!();
    }
}

fn main() {
	//pega tecla do stdin
    let stdin = stdin();
    cria_raid();
    /*
    //entra em modo raw não sei bem o que é isso
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.{}",
           	// Clear the screen.
           	termion::clear::All,
           	// Goto (1,1).
           	termion::cursor::Goto(1, 1),
         	// Hide the cursor.
	    	termion::cursor::Hide).unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {        
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();
        match c.unwrap() {
        	Key::Char('q')   => break,        
            Key::Left      => println!("<left>"),
            Key::Right     => println!("<right>"),
            Key::Up        => println!("<up>"),
            Key::Down      => println!("<down>"),
            _              => println!("Other"),
        }
        // Flush again.
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
    */
}