
use std::net::UdpSocket; // https://doc.rust-lang.org/std/net/struct.UdpSocket.html
use std::thread;


fn main(){

    let socket = UdpSocket::bind("127.0.0.1:20071").expect("couldn't bind to address");
    socket.connect("127.0.0.1:20071");
    println!("Connected to server at 127.0.0.1:20071");

    let target_address = "127.0.0.1:30000";

    let socket_clone = socket.try_clone().expect("Failed to clone the socket");

    let messages = vec!["Test"];

    //Recive
    thread::spawn(move || {

        let mut buffer = [0;1024];

        loop{

            match socket.recv_from(&mut buffer){
                Ok((size,addr)) =>{
                    println!("Message recived:{:?}",&buffer);
                    buffer = [0;1024];
                }

                Err(e) => {
                    println!("Failure to recive");
                }
            }
        }
    });


    //Sending
    thread::spawn(move || {

        let mut i: u8 = 0;

        loop {
            i+=1;
            if i==u8::MAX{
                i=0;
            }
    
            match socket_clone.send_to(&[i] , target_address) {
                Ok(_) => println!("Sendt Current number: {}", i),
                Err(e) => eprintln!("Error sending data: {}", e),
            }
                thread::sleep(std::time::Duration::from_secs(1));
        }

    });

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }

}