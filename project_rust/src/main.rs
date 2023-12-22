use core::time;
use std::borrow::Borrow;
use std::io::{self,  Write, Read};
use std::net::TcpStream;
use std::{result, vec};
use std::thread::{self};
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;

use flate2::read::{ZlibDecoder, self};
use flate2::write::ZlibEncoder;

const SEGMENT_BITS: i32 = 0b0111_1111;
const CONTINUE_BIT: i32 = 0b1000_0000;

const COMPRESSION_THRESHOLD: i32 = 128;

fn main() -> Result<(), Box<dyn Error>>
{
    
    let mut stream = TcpStream::connect("127.0.0.1:25564")?;
    // handshake_serverbound("127.0.01", 25565, 2)?;

    let next_state = 2;

    
    if next_state == 1
    {
        let handshake_packet_state_1 = handshake_serverbound("127.0.0.1", 25565, 1)?;

        stream.write_varint(handshake_packet_state_1.len() as i32)?;
        stream.write_all(&handshake_packet_state_1)?;


        stream.write_byte(0x01)?; // REQUEST STATUS
        stream.write_byte(0x00)?;


        let size = stream.read_varint()?;
        let packet_id = stream.read_varint()?;
        let read_json = stream.read_string()?;

        println!("{} {} {}", size, packet_id, read_json);

        stream.write_byte(0x09)?;
        stream.write_byte(0x01)?;
        stream.write_long(111)?;

        let size = stream.read_varint()?;
        let packed_id = stream.read_byte()?;
        let response = stream.read_long()?;

        println!("{} {} {}", size, packed_id, response);
        println!("end status");
    }
    else 
    {
        let handshake_packet_state_2 = handshake_serverbound("127.0.0.1", 25565, 2)?;

        stream.write_varint(handshake_packet_state_2.len() as i32)?;
        stream.write_all(&handshake_packet_state_2)?;
        

        let login_start = login_start_serverbound("dennis")?;
        stream.write_varint(login_start.len() as i32)?;
        stream.write_all(&login_start)?;

        // stream.write_byte(0x17)?;
        // stream.write_byte(0x00)?;
        // stream.write_string("dennis", 16)?;

        //set-compression packet
        let set_compression_packet_size = stream.read_varint()?;
        let set_compression_packet_id = stream.read_byte()?;
        let set_compression_threshold  = stream.read_varint()?;


        println!("set-compression-packet SIZE: {set_compression_packet_size}, ID: {set_compression_packet_id}, THRESHOLD: {set_compression_threshold}");

        // FROM NOW ON PACKETS ARE COMPRESSED
        // FROM NOW ON PACKETS ARE COMPRESSED
        // FROM NOW ON PACKETS ARE COMPRESSED
        
        let mut received_packet = decode_packet(stream.try_clone()?)?;
        println!("-{}-", received_packet.len());

        let packet_size = received_packet.read_varint()?;
        let packet_id = received_packet.read_varint()?;
        let received_uuid = received_packet.read_uuid()?;
        let received_username = received_packet.read_string()?;
        println!("{} {} {} {}", packet_size, packet_id, received_uuid, received_username);

        println!("Connected.");

        let mut chat_message_string: Vec<u8> = Vec::new();
        chat_message_string.write_string("hello", 256)?;
        let chat_message = encode_packet(0x03, &chat_message_string)?;
        println!("-{}-", chat_message.len());
        stream.write_all(&chat_message)?;
        //acknowledge the connection
    }


    // let stream_cloned = stream.try_clone()?;
    // let listen_to_server_thread = thread::spawn
    // (
    //     ||
    //     {
    //         let packet_size = stream.read_varint();
    //         let packet_id = stream.read_byte();


    //     }
    // );



    loop{}


    // listen_to_server_thread.join().unwrap();
    Ok(())
}

fn decode_packet(mut stream: TcpStream) -> io::Result<Vec<u8>>
{
    let compressed_packet_length = stream.read_varint()?;
    println!("compressed_packet_length: {}", compressed_packet_length);
    let data_length = stream.read_varint()?;
    println!("data_length: {}", data_length);


    let mut final_packet: Vec<u8> = Vec::new();

    if data_length > 0
    {
        let mut compressed_data = vec![0u8; compressed_packet_length as usize];
        stream.read_exact(&mut compressed_data)?;
        final_packet.write_varint(data_length)?;
        let mut decoded_array = Vec::new();
        let mut zlib_decoder = ZlibDecoder::new(compressed_data.as_slice());
        zlib_decoder.read_to_end(&mut decoded_array)?;
        final_packet.extend_from_slice(&decoded_array);
    }
    else
    {
        final_packet.write_varint(compressed_packet_length)?;
        let mut uncompressed_data = vec![0u8; compressed_packet_length as usize];
        stream.read_exact(&mut uncompressed_data)?;
        println!("uncompressed data size: {}", uncompressed_data.len());
        final_packet.extend_from_slice(&uncompressed_data);
    }
    Ok(final_packet)
}

fn encode_packet(mut packet_id: i32, mut data: &[u8]) -> io::Result<Vec<u8>>
{
    let mut final_packet: Vec<u8> = Vec::new();
    println!("data-len: {}", data.len());
    if COMPRESSION_THRESHOLD >= 0 && data.len() as i32 > COMPRESSION_THRESHOLD
    {
        let mut zlib_encoder = ZlibEncoder::new(Vec::new(), Default::default());
        
        let mut packet_id_as_vec = Vec::new();
        packet_id_as_vec.write_varint(packet_id)?;

        zlib_encoder.write_all(&packet_id_as_vec)?;
        zlib_encoder.write_all(data)?;

        let compressed_data = zlib_encoder.finish()?;

        final_packet.write_varint((compressed_data.len() + packet_id.get_varint_len()) as i32)?;
        final_packet.write_varint(compressed_data.len() as i32)?;
        final_packet.write_all(&compressed_data)?;

    }
    else
    {
        //println!("size: {}", final_packet.len());
        final_packet.write_varint((packet_id.get_varint_len() + data.len() + 1) as i32)?;
        //println!("size: {}", final_packet.len());

        final_packet.write_varint(0)?;
        //println!("size: {}", final_packet.len());

        final_packet.write_varint(packet_id)?;
        //println!("size: {}", final_packet.len());

        final_packet.write_all(data)?;
        //println!("size: {}", final_packet.len());

    }

    Ok(final_packet)
}


fn handshake_serverbound(_address: &str, _port: u16, _state: i32) -> io::Result<Vec<u8>>
{
    let mut formed_packet = Vec::new();

    formed_packet.write_byte(0x00)?; // PacketID-ul
    formed_packet.write_varint(757)?; // Protocolul pentru 1.18
    formed_packet.write_string(_address, 255)?; // Adresa la care ma conectez // 127.0.0.1 (folosita si pt mc.minecraft-romania.ro:25565 de ex)
    formed_packet.write_u16(_port)?; // PORT-ul
    formed_packet.write_varint(_state)?;

    Ok(formed_packet)
}

fn login_start_serverbound(_username: &str) -> io::Result<Vec<u8>>
{
    let mut formed_packet = Vec::new();
    if _username.len() > 16 || _username.len() == 0
    {
        println!("Invalid username.");
    }
    formed_packet.write_byte(0x00)?;
    formed_packet.write_string(_username, 16)?;

    Ok(formed_packet)
}


//trait-uri

trait VarIntLength
{
    fn get_varint_len(&self) -> usize;
}

impl VarIntLength for i32
{
    fn get_varint_len(&self) -> usize
    {
        let mut varint_length = 0;
        let mut value = *self;
        loop
        {
            varint_length = varint_length + 1;

            value >>= 7;

            if value == 0
            {
                break;
            }
        }
        return varint_length;
    }
}



trait WriteVarInt
{
    fn write_varint(&mut self, _value: i32) -> io::Result<()>;
}

trait ReadVarInt
{
    fn read_varint(&mut self) -> io::Result<i32>;
}



trait WriteString
{
    fn write_string(&mut self, _value: &str, _size: u32) -> io::Result<()>;
}

trait ReadString
{
    fn read_string(&mut self) -> io::Result<String>;
}


trait WriteU16
{
    fn write_u16(&mut self, _value: u16) -> io::Result<()>;
}

trait ReadU16
{
    fn read_u16(&mut self) -> io::Result<u16>;
}


trait WriteVarLong
{
    fn write_varlong(&mut self, _value: i64) -> io::Result<()>;
}

trait ReadVarLong
{
    fn read_varlong(&mut self) -> io::Result<i64>;
}



trait WriteByte
{
    fn write_byte(&mut self, _value: u8) -> io::Result<()>;
}

trait ReadByte
{
    fn read_byte(&mut self) -> io::Result<i8>;
}

trait ReadLong
{
    fn read_long(&mut self) -> io::Result<i64>;
}

trait WriteLong
{
    fn write_long(&mut self, _value: i64) -> io::Result<()>;
}


trait WriteUUID{
    fn write_uuid(&mut self, _value: i128) -> io::Result<()>;
}

trait ReadUUID
{
    fn read_uuid(&mut self) -> io::Result<i128>;
}

impl ReadUUID for TcpStream
{
    fn read_uuid(&mut self) -> io::Result<i128>
    {
        let mut read_buffer = [0; 16];
        self.read_exact(&mut read_buffer)?;
        Ok(i128::from_le_bytes(read_buffer))

    }
}

impl ReadUUID for Vec<u8>
{
    fn read_uuid(&mut self) -> io::Result<i128>
    {
        let mut read_buffer:[u8; 16] = [0; 16];

        for i in 0..15
        {
            read_buffer[i] = self.read_byte()? as u8;
        }
        Ok(i128::from_le_bytes(read_buffer))
    }
}


trait ReadJSONString
{
    fn read_json_string(&mut self, _length: usize) -> io::Result<String>;
}





/////////////////////////////////////////////////////////////////////////////////////////////////////// VARINT
impl WriteVarInt for TcpStream
{
    fn write_varint(&mut self, mut _value: i32) -> io::Result<()>
    {
        loop
        {
            if (_value & !SEGMENT_BITS) == 0
            {
                self.write_byte(_value as u8)?;
                break;
            }

            self.write_byte(((_value & SEGMENT_BITS) | CONTINUE_BIT) as u8)?;

            _value = ((_value as u32) >> 7) as i32;
            
            //let aux: u32 = {let bytes = _value.to_be_bytes(); u32::from_be_bytes(bytes)};
            //_value = (aux >> 7) as i32;

            //_value = _value >> 7 & (!(!0u32 >> 7)) as i32;
        }
        Ok(())
    }
}

impl WriteVarInt for Vec<u8>
{
    fn write_varint(&mut self, mut _value: i32) -> io::Result<()>
    {
        loop
        {
            if (_value & !SEGMENT_BITS) == 0
            {
                self.push(_value as u8);
                break;
            }

            self.push(((_value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

            _value = ((_value as u32) >> 7) as i32;
            
            //let aux: u32 = {let bytes = _value.to_be_bytes(); u32::from_be_bytes(bytes)};
            //_value = (aux >> 7) as i32;

            //_value = _value >> 7 & (!(!0u32 >> 7)) as i32;
        }
        Ok(())
    }
}


impl ReadVarInt for TcpStream
{
    fn read_varint(&mut self) -> io::Result<i32>
    {
        let mut value: i32 = 0;
        let mut position: i32 = 0;
        let mut current_octet: u8;

        loop
        {
            current_octet = self.read_byte()? as u8;
            value |= (i32::from(SEGMENT_BITS as u8 & current_octet) << position) as i32;

            if (current_octet & CONTINUE_BIT as u8) == 0
            {
                break;
            }

            position += 7;
        }
        if position >= 32
        {
            println!("VarInt too big!");
        } 
        Ok(value)
    }
}

impl ReadVarInt for Vec<u8>
{
    fn read_varint(&mut self) -> io::Result<i32>
    {
        let mut value: i32 = 0;
        let mut position: i32 = 0;
        let mut current_octet: u8;

        loop
        {
            current_octet = self.read_byte()? as u8;
            value |= (i32::from(SEGMENT_BITS as u8 & current_octet) << position) as i32;

            if (current_octet & CONTINUE_BIT as u8) == 0
            {
                break;
            }

            position += 7;
        }
        if position >= 32
        {
            println!("VarInt too big!");
        } 
        Ok(value)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// STRING

impl WriteString for TcpStream
{
    fn write_string(&mut self, _value: &str, _size: u32) -> io::Result<()>
    {
        let mut result = Vec::new();
        if _value.len() > _size as usize
        {
            println!("Write string failure.");
        }
        result.write_varint(_value.len() as i32)?;
        result.extend_from_slice(_value.as_bytes());
        self.write_all(&result)?;
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
        self.write_varint((_value.len()) as i32)?;
        self.extend_from_slice(_value.as_bytes());
        Ok(())
    }
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

impl ReadString for Vec<u8>
{
    fn read_string(&mut self) -> io::Result<String> 
    {
        self.read_byte()?;
        let size_to_be_read = self.read_varint()? as usize;
        println!("size: {}", size_to_be_read);
        let mut read_buffer = vec![0; size_to_be_read];
        for i in 0..size_to_be_read
        {
            read_buffer[i] = self.read_byte()? as u8;
        }
        let result = String::from_utf8(read_buffer);
        Ok(result.unwrap())
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// VARLONG

impl WriteVarLong for TcpStream
{
    fn write_varlong(&mut self, mut _value: i64) -> io::Result<()>
    {
        loop
        {
            if (_value & !SEGMENT_BITS as i64) == 0
            {
                self.write_byte(_value as u8)?;
                break;
            }

            self.write_byte(((_value & 0x07) | CONTINUE_BIT as i64) as u8)?;

            _value >>= 7;
        }
        Ok(())
    }
}

impl WriteVarLong for Vec<u8>
{
    fn write_varlong(&mut self, mut _value: i64) -> io::Result<()>
    {
        loop
        {
            if (_value & !SEGMENT_BITS as i64) == 0
            {
                self.push(_value as u8);
                break;
            }

            self.push(((_value & 0x07) | CONTINUE_BIT as i64) as u8);

            _value >>= 7;
        }
        Ok(())
    }
}

impl ReadVarLong for TcpStream
{
    fn read_varlong(&mut self) -> io::Result<i64>
    {
        let mut value: i64 = 0;
        let mut position: i64 = 0;
        let mut current_octet: u8;

        loop
        {
            current_octet = self.read_byte()? as u8;
            value |= (i32::from(SEGMENT_BITS as u8 & current_octet) << position) as i64;

            if (current_octet & CONTINUE_BIT as u8) == 0
            {
                break;
            }

            position += 7;
        }
        if position >= 64
        {
            println!("VarLong too big!");
        } 

        Ok(value)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// BYTE

impl WriteByte for TcpStream
{
    fn write_byte(&mut self, _value: u8) -> io::Result<()> 
    {
        self.write_all(&_value.to_be_bytes())?;
        Ok(())
    }
}

impl WriteByte for Vec<u8>
{
    fn write_byte(&mut self, _value: u8) -> io::Result<()> 
    {
        self.extend_from_slice(&_value.to_be_bytes());
        Ok(())
    }
}

impl ReadByte for TcpStream
{
    fn read_byte(&mut self) -> io::Result<i8>
    {
        let mut read_buffer = [0; 1];
        self.read_exact(&mut read_buffer)?;
        Ok(read_buffer[0].to_le() as i8)
    }
}

impl ReadByte for Vec<u8>
{
    fn read_byte(&mut self) -> io::Result<i8>
    {
        let mut result: i8 = 0;
        if self.len() > 0
        {
            result = self.remove(0) as i8;
        }
        Ok(result)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// U16
/// 
impl WriteU16 for TcpStream
{
    fn write_u16(&mut self, _value: u16) -> io::Result<()>
    {
        self.write_all(&_value.to_be_bytes())?;
        Ok(())
    }
}

impl WriteU16 for Vec<u8>
{
    fn write_u16(&mut self, _value: u16) -> io::Result<()>
    {
        self.extend_from_slice(&_value.to_be_bytes());
        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// LONG

impl WriteLong for TcpStream
{
    fn write_long(&mut self, _value: i64) -> io::Result<()> 
    {
        self.write_all(&_value.to_be_bytes())?;
        Ok(())
    }
}

impl WriteLong for Vec<u8>
{
    fn write_long(&mut self, _value: i64) -> io::Result<()> 
    {
        self.extend_from_slice(&_value.to_be_bytes());
        Ok(())
    }
}

impl ReadLong for TcpStream
{
    fn read_long(&mut self) -> io::Result<i64> 
    {
        let mut read_buffer = [0; 8];
        self.read_exact(&mut read_buffer)?;
        
        Ok(i64::from_be_bytes(read_buffer))
    }
}


/////////////////////////////////////////////////////////////////////////////////////////////////////// OTHERS

impl ReadJSONString for TcpStream
{
    fn read_json_string(&mut self, _length: usize) -> io::Result<String> 
    {
        let mut read_buffer = vec![0; 8];
        let _ = self.read_exact(&mut read_buffer);
        let result = String::from_utf8(read_buffer);
        Ok(result.unwrap())
    }
}

impl WriteUUID for Vec<u8>
{
    fn write_uuid(&mut self, _value: i128) -> io::Result<()> 
    {
        self.extend_from_slice(&_value.to_be_bytes());
        Ok(())
    }
}

impl WriteUUID for TcpStream
{
    fn write_uuid(&mut self, _value: i128) -> io::Result<()> 
    {
        self.write_all(&_value.to_be_bytes())?;
        Ok(())
    }
}


