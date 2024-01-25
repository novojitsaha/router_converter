use std::net::Ipv4Addr;
fn main(){
    println!("compiled!");
}

pub struct Config {

    local : String,
    local_ip: (Ipv4Addr, Ipv4Addr), // (ip, subnet_mask)

    destination_1 : String, 
    destination_1_ip : (Ipv4Addr, Ipv4Addr),

    destination_2: String,
    destination_2_ip : (Ipv4Addr, Ipv4Addr),

    tc_ip : (Ipv4Addr, Ipv4Addr),
    sts_ip: (Ipv4Addr, Ipv4Addr),

    ospf : Vec<(Ipv4Addr, Ipv4Addr)>,
    routes : Vec<(Ipv4Addr, Ipv4Addr, Ipv4Addr)> // (destination_ip, destinatio_subnet_mask, gateway_ip)
}

// default implementation for Config
impl Default for Config{
    fn default() -> Config {

        let default_addr = Ipv4Addr::UNSPECIFIED;

        Config {

            local : String::from("default_local"),
            local_ip : (default_addr, default_addr),
        
            destination_1 : String::from("default_dest_1"), 
            destination_1_ip : (default_addr, default_addr),
        
            destination_2 : String::from("default_dest_2"),
            destination_2_ip : (default_addr, default_addr),
        
            tc_ip : (default_addr, default_addr),
            sts_ip : (default_addr, default_addr),
        
            ospf : Vec::new(),
            routes : Vec::new()
        }
    }
    

}

impl Config {
    fn set_local(&mut self, name:String, ip : (Ipv4Addr, Ipv4Addr)){
        self.local = name;
        self.local_ip = ip;
    }

    fn set_dest_1(&mut self, name:String, ip : (Ipv4Addr, Ipv4Addr)){
        self.destination_1 = name;
        self.destination_1_ip = ip;
    }

    fn set_dest_2(&mut self, name:String, ip : (Ipv4Addr, Ipv4Addr)){
        self.destination_2 = name;
        self.destination_2_ip = ip;
    }

    fn set_tc(&mut self,ip : (Ipv4Addr, Ipv4Addr)){
        self.tc_ip = ip;
    }
    
    fn set_sts(&mut self,ip : (Ipv4Addr, Ipv4Addr)){
        self.sts_ip = ip;
    }
    


    fn get_local(&self) -> (&String, (Ipv4Addr,Ipv4Addr)) {
        (&self.local,self.local_ip)
    }

    fn get_dest_1(&self) -> (&String, (Ipv4Addr,Ipv4Addr)){

        (&self.destination_1, self.destination_1_ip)
    }

    fn get_dest_2(&self) -> (&String, (Ipv4Addr,Ipv4Addr)){

        (&self.destination_2,self.destination_2_ip)
    }

    fn get_tc(&self) -> (Ipv4Addr,Ipv4Addr) {
        self.tc_ip
    }
    
    fn get_sts(&self) -> (Ipv4Addr,Ipv4Addr){
        self.sts_ip
    }

}


