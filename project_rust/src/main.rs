use std::io::prelude::*;
use std::net::TcpStream;

fn main()
{
    if let Ok(server_stream) = TcpStream::connect("127.0.0.1:25565")
    {
        println!("Connected to server.");
    }
    else 
    {
        println!("Connection failed...");    
    }
}
