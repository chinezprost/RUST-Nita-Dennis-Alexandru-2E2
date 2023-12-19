use std::error::Error;
use std::net::TcpStream;
use std::io::{self};

fn main() -> Result<(), Box<dyn Error>>
{
    
    let mut stream = TcpStream::connect("127.0.0.1:25565")?;
    handshake_serverbound("127.0.01", 25565, 2)?;
    
    Ok(())
}

fn handshake_serverbound(_address: &str, _port: u16, _state: i32) -> io::Result<Vec<u8>>
{
    let mut packet_to_be_sent = Vec::new();

    packet_to_be_sent.write_varint(0)?; // PacketID-ul
    packet_to_be_sent.write_varint(765)?; // Protocolul pentru 1.18
    packet_to_be_sent.write_string(_address)?; // Protocolul pentru 1.18
    packet_to_be_sent.write_u16(_port)?; // PORT-ul
    packet_to_be_sent.write_varint(_state)?;


    for i in &packet_to_be_sent
    {
        print!("{} ", i);
    }
    Ok(packet_to_be_sent.clone())

}
//trait-uri
trait WriteVarInt
{
    fn write_varint(&mut self, _value: i32) -> io::Result<()>;
}

trait WriteString
{
    fn write_string(&mut self, _value: &str) -> io::Result<()>;
}

trait WriteU16
{
    fn write_u16(&mut self, _value: u16) -> io::Result<()>;
}

//implementari
impl WriteVarInt for Vec<u8>
{
    fn write_varint(&mut self, mut _value: i32) -> io::Result<()>
    {
        loop
        {
            let mut octet = (_value & 0b01111111) as u8; // mă interesează cei 7 biți cei mai nesemnificativi
            _value = _value >> 7; // pregătesc următorii <= 7 biți pentru procesare
            if _value != 0
            {
                octet = octet | 0b10000000;  // dacă mai am biți de procesat, setez al 8-lea bit cu 1 să semnalez că mai am de lucru
            }

            self.push(octet); 
            if _value == 0 // daca _value = 0, atunci am terminat si ma opresc 
            {
                break;
            }
        }
        Ok(())
    }
}

impl WriteString for Vec<u8>
{
    fn write_string(&mut self, _value: &str) -> io::Result<()>
    {
        self.write_varint(_value.len() as i32)?;
        self.extend_from_slice(_value.as_bytes());
        Ok(())
    }
}

impl WriteU16 for Vec<u8>
{
    fn write_u16(&mut self, _value: u16) -> io::Result<()>
    {
        self.extend_from_slice(&_value.to_le_bytes());
        Ok(())
    }
}
