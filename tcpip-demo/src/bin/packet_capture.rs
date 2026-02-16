use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::Packet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::{TcpFlags, TcpPacket};

fn main() {
    let interface = datalink::interfaces();

    // 適切なインターフェースを選択(en0, eth0)
    let interface = interface
        .into_iter()
        .find(|iface| iface.name == "en0" || iface.name == "eth0")
        .expect("not found network interface");

    println!("Capture Start: {}", interface.name);
    println!("Monitoring TCP packets...\n");

    // open datalink Channel
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Failed to create Channel: {}", e),
    };

    // Receive a packet
    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet = EthernetPacket::new(packet).unwrap();

                if ethernet.get_ethertype() == EtherTypes::Ipv4 {
                    if let Some(ipv4) = Ipv4Packet::new(ethernet.payload()) {
                        // Only TCP
                        if ipv4.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                            if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                                print_tcp_packet(&ipv4, &tcp);
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn print_tcp_packet(ipv4: &Ipv4Packet, tcp: &TcpPacket) {
    println!("═══════════════════════════════════════════════");
    println!("TCP Packet");
    println!("───────────────────────────────────────────────");

    // IP Layer
    println!(" IP Layer:");
    println!("  送信元IP: {}", ipv4.get_source());
    println!("  宛先IP: {}", ipv4.get_destination());
    println!("  パケット長: {} バイト", ipv4.get_total_length());

    // TCP Layer
    println!("\nTCP Layer:");
    println!("  送信元ポート: {}", tcp.get_source());
    println!("  宛先ポート: {}", tcp.get_destination());
    println!("  シーケンス番号: {}", tcp.get_sequence());
    println!("  ACK番号: {}", tcp.get_acknowledgement());
    println!("  ヘッダ長: {} バイト", tcp.get_data_offset() * 4);
    println!("  ウィンドウサイズ: {} バイト", tcp.get_window());

    // フラグの表示
    println!("\n Flag:");
    let flags_value = tcp.get_flags();
    let flags = [
        ("SYN", TcpFlags::SYN),
        ("ACK", TcpFlags::ACK),
        ("FIN", TcpFlags::FIN),
        ("RST", TcpFlags::RST),
        ("PSH", TcpFlags::PSH),
        ("URG", TcpFlags::URG),
    ];

    let active_flags: Vec<&str> = flags
        .iter()
        .filter(|(_, mask)| flags_value & *mask != 0)
        .map(|(name, _)| *name)
        .collect();

    if active_flags.is_empty() {
        println!("  (Non Flag)");
    } else {
        println!("  {}", active_flags.join(", "));
    }

    // ペイロードサイズ
    let payload_size = tcp.payload().len();
    if payload_size > 0 {
        println!("\n Payload: {} Bytes", payload_size);

        // データの一部を表示（最初の64バイト）
        let preview_len = payload_size.min(64);
        let preview = &tcp.payload()[..preview_len];

        // ASCIIとして表示可能な部分を抽出
        let text: String = preview
            .iter()
            .map(|&b| {
                if b.is_ascii_graphic() || b == b' ' {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();

        println!("  Preview: {}", text);
    }

    println!("═══════════════════════════════════════════════\n");
}
