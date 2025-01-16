
use std::net::UdpSocket; // https://doc.rust-lang.org/std/net/struct.UdpSocket.html
use std::thread;

pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr)>

pub fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> Result<usize>

fn main(){

    let socket = UdpSocket::bind("127.0.0.1:20071").expect("couldn't bind to address");
    socket.connect("127.0.0.1:20071")?;
    println!("Connected to server at 127.0.0.1:20071");

    let target_address = "127.0.0.1:30000";

    let socket_clone = socket.try_clone().expect("Failed to clone the socket");

    let messages = vec!["Test"];


    thread::spawn(move || {

        let mut buffer = [0;1024];

        while true{

            match socket.recv_from(&mut buffer){
                Ok((size,addr)) =>{
                    println!("Message recived:{}",buffer);
                    buffer.clear();
                }

                Err(e) => {
                    println!("Failure to recive");
                }
            }
        }
    });

    thread::spawn(move || {

        let mut i: u32 = 0;

        while true {
            i++;
            if i>=u32::MAX{
                i=0;
            }
    
            match socket_clone.send_to(i.as_bytes(), target_address) {
                Ok(_) => println!("Current number: {}", i),
                Err(e) => eprintln!("Error sending data: {}", e),
            }
                thread::sleep(std::time::Duration::from_secs(1));
        }

    });

    while true {
        thread::sleep(std::time::Duration::from_secs(1));
    }

}