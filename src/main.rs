#![allow(unused)]

extern crate hostname;
extern crate dns_lookup;

use std::net::UdpSocket;
use std::net::{IpAddr, Ipv6Addr};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::io;
use std::error::Error;

use std::thread;

use std::net;
use dns_lookup::lookup_host;

use std::time::{Duration, Instant};

use piston_window::*;
use piston_window::types::Color;

use jogo::Jogo;
use printa::para_posicao_u32;

const PRETO: Color = [0.5, 0.5, 0.5, 1.0];

mod printa;
mod tron;
mod jogo;


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
			//player ainda não exite mas deve ser numero
			//identificador da thread
			jogo.tecla_precionada(key,player);
        }
        janela.draw_2d(&event, |c, g| {
            clear(PRETO, g);
            jogo.printa(&c, g,player);
        });

        event.update(|arg| {
            jogo.atualiza(arg.dt);
        });
    }
}

// Esse aqui é o servidor
fn recv()  -> Result<(), io::Error> {

	// Servidor iniciando
	println!("Servidor iniciando");

	// Pega o próprio nome 
    let hostname = hostname::get_hostname().unwrap();
	println!("Meu nome é {}", hostname);

	// Usando o seu nome, pega seu IP
	let ips = dns_lookup::lookup_host(&hostname).unwrap();
 	let mut ipOutroHost : Ipv4Addr = (Ipv4Addr::new(127, 0, 0, 1));

 	// Do IP, tira o IPv4 
	for host in ips {
       	if let IpAddr::V4(ipv4) = host {
	    	ipOutroHost = ipv4;
		}
    }

    // junta o IP com a porta que vai usar
    let meuIP =  ipOutroHost; 
    let connection = SocketAddrV4::new(meuIP, 9991);
    println!("Meu endereço é {}", connection);


    // Bind no socket
    let socket = UdpSocket::bind(connection)?;
    println!("Dei Bind no socket, e agora vou começar a escutar");

    inicia_jogo();


    let mut nThread : i8 = 0;
    let mut porta : u16 = 9993;
    // Loop
    loop {

	    // Cria o buffer onde recebe as mensagens
	    let mut buf = [0; 10]; // 10 casas com 0

	    println!("VOU RECEBER");

	    // Escuta no socket
	    let (qtdB_recebidos, remetente) = socket.recv_from(&mut buf)?;

	    // Print only the valid data (slice)
	    println!("Recebi: {0:?}", &buf[0 .. qtdB_recebidos]);
	    println!("Enviado de {}", remetente);


	    // Trata da mensagem pra ver se tá tudo ok
	    println!("Criando uma thread para cuidar disso");

	    // Cria uma thread
	    let tred = thread::spawn(move || {
	    	let meuNum = nThread;
	   		println!(" -----  Oi, eu sou a thread {}", nThread);

	    	// ---------------------------Cria socket da thread
		    let connectionThread = SocketAddrV4::new(meuIP, porta);

		    // Bind the socket
		    let socketThread = UdpSocket::bind(connectionThread).unwrap();

		    // Send data via the socket
		    //let mut bufa = 9993;
		    let mut bufa = &[9, 9, 9, 3];
		    socketThread.send_to(bufa, remetente).unwrap();
		    println!("{:?}", bufa);

		    loop {
    	    	thread::sleep(Duration::from_millis(500));
    	    	println!(" ----- Sou a thread {}, e o meu nThread esta em {}", meuNum, nThread);
		    }

		});

		//let res = tred.join();
		porta += 1;
		nThread += 1;
	}


    Ok(())
}

fn main() {
    match recv() {
        Ok(()) => println!("All recv-ing went well"),
        Err(err) => println!("Error: {:?}", err),
    }

}