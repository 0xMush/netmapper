use tokio::net::TcpStream;
use std::env;




#[tokio::main]
async fn main() {


   

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <ip> <start port>", args[0]);
        return;
    }

    for arg in &args {
        println!("{}", arg);
        }

    // 2. Access the inputs by index
    let target_ip = &args[1];
    let target_port = &args[2];

    // check for ports range -p
    
    if args[2] == "-p"{
        let range = &args[3];
        let parts: Vec<&str> = range.split("-").collect();
        let start: u16 = parts[0].parse().expect("expecting a number");
        let end: u16 = parts[1].parse().expect("expecting a number");
        
        let mut handles = Vec::new();

        for check_port in start..=end {
            let addr = format!("{}:{}", target_ip, check_port);

            let handle = tokio::spawn(async move {
                match TcpStream::connect(&addr).await {
                    Ok(_) => println!("{} is OPEN", check_port),
                    Err(_) => println!("{} is CLOSED", check_port),
                }
            });

            handles.push(handle); // push the handle into the vector
        }
        // Step 3: Wait for all tasks AFTER the loop
        for handle in handles {
            let _ = handle.await;
        }
            
     }else{
            let addr = format!("{}:{}", target_ip, target_port);
            match TcpStream::connect(&addr).await {
                    Ok(_) => println!("{} is OPEN", target_port),
                    Err(_) => println!("{} is CLOSED", target_port),
                    }


    }
}

async fn port_scanner() {



    
}