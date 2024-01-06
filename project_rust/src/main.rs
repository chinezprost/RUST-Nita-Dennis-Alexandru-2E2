use crossterm::{cursor, terminal, ExecutableCommand};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde_json::Value;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread::{self};
use std::{result, vec};


use colored::*;


const SEGMENT_BITS: i32 = 0b0111_1111;
const CONTINUE_BIT: i32 = 0b1000_0000;

const IPV4_ADRESS: &str = "localhost";
const PORT: i16 = 25565;

const COMPRESSION_THRESHOLD: i32 = 128; //default threshold
#[derive(Clone)]
struct CurrentUserList {
    online_players_count: i32,
    online_players_list: Vec<(i128, String)>,
}

struct Printed {
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    color: String,
}

type MainResult<T> = result::Result<T, Box<dyn std::error::Error>>;

fn main() -> MainResult<()> {
    let mut stream: TcpStream;
    println!("Trying to connect to {}:{} ...", IPV4_ADRESS, PORT);
    if let Ok(_stream) = TcpStream::connect(format!("{}:{}", IPV4_ADRESS, PORT)) {
        println!(
            "{}",
            "Connected succesfully to the specified server!".green()
        );
        stream = _stream;
    } else {
        println!("{}", "Couldn't connect to server! Exiting...".red());
        std::process::exit(0);
    }

    // handshake_serverbound("127.0.01", 25565, 2)?;

    let next_state;

    let current_player_list = Arc::new(Mutex::new(CurrentUserList {
        online_players_count: -1,
        online_players_list: Vec::new(),
    }));

    let current_player_list_clone = Arc::clone(&current_player_list);
    let current_player_list_clone2 = Arc::clone(&current_player_list);

    println!(
        "{}{}{}",
        "Welcome to MClient, an Interface Client for Minecraft servers (".bright_yellow(),
        "server version 1.18.0".red(),
        ").".bright_yellow()
    );
    println!(
        "Enter [{}] for status check or [{}] for connecting as a player to the server.",
        "stat".bright_cyan(),
        "login username".bright_cyan()
    );

    let mut _processed_username = String::new();
    loop {
        let mut _input = String::new();
        io::stdin().read_line(&mut _input)?;

        let _input_word_count = _input.split_ascii_whitespace().count();

        match _input_word_count {
            1 => {
                if _input.trim().to_lowercase() == "stat" {
                    next_state = 1;
                    break;
                }
            }
            2 => {
                let _split_input = _input.split_once(' ').expect("Couldn't split user input.");
                if _split_input.0.trim() == "login" {
                    if _split_input.1.trim().len() > 16 {
                        println!(
                            "{}",
                            "Username should not be longer than 16 characters.".red()
                        );
                    } else {
                        _processed_username = _split_input.1.trim().to_string();
                        next_state = 2;
                        break;
                    }
                }
            }
            _ => (),
        }
        println!("{}", "Invalid input. Try again...".red());
    }

    if next_state == 1 {
        let handshake_packet_state_1 = handshake_serverbound("127.0.0.1", 25565, 1)?;

        stream.write_varint(handshake_packet_state_1.len() as i32)?;
        stream.write_all(&handshake_packet_state_1)?;

        stream.write_byte(0x01)?; // REQUEST STATUS
        stream.write_byte(0x00)?;

        let _ = stream.read_varint()?;
        let _ = stream.read_varint()?;
        let read_json = stream.read_string()?;

        stream.write_byte(0x09)?;
        stream.write_byte(0x01)?;
        stream.write_long(111)?;

        let _ = stream.read_varint()?;
        let _ = stream.read_byte()?;
        let _ = stream.read_long()?;
        let parsed_response: Value = serde_json::from_str(&read_json)?;

        let description_text = parsed_response["description"]["text"].as_str().unwrap();
        let players_max = parsed_response["players"]["max"].as_i64().unwrap();
        let players_online = parsed_response["players"]["online"].as_i64().unwrap();
        let version_name = parsed_response["version"]["name"].as_str().unwrap();
        let protocol_id = parsed_response["version"]["protocol"].as_i64().unwrap();

        println!(
            "{}",
            "==================== STATUS ===================="
                .bold()
                .blue()
        );
        println!("{} {}", "Server Description:".yellow(), description_text);
        println!(
            "{} [{}/{}]",
            "Online Players:".yellow(),
            players_online,
            players_max
        );
        println!("{} {}", "Minecraft Version:".yellow(), version_name);
        println!("{} {}", "Server Protocol:".yellow(), protocol_id);
        println!(
            "{}",
            "================================================"
                .bold()
                .blue()
        );

        std::process::exit(0);
    } else {
        let handshake_packet_state_2 = handshake_serverbound("127.0.0.1", 25564, 2)?;

        stream.write_varint(handshake_packet_state_2.len() as i32)?;
        stream.write_all(&handshake_packet_state_2)?;

        let login_start = login_start_serverbound(&_processed_username)?;
        stream.write_varint(login_start.len() as i32)?;
        stream.write_all(&login_start)?;

        //set-compression packet
        let _ = stream.read_varint()?;
        let _ = stream.read_byte()?;
        let _ = stream.read_varint()?;
        //println!("[SERVER]: Packet Threshold set to {}!", packet_threshold);

        // FROM NOW ON PACKETS ARE COMPRESSED
        // FROM NOW ON PACKETS ARE COMPRESSED
        // FROM NOW ON PACKETS ARE COMPRESSED
        let stream_cloned = stream.try_clone()?;
        let mut send_packet_stream = stream.try_clone()?;

        let _ = thread::spawn(move || -> Result<(), std::io::Error> {
            let mut has_read_login_succes = false;
            loop {
                let mut received_packet = decode_packet(stream_cloned.try_clone()?)?;

                if received_packet.is_empty() {
                    continue;
                }
                let _ = received_packet.read_varint()?;
                let packet_id = received_packet.read_varint()?;

                match packet_id {
                    0x02 => {
                        if has_read_login_succes {
                            continue;
                        }
                        println!(
                            "{} {}",
                            "You've joined the game as user:".bright_green(),
                            _processed_username.white()
                        );
                        println!(
                            "Type {} for chat, {} to see online players.",
                            "[s <text>]".cyan(),
                            "[players]".cyan()
                        );
                        has_read_login_succes = true;
                    }
                    0x0F => {
                        let received_message = received_packet.read_string()?;
                        println!("{}", received_message);
                        let _ = received_packet.read_byte()?;
                        let _ = received_packet.read_uuid()?;

                        let json_chat: Value = serde_json::from_str(&received_message)
                            .expect("Couldn't deseralize JSON.");

                        if json_chat["translate"] == "chat.type.text" {
                            if let Some(with_partition) = json_chat["with"].as_array() {
                                if let Some(username_partition) =
                                    with_partition[0]["insertion"].as_str()
                                {
                                    print!("<{}> ", username_partition);
                                }
                                if let Some(user_text_partition) = with_partition[1].as_str() {
                                    println!("{}", user_text_partition);
                                }
                            }
                        }
                        else if json_chat["translate"] == "chat.type.announcement"
                        {
                            println!("announcement failed");
                        }
                        else if json_chat["translate"] == "commands.message.display.incoming"
                        {
                            if let Some(with_partition) = json_chat["with"].as_array() {
                                if let Some(username_partition) =
                                    with_partition[0]["insertion"].as_str()
                                {
                                    print!("{} {}", username_partition.custom_color(CustomColor { r: (128), g: (128), b: (128) }), "whispers to you: ".custom_color(CustomColor { r: (128), g: (128), b: (128) }));
                                }
                                if let Some(user_text_partition) = with_partition[1]["text"].as_str() {
                                    println!(" {}", user_text_partition.custom_color(CustomColor { r: (128), g: (128), b: (128) }));
                                }
                            }
                        }
                        else if json_chat["extra"] == "command.unknown.command"
                        {
                            println!("{}", "Unknown command!".red());
                        }
                        else 
                        {
                            if let Some(properties) = json_chat["extra"].as_array()
                            {
                                for property in properties
                                {
                                    let mut message = ColoredString::from(property["text"].as_str().unwrap());
                                    if let Some(_) = property["bold"].as_bool()
                                    {
                                        message = message.bold();
                                    }

                                    if let Some(_) = property["strikethrough"].as_bool()
                                    {
                                        message = message.strikethrough();
                                    }

                                    if let Some(_) = property["italic"].as_bool()
                                    {
                                        message = message.italic();

                                    }

                                    if let Some(_) = property["underlined"].as_bool()
                                    {
                                        message = message.underline();
                                    }

                                    if let Some(value) = property["color"].as_str()
                                    {
                                        match value
                                        {
                                            "black" => { message = message.custom_color(CustomColor { r: (0), g: (0), b: (0) })}
                                            "dark_blue" => { message = message.custom_color(CustomColor { r: (0), g: (0), b: (170) })}
                                            "dark_green" => { message = message.custom_color(CustomColor { r: (0), g: (170), b: (0) })}
                                            "dark_aqua" => { message = message.custom_color(CustomColor { r: (0), g: (170), b: (170) })}
                                            "dark_red" => { message = message.custom_color(CustomColor { r: (170), g: (0), b: (0) })}
                                            "dark_purple" => { message = message.custom_color(CustomColor { r: (170), g: (0), b: (170) })}
                                            "gold" => { message = message.custom_color(CustomColor { r: (255), g: (170), b: (0) })}
                                            "gray" => { message = message.custom_color(CustomColor { r: (170), g: (170), b: (170) })}
                                            "dark_gray" => { message = message.custom_color(CustomColor { r: (85), g: (85), b: (85) })}
                                            "blue" => { message = message.custom_color(CustomColor { r: (85), g: (85), b: (255) })}
                                            "green" => { message = message.custom_color(CustomColor { r: (85), g: (255), b: (85) })}
                                            "aqua" => { message = message.custom_color(CustomColor { r: (85), g: (255), b: (255) })}
                                            "red" => { message = message.custom_color(CustomColor { r: (255), g: (85), b: (85) })}
                                            "light_purple" => { message = message.custom_color(CustomColor { r: (255), g: (85), b: (255) })}
                                            "yellow" => { message = message.custom_color(CustomColor { r: (255), g: (255), b: (85) })}
                                            "white" => { message = message.custom_color(CustomColor { r: (255), g: (255), b: (255) })}
                                            _ => ()
                                        }
                                    }

                                    print!("{}", message);
                                }
                            }
                            println!("");

                            
                        }

                        
                    }
                    0x21 => {
                        let received_keep_alive_long = received_packet.read_long()?;

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

                        for _ in 0..number_of_players {
                            let player_uuid = received_packet.read_uuid()?;
                            match pack_action {
                                0 => {
                                    let username = received_packet.read_string()?;
                                    //ignore
                                    let no_of_properties = received_packet.read_varint()?;
                                    for _ in 0..no_of_properties {
                                        received_packet.read_string()?;
                                        received_packet.read_string()?;
                                        let is_signed = received_packet.read_byte().unwrap();
                                        if is_signed == 0x01 {
                                            received_packet.read_string()?;
                                        }
                                    }
                                    received_packet.read_varint()?;
                                    received_packet.read_varint()?;
                                    received_packet.read_byte()?;
                                    received_packet.read_string()?;
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
                                    received_packet.read_varint()?;
                                }
                                2 => {
                                    received_packet.read_varint()?;
                                }
                                3 => {
                                    received_packet.read_byte()?;
                                    received_packet.read_string()?;
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
                        //let received_disconnect_message = received_packet.read_string()?;
                        println!("{}", "You have been disconnected from the server!".red());
                        std::process::exit(0x00);
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
    let client_logic = thread::spawn(move || -> Result<(), std::io::Error> {
        loop {
            let mut _input_command = String::new();
            io::stdin()
                .read_line(&mut _input_command)
                .expect("Couldn't read from console.");

            io::stdout().execute(cursor::MoveUp(1))?;
            io::stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine))?;

            let mut _input_command_split = _input_command.split_once(' ');

            let mut _input_command_split = _input_command.split_once(' ');

            let _command_count = _input_command.split(' ').count();

            match _command_count {
                1 => {
                    let command_type = _input_command.trim();
                    match command_type {
                        "players" => {
                            let current_player_list = current_player_list_clone2.lock().unwrap();
                            println!(
                                "{}",
                                "============================================".bold().cyan()
                            );
                            println!(
                                "{} {} {}",
                                "======= There are".cyan(),
                                current_player_list.clone().online_players_count,
                                "players online! ========".cyan()
                            );
                            let mut count = 1;
                            for i in &current_player_list.clone().online_players_list {
                                println!("{}. {}", count, i.1);
                                count += 1;
                            }
                            println!(
                                "{}",
                                "============================================".bold().cyan()
                            );
                        }
                        _ => (),
                    }
                }
                _ => {
                    // if let x = _input_command.split(' ') {
                    //     match x.next() {
                    //         "s" => {
                    //             let mut chat_message_string: Vec<u8> = Vec::new();
                    //             chat_message_string
                    //                 .write_string(&x.collect::<Vec<&str>>().join(" "))
                    //                 .expect("Couldn't write string");
                    //             let chat_message = encode_packet(0x03, &chat_message_string)
                    //                 .expect("Couldn't encode chat message");
                    //             stream.write_all(&chat_message).expect("Couldn't write.");
                    //         }
                    //         _ => (),
                    //     }
                    // }
                }

                _ => {}
            }
        }
    });
    client_logic.join().unwrap()?;
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
                return Err(err);
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
    Ok(final_packet)
}

fn encode_packet(packet_id: i32, data: &[u8]) -> io::Result<Vec<u8>> {
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
        //println!("Sending uncompressed packet with ID: {}", packet_id);
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
    if _username.len() > 16 || _username.is_empty() {
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
            varint_length += 1;
            value >>= 7;
            if value == 0 {
                break;
            }
        }
        varint_length
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

        for i in &mut read_buffer {
            *i = self.read_byte()? as u8;
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
            value |= i32::from(SEGMENT_BITS as u8 & current_octet) << position;

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
            value |= i32::from(SEGMENT_BITS as u8 & current_octet) << position;

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
        for i in read_buffer.iter_mut().take(size_to_be_read) {
            *i = self.read_byte()? as u8;
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
        self.read_exact(&mut read_buffer)?;
        Ok(read_buffer[0].to_le() as i8)
    }
}

impl ReadByte for Vec<u8> {
    fn read_byte(&mut self) -> io::Result<i8> {
        let mut result: i8 = 0;
        if !self.is_empty() {
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

        for i in &mut read_buffer {
            *i = self.read_byte()? as u8;
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
