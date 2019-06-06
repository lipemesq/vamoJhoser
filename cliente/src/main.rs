#![allow(unused)]
use std::net::UdpSocket;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::net::{IpAddr, Ipv6Addr};

use std::time::{Duration, Instant};
use std::io;
use std::io::ErrorKind;
use std::thread;

//fn main() -> std::io::Result<()> {
    // /{
        /*let mut socket = UdpSocket::bind("127.0.0.1:34254")?;
        //socket.connect("10.254.223.80:34254").expect("CONEECT");

        let mut i = 0;

        let ip = Ipv4Addr::new(10, 254, 223, 80);
   		let connection = SocketAddrV4::new(ip, 34254);

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 10];
        loop {
        	buf[0] = i;
        	socket.send_to(&mut buf, connection)?;
        	println!("Enviei {}", buf[0]);
        	thread::sleep(Duration::from_millis(5));
        	i += 1;
    	}

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        /*let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;*/
    } // the socket is closed here*/
    /*let ip = Ipv4Addr::new(10, 254, 223, 79);
    let connection = SocketAddrV4::new(ip, 9992);

    // Bind the socket
    let mut socket = UdpSocket::bind(connection)?;

    // Define the remote connection (to send the data to)
    let ip2 = Ipv4Addr::new(10, 254, 223, 80);
    let connection2 = SocketAddrV4::new(ip, 9991);

    // Send data via the socket
    let buf = &[0x01, 0x02, 0x03];
    socket.send_to(buf, connection2)?;
    println!("{:?}", buf);
	}
    Ok(())*/

//}
// ---------------------FUNCIONA
/*
fn snd()  -> Result<(), io::Error> {
    // Define the local connection (to send the data from)
    let ip = Ipv4Addr::new(10,254,223,79); //10.254.223.79
    let ipa = Ipv4Addr::new(10,254,223,80); //10.254.223.79
    let connection = SocketAddrV4::new(ip, 9991);

    // Bind the socket
    let socket = UdpSocket::bind(connection)?;

    // Define the remote connection (to send the data to)
    let connection2 = SocketAddrV4::new(ipa, 9991);

    // Send data via the socket
    let buf = &[0x01, 0x02, 0x03];
    socket.send_to(buf, connection2)?;
    println!("{:?}", buf);

    Ok(())
}

fn main() {
    match snd() {
        Ok(()) => println!("All snd-ing went well"),
        Err(err) => println!("Error: {:?}", err),
    }
}
///////////////////////////////
*/

extern crate dns_lookup;
use std::net;
use dns_lookup::lookup_host;

fn snd()  -> Result<(), io::Error> {
	let mut nomeHost : String = String::from("");
	for argument in env::args() {
    	println!("{}", argument);
    	nomeHost = String::from(argument);
	}

 	let ips = dns_lookup::lookup_host(&nomeHost).unwrap();
 	let mut ipOutroHost : Ipv4Addr = (Ipv4Addr::new(127, 0, 0, 1));

	for host in ips {
        println!("found address: {}", host);
       	if let IpAddr::V4(ipv4) = host {
	    	ipOutroHost = ipv4;
		}
    }

    // Define the local connection (to send the data from)
    let ip = Ipv4Addr::new(10,254,223,79); //10.254.223.79
    let ipa = Ipv4Addr::new(10,254,223,73); //10.254.223.79
    let connection = SocketAddrV4::new(ip, 9992);

    // Bind the socket
    let socket = UdpSocket::bind(connection)?;

    // Define the remote connection (to send the data to)
    let connection2 = SocketAddrV4::new(ipOutroHost, 9991);

    // Send data via the socket
    let buf = &[0x01, 0x02, 0x03];
    socket.send_to(buf, connection2)?;
    println!("{:?}", buf);


    let tred = thread::spawn(move || {
     
        // ---------------------------Cria socket da thread

        // Read from the socket
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf).expect("sdf");

        // Print only the valid data (slice)
        println!("{:?}", &buf[0 .. amt]);


    });

    let res = tred.join();



    Ok(())
}

use std::env;
fn main() {
	
    match snd() {
        Ok(()) => println!("All snd-ing went well"),
        Err(err) => println!("Error: {:?}", err),
    }
}
