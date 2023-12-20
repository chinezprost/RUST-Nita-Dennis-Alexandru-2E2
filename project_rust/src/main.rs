use std::io::{self,  Write, Read};
use std::net::TcpStream;
use std::ptr::null;
use std::thread::{self, current};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>
{
    
    let mut stream = TcpStream::connect("127.0.0.1:25565")?;
    //handshake_serverbound("127.0.01", 25565, 2)?;

    //let stream_cloned = stream.try_clone()?;
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
    stream.write_byte(1)?; // REQUEST STATUS
    stream.write_byte(0x00)?;

    let length = stream.read_varint()?; // STATUS RESPONSE
    let packedid = stream.read_varint()?;
    let length2 = stream.read_varint()?;
    let json_string = stream.read_json_string(length2 as usize)?;

    stream.write_byte(9)?;
    stream.write_byte(0x01)?;    
    stream.write_long(12)?;
        
    let ping_request_size = stream.read_varint()?;
    let packed_id = stream.read_varint()?;
    let ping_received_long = stream.read_long()?;


    println!("STATUS: response: packedlenght: {} packetid: {} jsonlength: {} json_string: {}", length, packedid, length2, json_string);
    println!("PING: ping_size: {} ping_pack_id: {} ping_recv_long: {}", ping_request_size, packed_id, ping_received_long);

    loop{}
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

trait ReadJSONString
{
    fn read_json_string(&mut self, _length: usize) -> io::Result<String>;
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

impl ReadJSONString for TcpStream
{
    fn read_json_string(&mut self, _length: usize) -> io::Result<String> 
    {
        let mut read_buffer = vec![0; _length-3];
        let _ = self.read_exact(&mut read_buffer);
        let result = String::from_utf8(read_buffer);
        Ok(result.unwrap())
    }
}

// impl ReadVarInt for TcpStream
// {
//     fn read_varint(&mut self) -> io::Result<i32>
//     {
//         let mut shift_pos = 0;
//         let mut result = 0;

//         loop
//         {
//             let mut octet = [0];
//             self.read_exact(&mut octet)?;

//             let octet = octet[0] as i32;
//             result = (result | 0b0111_1111) << shift_pos;

//             if(octet & 0b1000_0000) == 0
//             {
//                 break;
//             }

//             shift_pos = shift_pos + 7;
//         }

//         Ok(result)
//     }
// }

impl ReadVarInt for TcpStream
{
    fn read_varint(&mut self) -> io::Result<i32>
    {
        let mut value: i32 = 0;
        let mut position = 0;
        let mut currentOctet: i8;

        loop
        {
            currentOctet = self.read_byte()?;
            value |= (currentOctet as i32 & 0x7F) << position;

            if (currentOctet as i32 & 0x80) == 0
            {
                break;
            }

            position += 7;
        }

        Ok(value)
    }
}



impl ReadLong for TcpStream
{
    fn read_long(&mut self) -> io::Result<i64>
    {
        let mut value: i64 = 0;
        let mut position = 0;
        let mut currentOctet: i8;

        loop
        {
            currentOctet = self.read_byte()?;
            value |= (currentOctet as i64 & 0x7F) << position;

            if (currentOctet as i64 & 0x80) == 0
            {
                break;
            }

            position += 7;
        }

        Ok(value)
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

impl WriteByte for Vec<u8>
{
    fn write_byte(&mut self, _value: i8) -> io::Result<()> 
    {
        self.extend_from_slice(&_value.to_le_bytes());
        Ok(())
    }
}


//for tcpstream
impl WriteVarInt for TcpStream
{
    fn write_varint(&mut self, mut _value: i32) -> io::Result<()>
    {
        let mut result = Vec::new();
        loop
        {
            let mut octet = (_value & 0b01111111) as u8; // mă interesează cei 7 biți cei mai nesemnificativi
            _value = _value >> 7; // pregătesc următorii <= 7 biți pentru procesare
            if _value != 0
            {
                octet = octet | 0b10000000;  // dacă mai am biți de procesat, setez al 8-lea bit cu 1 să semnalez că mai am de lucru
            }

            result.push(octet); 
            if _value == 0 // daca _value = 0, atunci am terminat si ma opresc 
            {
                break;
            }
        }
        self.write_all(&result)?;
        Ok(())
    }
}

impl WriteString for TcpStream
{
    fn write_string(&mut self, _value: &str, _size: u32) -> io::Result<()>
    {
        let mut result = Vec::new();
        if _value.len() > _size as usize
        {
            println!("Write string failure.");
        }
        result.write_varint(_size as i32)?;
        result.extend_from_slice(_value.as_bytes());

        self.write_all(&result)?;
        Ok(())
    }
}

impl WriteU16 for TcpStream
{
    fn write_u16(&mut self, _value: u16) -> io::Result<()>
    {
        let mut result = Vec::new();
        result.extend_from_slice(&_value.to_le_bytes());
        self.write_all(&result)?;
        Ok(())
    }
}

impl WriteLong for TcpStream
{
    fn write_long(&mut self, _value: i64) -> io::Result<()>
    {
        let mut result = Vec::new();
        result.extend_from_slice(&_value.to_le_bytes());
        self.write_all(&result)?;
        Ok(())
    }
}

impl WriteByte for TcpStream
{
    fn write_byte(&mut self, _value: i8) -> io::Result<()> 
    {
        self.write_all(&[_value as u8])?;
        Ok(())
    }
}
