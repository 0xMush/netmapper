use tokio::net::TcpStream;
use std::env;
use std::time::Duration;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <ip> <start port>", args[0]);
        return;
    }


    port_scanner(&args).await;

}

async fn port_scanner(args: &[String]){
    // 2. Access the inputs by index
    let target_ip = &args[1].clone();
    // let target_port = &args[2];

    // check for ports range -p
            
    let mut ports = Vec::new();

    for (i, arg) in args.iter().enumerate() { {
        println!("{}", arg);
        if arg == "-p"{
            let mut j = i + 1;

            while j < args.len() {
                if args[j].starts_with("-"){
                    break;
                }
                
                ports.push(args[j].clone());
                j += 1;
            }

            //my code
        }
        }
    }

    if ports.len() == 1{
        let ports_arg = &ports[0];

        if ports_arg.contains("-"){
            // detected range
            let parts: Vec<&str> = ports_arg.split("-").collect();
            let start: u16 = parts[0].parse().expect("expected port number");
            let end: u16 = parts[1].parse().expect("expected port number");

            let mut handles = Vec::new();

            for check_port in start..=end {
                let addr = format!("{}:{}", target_ip, check_port);

                let handle = tokio::spawn(async move {
                    match TcpStream::connect(&addr).await {
                        Ok(mut stream) => {
                            let mut buffer = [0; 256];
                            let read_result = tokio::time::timeout(Duration::from_secs(2), stream.read(&mut buffer)).await;
                            
                            // Read results for banner and errors
                            match read_result{
                                Ok(Ok(n)) => {
                                    let banner = String::from_utf8_lossy(&buffer[0..n]);
                                    println!("Port is open {} {}", check_port, banner);
                                }
                                Ok(Err(e)) => {
                                    println!("Port is open {} Read error: {}", check_port, e);
                                }

                                Err(_) => {
                                    println!("Port is Open {}", check_port);
                                }
                            }

                            
                        }
                        Err(_) => println!("{} is CLOSED", check_port),
                    }

                });

                handles.push(handle);
            }
            for handle in handles {
                let _ = handle.await;  // ← Now await each one
            }

        } else {
            let addr = format!("{}:{}", target_ip, ports_arg);
            match TcpStream::connect(&addr).await{
                Ok(mut stream) => {
                    let mut buffer = [0; 256];
                    let read_result = tokio::time::timeout(Duration::from_secs(2), stream.read(&mut buffer)).await;

                    match read_result{
                        Ok(Ok(n)) => {
                            let banner = String::from_utf8_lossy(&buffer[0..n]);
                            println!("Port is Open {} {}", ports_arg, banner);
                        }
                        Ok(Err(e)) => {
                            println!("Port is Open {} Read Error: {}", ports_arg, e);
                        }
                        Err(_) => {
                            println!("Port is Open {}", ports_arg);
                        }
                    }





                } 
                Err(_) => println!("{} Is Closed", ports_arg)
            }
        }
    }
    else if ports.len() > 1{

        let mut handles = Vec::new();

        for ports_str in ports {
            let uport: u16 = ports_str.parse().expect("expecting port number");
            let addr = format!("{}:{}", target_ip, uport);
            
            let handle = tokio::spawn(async move {
                match TcpStream::connect(&addr).await{
                    Ok(mut stream) => {
                        let mut buffer = [0; 256];
                        let read_result = tokio::time::timeout(Duration::from_secs(2), stream.read(&mut buffer)).await;

                        match read_result {
                            Ok(Ok(n)) => {
                                let banner = String::from_utf8_lossy(&buffer[0..n]);
                                println!("Port is Open {} {}", uport, banner);
                            }
                            Ok(Err(e)) => {
                                println!("Porn is Open {} Read Error: {}", uport, e);
                            }
                            Err(_) => {
                                println!("Port is Open {}", uport);
                            }
                        } 
                    }
                    Err(_) => println!("{} Is Clossed", uport)
                }
                
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.await;
        }

    }


}

    