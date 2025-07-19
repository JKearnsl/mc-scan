// use std::net::UdpSocket;
// use std::time::{SystemTime, UNIX_EPOCH};
// use byteorder::{BigEndian, WriteBytesExt};
// use std::str;
//
// fn ping_bedrock_server(address: &str) -> Result<ServerInfo, Box<dyn std::error::Error>> {
//     let socket = UdpSocket::bind("0.0.0.0:0")?;
//     socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
//     socket.connect(address)?;
//
//     // Создаем пакет для запроса статуса
//     let mut packet = vec![0x01]; // Packet ID (Unconnected Ping)
//     packet.write_u64::<BigEndian>(
//         SystemTime::now()
//             .duration_since(UNIX_EPOCH)?
//             .as_millis() as u64,
//     )?; // Timestamp
//     packet.extend_from_slice(&[
//         0x00, 0xFF, 0xFF, 0x00, 0xFE, 0xFE, 0xFE, 0xFE, 0xFD, 0xFD, 0xFD, 0xFD, 0x12, 0x34, 0x56,
//         0x78,
//     ]); // Magic
//     packet.write_u64::<BigEndian>(0)?; // Client GUID
//
//     socket.send(&packet)?;
//
//     let mut buffer = [0u8; 4096];
//     let size = socket.recv(&mut buffer)?;
//
//     // Проверяем минимальный размер ответа
//     if size < 35 {
//         return Err("Invalid response length".into());
//     }
//
//     // Извлекаем текстовые данные (после 35 байт)
//     let text_data = &buffer[35..size];
//     let response_text = String::from_utf8_lossy(text_data);
//
//     // Парсим текстовый формат: MCPE;MOTD;Protocol;Version;Online;Max;ServerID;SubMotd;GameMode;GameModeID;Port;PortV6
//     let parts: Vec<&str> = response_text.split(';').collect();
//
//     if parts.len() < 10 {
//         return Err(format!("Invalid response format: only {} parts", parts.len()).into());
//     }
//
//     Ok(ServerInfo {
//         edition: parts[0].to_string(),
//         motd: parts[1].to_string(),
//         protocol: parts[2].parse().unwrap_or(0),
//         version: parts[3].to_string(),
//         online_players: parts[4].parse().unwrap_or(0),
//         max_players: parts[5].parse().unwrap_or(0),
//         server_id: parts[6].to_string(),
//         sub_motd: parts[7].to_string(),
//         gamemode: parts[8].to_string(),
//         gamemode_id: parts[9].parse().unwrap_or(0),
//         // port: parts[10].parse().unwrap_or(19132),
//     })
// }
//
// #[derive(Debug)]
// struct ServerInfo {
//     edition: String,
//     motd: String,
//     protocol: u32,
//     version: String,
//     online_players: u32,
//     max_players: u32,
//     server_id: String,
//     sub_motd: String,
//     gamemode: String,
//     gamemode_id: u32,
//     // port: u16,
// }
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let address = "play.craftersmc.net:19132";
//     let info = ping_bedrock_server(address)?;
//
//     println!("Тип: {}", info.edition);
//     println!("Название: {}", info.motd);
//     println!("Версия: {}", info.version);
//     println!("Протокол: {}", info.protocol);
//     println!("Игроки: {}/{}", info.online_players, info.max_players);
//     println!("ID сервера: {}", info.server_id);
//     println!("Доп. описание: {}", info.sub_motd);
//     println!("Режим игры: {}", info.gamemode);
//     println!("ID режима: {}", info.gamemode_id);
//     // println!("Порт: {}", info.port);
//
//     Ok(())
// }
//
// // fn clean_motd(motd: &str) -> String {
// //     motd.replace("§r", "")
// //         .replace("§b", "")
// //         .replace("§l", "")
// //         .replace("§8", "")
// //         .replace("§", "")
// //         .replace('\0', "")
// //         .trim().to_string()
// // }
