use core::time;
use flate2::read::{self, ZlibDecoder};
use flate2::write::ZlibEncoder;
use std::borrow::Borrow;
use std::error::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::{self};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{result, vec};

const SEGMENT_BITS: i32 = 0b0111_1111;
const CONTINUE_BIT: i32 = 0b1000_0000;

const COMPRESSION_THRESHOLD: i32 = 128;
#[derive(Clone)]
struct CurrentUserList {
    online_players_count: i32,
    online_players_list: Vec<(i128, String)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:25564")?;

    // handshake_serverbound("127.0.01", 25565, 2)?;

    let next_state = 2;

    let current_player_list = Arc::new(Mutex::new(CurrentUserList {
        online_players_count: 0,
        online_players_list: Vec::new(),
    }));

    let current_player_list_clone = Arc::clone(&current_player_list);
    let current_player_list_clone2 = Arc::clone(&current_player_list);

    if next_state == 1 {
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
    } else {
        let handshake_packet_state_2 = handshake_serverbound("127.0.0.1", 25564, 2)?;

        stream.write_varint(handshake_packet_state_2.len() as i32)?;
        stream.write_all(&handshake_packet_state_2)?;

        let login_start = login_start_serverbound("dennis")?;
        stream.write_varint(login_start.len() as i32)?;
        stream.write_all(&login_start)?;

        // stream.write_byte(0x17)?;
        // stream.write_byte(0x00)?;
        // stream.write_string("dennis", 16)?;

        //set-compression packet
        let _ = stream.read_varint()?;
        let _ = stream.read_byte()?;
        let packet_threshold = stream.read_varint()?;
        println!("[SERVER]: Packet Threshold set to {}.", packet_threshold);

        // FROM NOW ON PACKETS ARE COMPRESSED
        // FROM NOW ON PACKETS ARE COMPRESSED
        // FROM NOW ON PACKETS ARE COMPRESSED
        let mut stream_cloned = stream.try_clone()?;
        let mut send_packet_stream = stream.try_clone()?;

        let listen_to_server_thread = thread::spawn(move || {
            let mut has_read_login_succes = false;
            loop {
                let mut received_packet =
                    decode_packet(stream_cloned.try_clone().unwrap()).unwrap();

                if received_packet.len() == 0 {
                    continue;
                }
                let packet_size = received_packet.read_varint().unwrap();
                let packet_id = received_packet.read_varint().unwrap();

                match packet_id {
                    0x03 => {}
                    0x02 => {
                        if has_read_login_succes == true {
                            continue;
                        }
                        println!("Valid Packet. ID: {}", packet_id);
                        let received_uuid = received_packet.read_uuid().unwrap();
                        let received_username = received_packet.read_string().unwrap();
                        println!(
                            "{} {} {} {}",
                            packet_size, packet_id, received_uuid, received_username
                        );
                        println!("Connected.");
                        has_read_login_succes = true;
                    }
                    0x0F => {
                        println!("Valid Packet. ID: {}", packet_id);

                        let received_message = received_packet.read_string().unwrap();
                        let received_position = received_packet.read_byte().unwrap();
                        let received_sender = received_packet.read_uuid().unwrap();

                        println!(
                            "{} {} {}",
                            received_message, received_position, received_sender
                        );
                    }
                    0x00 => {
                        //println!("Server Spawn:");
                        let entity_id = received_packet.read_varint().unwrap();
                        let object_uuid = received_packet.read_uuid().unwrap();
                        let entity_type = received_packet.read_varint().unwrap();

                        //println!("{} {} {}", entity_id, object_uuid, entity_type);
                    }
                    0x21 => {
                        println!("Valid Packet. ID: {}", packet_id);
                        let received_keep_alive_long = received_packet.read_long().unwrap();
                        println!("From server Long: {}", received_keep_alive_long);

                        let mut keep_alive_packet: Vec<u8> = Vec::new();
                        keep_alive_packet
                            .write_long(received_keep_alive_long)
                            .expect("Couldn't write long.");
                        let encoded_packet = encode_packet(0x0F, &keep_alive_packet)
                            .expect("Couldn't encode packet.");

                        send_packet_stream
                            .write_all(&encoded_packet)
                            .expect("Couldnt write packet.");
                    }
                    0x36 => {
                        let pack_action =
                            received_packet.read_varint().expect("Couldn't read action");
                        let number_of_players = received_packet
                            .read_varint()
                            .expect("Coulnd't read number of players");

                        let mut current_player_list = current_player_list_clone.lock().unwrap();

                        for j in 0..number_of_players {
                            let player_uuid = received_packet.read_uuid().unwrap();
                            match pack_action {
                                0 => {
                                    let username = received_packet.read_string().unwrap();
                                    //ignore
                                    let no_of_properties = received_packet.read_varint().unwrap();
                                    for i in 0..no_of_properties {
                                        received_packet.read_string();
                                        received_packet.read_string();
                                        let is_signed = received_packet.read_byte().unwrap();
                                        if is_signed == 0x01 {
                                            received_packet.read_string();
                                        }
                                    }
                                    received_packet.read_varint();
                                    received_packet.read_varint();
                                    received_packet.read_byte();
                                    received_packet.read_string();
                                    //ignore

                                    current_player_list.online_players_count =
                                        number_of_players + 1;
                                    if !current_player_list
                                        .online_players_list
                                        .iter()
                                        .any(|&(i, _)| i == player_uuid)
                                    {
                                        current_player_list
                                            .online_players_list
                                            .push((player_uuid, username));
                                    }
                                }
                                1 => {
                                    received_packet.read_varint();
                                }
                                2 => {
                                    received_packet.read_varint();
                                }
                                3 => {
                                    received_packet.read_byte();
                                    received_packet.read_string();
                                }
                                4 => {
                                    current_player_list.online_players_count =
                                        number_of_players - 1;
                                    if let Some(pos) = current_player_list
                                        .online_players_list
                                        .iter()
                                        .position(|&(i, _)| i == player_uuid)
                                    {
                                        current_player_list.online_players_list.remove(pos);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    0x1A => {
                        let received_disconnect_message = received_packet.read_string().unwrap();
                        println!("{}", received_disconnect_message);
                    }
                    _ => (),
                }
            }
        });

        // let heartbeat_thread = thread::spawn
        // (
        //     {

        //     }
        // );

        //send info packet
        // let mut client_settings_packet: Vec<u8> = Vec::new();
        // client_settings_packet.write_string("en_GB")?;
        // client_settings_packet.write_byte(8)?;
        // client_settings_packet.write_varint(0x00)?;
        // client_settings_packet.write_byte(0x01)?;
        // client_settings_packet.write_byte(0x01)?;
        // client_settings_packet.write_varint(0x01)?;
        // client_settings_packet.write_byte(0x00)?;
        // client_settings_packet.write_byte(0x01)?;

        // let client_settings_encoded_packet = encode_packet(0x05, &client_settings_packet)?;
        // stream.write_all(&client_settings_encoded_packet)?;
        //end send info packet

        // let mut chat_message_string: Vec<u8> = Vec::new();
        // chat_message_string.write_string("hello")?;
        // let chat_message = encode_packet(0x03, &chat_message_string)?;

        // stream.write_all(&chat_message)?;
        //acknowledge the connection
    }

    // send commands loop
    let client_logic = thread::spawn(move || loop {
        let mut _input_command = String::new();
        io::stdin()
            .read_line(&mut _input_command)
            .expect("Couldn't read from console.");

        let mut _input_command_split = _input_command.split_once(" ");

        let mut _input_command_split = _input_command.split_once(" ");

        if let Some((command_type, rest)) = _input_command_split {
            println!("Command Type: {}", command_type);
            println!("Rest: {}", rest);

            match command_type {
                "s" => {
                    let mut chat_message_string: Vec<u8> = Vec::new();
                    chat_message_string
                        .write_string(rest)
                        .expect("Couldn't write string");
                    let chat_message = encode_packet(0x03, &chat_message_string)
                        .expect("Couldn't encode chat message");
                    stream.write_all(&chat_message).expect("Couldn't write.");
                }
                "p" => {
                    let current_player_list = current_player_list_clone2.lock().unwrap();
                    println!("============================================");
                    println!(
                        "======= There are {} players online ========",
                        current_player_list.clone().online_players_count
                    );
                    for i in &current_player_list.clone().online_players_list {
                        println!("{} {}", i.1, i.0);
                    }
                    println!("============================================");
                }
                _ => (),
            }
        } else {
            println!("No input.");
        }
    });
    client_logic.join().unwrap();
    Ok(())
}

fn decode_packet(mut stream: TcpStream) -> io::Result<Vec<u8>> {
    let compressed_packet_length = stream.read_varint()?; // 1
    let data_length = stream.read_varint()?; // 1

    if compressed_packet_length == 0 {
        return Ok(Vec::new());
    }

    let mut final_packet: Vec<u8> = Vec::new();

    if data_length > 0 {
        //println!("Decoding compressed: C-Size: {compressed_packet_length}, U-Size: {data_length}.");
        let mut compressed_data =
            vec![0u8; (compressed_packet_length - data_length.get_varint_len() as i32) as usize];
        stream.read_exact(&mut compressed_data)?;
        final_packet.write_varint(data_length)?;

        let mut decoded_array = Vec::new();
        let mut zlib_decoder = ZlibDecoder::new(compressed_data.as_slice());

        match zlib_decoder.read_to_end(&mut decoded_array) {
            Ok(_) => {
                final_packet.extend_from_slice(&decoded_array);
            }
            Err(err) => {
                eprintln!("Error decompressing data: {:?}", err);
            }
        }
    } else {
        //println!("Decoding uncompressed: C-Size: {compressed_packet_length}, U-Size: {data_length}.");
        final_packet.write_varint(compressed_packet_length)?;
        let mut uncompressed_data =
            vec![0u8; (compressed_packet_length - data_length.get_varint_len() as i32) as usize];
        stream.read_exact(&mut uncompressed_data)?;
        final_packet.extend_from_slice(&uncompressed_data);
    }
    return Ok(final_packet);
}

fn encode_packet(mut packet_id: i32, mut data: &[u8]) -> io::Result<Vec<u8>> {
    //println!("Sending compressed packet with ID: {}", packet_id);
    let mut final_packet: Vec<u8> = Vec::new();
    if COMPRESSION_THRESHOLD >= 0 && data.len() as i32 > COMPRESSION_THRESHOLD {
        let mut zlib_encoder = ZlibEncoder::new(Vec::new(), Default::default());

        let mut packet_id_as_vec = Vec::new();
        packet_id_as_vec.write_varint(packet_id)?;

        zlib_encoder.write_all(&packet_id_as_vec)?;
        zlib_encoder.write_all(data)?;

        let compressed_data = zlib_encoder.finish()?;

        final_packet
            .write_varint((compressed_data.len() + packet_id.get_varint_len() + 1) as i32)?;
        final_packet.write_varint(compressed_data.len() as i32)?;
        final_packet.write_all(&compressed_data)?;
    } else {
        println!("Sending uncompressed packet with ID: {}", packet_id);
        //println!("size: {}", final_packet.len());
        final_packet.write_varint((packet_id.get_varint_len() + data.len() + 1) as i32)?;
        // println!(
        //     "size: {} {}",
        //     final_packet.len(),
        //     packet_id.get_varint_len() + data.len() + 1
        // );

        final_packet.write_varint(0)?;
        //println!("size: {}", final_packet.len());

        final_packet.write_varint(packet_id)?;
        //println!("size: {} {}", final_packet.len(), packet_id);

        final_packet.write_all(data)?;
        //println!("size: {}", final_packet.len());
    }

    for i in &final_packet {
        print!("{} ", i);
    }
    Ok(final_packet)
}

fn handshake_serverbound(_address: &str, _port: u16, _state: i32) -> io::Result<Vec<u8>> {
    let mut formed_packet = Vec::new();

    formed_packet.write_byte(0x00)?; // PacketID-ul
    formed_packet.write_varint(757)?; // Protocolul pentru 1.18
    formed_packet.write_string(_address)?; // Adresa la care ma conectez // 127.0.0.1 (folosita si pt mc.minecraft-romania.ro:25565 de ex)
    formed_packet.write_u16(_port)?; // PORT-ul
    formed_packet.write_varint(_state)?;

    Ok(formed_packet)
}

fn login_start_serverbound(_username: &str) -> io::Result<Vec<u8>> {
    let mut formed_packet = Vec::new();
    if _username.len() > 16 || _username.len() == 0 {
        println!("Invalid username.");
    }
    formed_packet.write_byte(0x00)?;
    formed_packet.write_string(_username)?;

    Ok(formed_packet)
}

//trait-uri

trait VarIntLength {
    fn get_varint_len(&self) -> usize;
}

impl VarIntLength for i32 {
    fn get_varint_len(&self) -> usize {
        let mut varint_length = 0;
        let mut value = *self;
        loop {
            varint_length = varint_length + 1;

            value >>= 7;

            if value == 0 {
                break;
            }
        }
        return varint_length;
    }
}

trait WriteVarInt {
    fn write_varint(&mut self, _value: i32) -> io::Result<()>;
}

trait ReadVarInt {
    fn read_varint(&mut self) -> io::Result<i32>;
}

trait WriteString {
    fn write_string(&mut self, _value: &str) -> io::Result<()>;
}

trait ReadString {
    fn read_string(&mut self) -> io::Result<String>;
}

trait WriteU16 {
    fn write_u16(&mut self, _value: u16) -> io::Result<()>;
}

trait ReadU16 {
    fn read_u16(&mut self) -> io::Result<u16>;
}

trait WriteVarLong {
    fn write_varlong(&mut self, _value: i64) -> io::Result<()>;
}

trait ReadVarLong {
    fn read_varlong(&mut self) -> io::Result<i64>;
}

trait WriteByte {
    fn write_byte(&mut self, _value: u8) -> io::Result<()>;
}

trait ReadByte {
    fn read_byte(&mut self) -> io::Result<i8>;
}

trait ReadLong {
    fn read_long(&mut self) -> io::Result<i64>;
}

trait WriteLong {
    fn write_long(&mut self, _value: i64) -> io::Result<()>;
}

trait WriteUUID {
    fn write_uuid(&mut self, _value: i128) -> io::Result<()>;
}

trait ReadUUID {
    fn read_uuid(&mut self) -> io::Result<i128>;
}

impl ReadUUID for TcpStream {
    fn read_uuid(&mut self) -> io::Result<i128> {
        let mut read_buffer = [0; 16];
        self.read_exact(&mut read_buffer)?;
        Ok(i128::from_le_bytes(read_buffer))
    }
}

impl ReadUUID for Vec<u8> {
    fn read_uuid(&mut self) -> io::Result<i128> {
        let mut read_buffer: [u8; 16] = [0; 16];

        for i in 0..16 {
            read_buffer[i] = self.read_byte()? as u8;
        }
        Ok(i128::from_le_bytes(read_buffer))
    }
}

trait ReadJSONString {
    fn read_json_string(&mut self, _length: usize) -> io::Result<String>;
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// VARINT
impl WriteVarInt for TcpStream {
    fn write_varint(&mut self, mut _value: i32) -> io::Result<()> {
        loop {
            if (_value & !SEGMENT_BITS) == 0 {
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

impl WriteVarInt for Vec<u8> {
    fn write_varint(&mut self, mut _value: i32) -> io::Result<()> {
        loop {
            if (_value & !SEGMENT_BITS) == 0 {
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

impl ReadVarInt for TcpStream {
    fn read_varint(&mut self) -> io::Result<i32> {
        let mut value: i32 = 0;
        let mut position: i32 = 0;
        let mut current_octet: u8;

        loop {
            current_octet = self.read_byte()? as u8;
            value |= (i32::from(SEGMENT_BITS as u8 & current_octet) << position) as i32;

            if (current_octet & CONTINUE_BIT as u8) == 0 {
                break;
            }

            position += 7;
        }
        if position >= 32 {
            println!("VarInt too big!");
        }
        Ok(value)
    }
}

impl ReadVarInt for Vec<u8> {
    fn read_varint(&mut self) -> io::Result<i32> {
        let mut value: i32 = 0;
        let mut position: i32 = 0;
        let mut current_octet: u8;

        loop {
            current_octet = self.read_byte()? as u8;
            value |= (i32::from(SEGMENT_BITS as u8 & current_octet) << position) as i32;

            if (current_octet & CONTINUE_BIT as u8) == 0 {
                break;
            }

            position += 7;
        }
        if position >= 32 {
            println!("VarInt too big!");
        }
        Ok(value)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// STRING

impl WriteString for TcpStream {
    fn write_string(&mut self, _value: &str) -> io::Result<()> {
        let mut result = Vec::new();
        result.write_varint(_value.len() as i32)?;
        result.extend_from_slice(_value.as_bytes());
        self.write_all(&result)?;
        Ok(())
    }
}

impl WriteString for Vec<u8> {
    fn write_string(&mut self, _value: &str) -> io::Result<()> {
        self.write_varint((_value.len()) as i32)?;
        self.extend_from_slice(_value.as_bytes());
        Ok(())
    }
}

impl ReadString for TcpStream {
    fn read_string(&mut self) -> io::Result<String> {
        let size_to_be_read = self.read_varint()? as usize;
        let mut read_buffer = vec![0; size_to_be_read];
        let _ = self.read_exact(&mut read_buffer);
        let result = String::from_utf8(read_buffer);
        Ok(result.unwrap())
    }
}

impl ReadString for Vec<u8> {
    fn read_string(&mut self) -> io::Result<String> {
        let size_to_be_read = self.read_varint()? as usize;
        let mut read_buffer = vec![0; size_to_be_read];
        for i in 0..size_to_be_read {
            read_buffer[i] = self.read_byte()? as u8;
        }
        let result = String::from_utf8(read_buffer);
        Ok(result.unwrap())
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// VARLONG

impl WriteVarLong for TcpStream {
    fn write_varlong(&mut self, mut _value: i64) -> io::Result<()> {
        loop {
            if (_value & !SEGMENT_BITS as i64) == 0 {
                self.write_byte(_value as u8)?;
                break;
            }

            self.write_byte(((_value & 0x07) | CONTINUE_BIT as i64) as u8)?;

            _value >>= 7;
        }
        Ok(())
    }
}

impl WriteVarLong for Vec<u8> {
    fn write_varlong(&mut self, mut _value: i64) -> io::Result<()> {
        loop {
            if (_value & !SEGMENT_BITS as i64) == 0 {
                self.push(_value as u8);
                break;
            }

            self.push(((_value & 0x07) | CONTINUE_BIT as i64) as u8);

            _value >>= 7;
        }
        Ok(())
    }
}

impl ReadVarLong for TcpStream {
    fn read_varlong(&mut self) -> io::Result<i64> {
        let mut value: i64 = 0;
        let mut position: i64 = 0;
        let mut current_octet: u8;

        loop {
            current_octet = self.read_byte()? as u8;
            value |= (i32::from(SEGMENT_BITS as u8 & current_octet) << position) as i64;

            if (current_octet & CONTINUE_BIT as u8) == 0 {
                break;
            }

            position += 7;
        }
        if position >= 64 {
            println!("VarLong too big!");
        }

        Ok(value)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// BYTE

impl WriteByte for TcpStream {
    fn write_byte(&mut self, _value: u8) -> io::Result<()> {
        self.write_all(&_value.to_be_bytes())?;
        Ok(())
    }
}

impl WriteByte for Vec<u8> {
    fn write_byte(&mut self, _value: u8) -> io::Result<()> {
        self.extend_from_slice(&_value.to_be_bytes());
        Ok(())
    }
}

impl ReadByte for TcpStream {
    fn read_byte(&mut self) -> io::Result<i8> {
        let mut read_buffer = [0; 1];
        self.read(&mut read_buffer)?;
        Ok(read_buffer[0].to_le() as i8)
    }
}

impl ReadByte for Vec<u8> {
    fn read_byte(&mut self) -> io::Result<i8> {
        let mut result: i8 = 0;
        if self.len() > 0 {
            result = self.remove(0) as i8;
        }
        Ok(result)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// U16
///
impl WriteU16 for TcpStream {
    fn write_u16(&mut self, _value: u16) -> io::Result<()> {
        self.write_all(&_value.to_be_bytes())?;
        Ok(())
    }
}

impl WriteU16 for Vec<u8> {
    fn write_u16(&mut self, _value: u16) -> io::Result<()> {
        self.extend_from_slice(&_value.to_be_bytes());
        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// LONG

impl WriteLong for TcpStream {
    fn write_long(&mut self, _value: i64) -> io::Result<()> {
        self.write_all(&_value.to_be_bytes())?;
        Ok(())
    }
}

impl WriteLong for Vec<u8> {
    fn write_long(&mut self, _value: i64) -> io::Result<()> {
        self.extend_from_slice(&_value.to_be_bytes());
        Ok(())
    }
}

impl ReadLong for TcpStream {
    fn read_long(&mut self) -> io::Result<i64> {
        let mut read_buffer = [0; 8];
        self.read_exact(&mut read_buffer)?;

        Ok(i64::from_be_bytes(read_buffer))
    }
}

impl ReadLong for Vec<u8> {
    fn read_long(&mut self) -> io::Result<i64> {
        let mut read_buffer: [u8; 8] = [0; 8];

        for i in 0..8 {
            read_buffer[i] = self.read_byte()? as u8;
        }
        Ok(i64::from_be_bytes(read_buffer))
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////// OTHERS

impl ReadJSONString for TcpStream {
    fn read_json_string(&mut self, _length: usize) -> io::Result<String> {
        let mut read_buffer = vec![0; 8];
        let _ = self.read_exact(&mut read_buffer);
        let result = String::from_utf8(read_buffer);
        Ok(result.unwrap())
    }
}

impl WriteUUID for Vec<u8> {
    fn write_uuid(&mut self, _value: i128) -> io::Result<()> {
        self.extend_from_slice(&_value.to_be_bytes());
        Ok(())
    }
}

impl WriteUUID for TcpStream {
    fn write_uuid(&mut self, _value: i128) -> io::Result<()> {
        self.write_all(&_value.to_be_bytes())?;
        Ok(())
    }
}
