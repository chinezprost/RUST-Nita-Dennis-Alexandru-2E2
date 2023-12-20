use std::io::{self,  Write, Read};
use std::net::TcpStream;
use std::thread;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>
{
    
    let mut stream = TcpStream::connect("127.0.0.1:25565")?;
    //handshake_serverbound("127.0.01", 25565, 2)?;

    let stream_cloned = stream.try_clone()?;
    // let listen_to_server_thread = thread::spawn
    // (
    //     ||
    //     {
    //         if let Err(e) = receive_message(stream_cloned)
    //         {
    //             eprintln!("Couldn't receive message: {}.", e);
    //         }
    //     }
    // );

    stream.write_all(&handshake_serverbound("127.0.0.1", 25565, 1)?)?;
    stream.write_all(&[0x01, 0x00])?;

    let length = stream.read_varint()?;
    let packedid = stream.read_varint()?;
    let length2 = stream.read_varint()?;
    let json_string = stream.read_string()?;

    println!("string: {} {} {} {}", length, packedid, length2, json_string);


    stream.write_all(&login_start_serverbound("dennis")?)?;
    stream.flush()?;


    // listen_to_server_thread.join().unwrap();
    Ok(())
}


fn receive_message(mut stream: TcpStream) -> Result<(), Box<dyn Error>>
{
    let mut buffer = [0; 1024];
    loop
    {
        match stream.read(&mut buffer)
        {
            Ok(bytes_read) if bytes_read > 0
                =>
                {
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("I've received: {}", message);
                }
            
            Ok(_)
                =>
                {
                    println!("Connection lost.");
                }

            Err(e)
                =>
                {
                    eprintln!("Error: {}", e);
                    break;
                }
        }
    }

    Ok(())
}

fn handshake_serverbound(_address: &str, _port: u16, _state: i32) -> io::Result<Vec<u8>>
{
    let mut formed_packet = Vec::new();

    formed_packet.write_varint(0)?; // PacketID-ul
    formed_packet.write_varint(757)?; // Protocolul pentru 1.18
    formed_packet.write_string(_address, 255)?; // Adresa la care ma conectez // 127.0.0.1 (folosita si pt mc.minecraft-romania.ro:25565 de ex)
    formed_packet.write_u16(_port)?; // PORT-ul
    formed_packet.write_varint(_state)?;

    let mut final_packet: Vec<u8> = Vec::new();
    final_packet.push(formed_packet.len() as u8);
    for octet in formed_packet
    {
        final_packet.push(octet);
    }

    Ok(final_packet)

}

fn status_serverbound() -> io::Result<Vec<u8>>
{
    let mut packet_to_be_sent = Vec::new();

    packet_to_be_sent.write_varint(0)?;

    Ok(packet_to_be_sent)
}


fn login_start_serverbound(_username: &str) -> io::Result<Vec<u8>>
{
    let mut packet_to_be_sent = Vec::new();
    packet_to_be_sent.write_string(_username, 16)?;
    

    Ok(packet_to_be_sent)
}

//trait-uri
trait WriteVarInt
{
    fn write_varint(&mut self, _value: i32) -> io::Result<()>;
}

trait WriteString
{
    fn write_string(&mut self, _value: &str, _size: u32) -> io::Result<()>;
}

trait WriteU16
{
    fn write_u16(&mut self, _value: u16) -> io::Result<()>;
}

trait WriteLong
{
    fn write_long(&mut self, _value: i64) -> io::Result<()>;
}

trait WriteByte
{
    fn write_byte(&mut self, _value: i8) -> io::Result<()>;
}


trait ReadVarInt
{
    fn read_varint(&mut self) -> io::Result<i32>;
}

trait ReadU16
{
    fn read_u16(&mut self) -> io::Result<u16>;
}

trait ReadString
{
    fn read_string(&mut self) -> io::Result<String>;
}

trait ReadLong
{
    fn read_long(&mut self) -> io::Result<i64>;
}

trait ReadByte
{
    fn read_byte(&mut self) -> io::Result<i8>;
}


impl ReadString for TcpStream
{
    fn read_string(&mut self) -> io::Result<String> 
    {
        let size_to_be_read = self.read_varint()? as usize;
        let mut read_buffer = vec![0; size_to_be_read];

        let _ = self.read_exact(&mut read_buffer);

        let result = String::from_utf8(read_buffer);

        Ok(result.unwrap())
    }
}

impl ReadVarInt for TcpStream
{
    fn read_varint(&mut self) -> io::Result<i32>
    {
        let mut shift_pos = 0;
        let mut result = 0;

        loop
        {
            let mut octet = [0];
            self.read_exact(&mut octet)?;

            let octet = octet[0] as i32;
            result = (result | 0b0111_1111) << shift_pos;

            if(octet & 0b1000_0000) == 0
            {
                break;
            }

            shift_pos = shift_pos + 7;
        }

        Ok(result)
    }
}

impl ReadLong for TcpStream
{
    fn read_long(&mut self) -> io::Result<i64>
    {
        let mut read_buffer = [0; 8];
        self.read_exact(&mut read_buffer)?;
        let result = i64::from_le_bytes(read_buffer);

        Ok(result)
    }
}

impl ReadByte for TcpStream
{
    fn read_byte(&mut self) -> io::Result<i8>
    {
        let mut read_buffer = [0; 1];
        self.read_exact(&mut read_buffer)?;
        Ok(read_buffer[0] as i8)
    }
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
    fn write_string(&mut self, _value: &str, _size: u32) -> io::Result<()>
    {
        if _value.len() > _size as usize
        {
            println!("Write string failure.");
        }
        self.write_varint(_size as i32)?;
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

impl WriteLong for Vec<u8>
{
    fn write_long(&mut self, _value: i64) -> io::Result<()>
    {
        self.extend_from_slice(&_value.to_le_bytes());
        Ok(())
    }
}
