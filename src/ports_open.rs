use std::net::{SocketAddr, TcpStream};

    // return vector of ports that are open
pub fn get_open_ports(start:u16, end:u16) -> Vec<u16> {
    let mut open_ports = Vec::new();
    for i in start..=end {
        // create an io stream
        let result = TcpStream::connect(("127.0.0.1", i));
        if result.is_ok(){
            open_ports.push(i);
        }
        // else {
        //     println!("Couldn't connect to port '{}'", i);
        // }
        
    }
    open_ports
}