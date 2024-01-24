use std::net::Ipv4Addr;

struct Config {

    local : String,
    local_ip: (Ipv4Addr, String),

    destination_1 : String, 
    destination_1_ip : (Ipv4Addr, String),

    destination_2: String,
    destination_2_ip : (Ipv4Addr, String),

    tc_ip : (Ipv4Addr, String),

    sts_ip: (Ipv4Addr, String),


    ospf : Vec<(Ipv4Addr, String)>,
    
    routes : Vec<(Ipv4Addr, String, Ipv4Addr)>
}

impl Config {
    fn new(local: &str) -> Self {
        Config {}
    }
}

