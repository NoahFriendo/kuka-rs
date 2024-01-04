mod kuka;

use kuka::{AxisPosition, KukaMessage, KukaResponse};

use std::net::{SocketAddr, UdpSocket};

use quick_xml::de::from_str;

fn main() -> std::io::Result<()> {
    // Binding the server to a local address
    let socket = UdpSocket::bind("127.0.0.1:7878")?;
    println!("UDP server listening on 127.0.0.1:7878");

    loop {
        let mut buf = [0u8; 1024]; // Buffer for storing received data
        let (amt, src): (usize, SocketAddr) = socket.recv_from(&mut buf)?;

        let xml_data = std::str::from_utf8(&buf[..amt]).unwrap();
        println!("Received XML: {}", xml_data);

        match from_str::<KukaMessage>(xml_data) {
            Ok(rob) => {
                // println!("Deserialized: {:?}", rob);
                // other code
                let response = KukaResponse {
                    type_: "KROSHU".to_string(),
                    ak: AxisPosition {
                        a1: 0.0,
                        a2: 0.0,
                        a3: 0.0,
                        a4: 0.0,
                        a5: 0.0,
                        a6: 0.0,
                    },
                    stop: 0,
                    ipoc: rob.ipoc,
                };

                match quick_xml::se::to_string(&response) {
                    Ok(response_xml) => {
                        // println!("Response XML: {}", response_xml);
                        socket.send_to(response_xml.as_bytes(), &src)?;
                        println!("Sent {} bytes to {}", response_xml.len(), src);
                    }
                    Err(e) => {
                        eprintln!("Failed to serialize: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to deserialize: {}", e);
            }
        }
    }
}
