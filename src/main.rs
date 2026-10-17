use x11rb::{connection::Connection, protocol::xproto::AtomEnum, wrapper::ConnectionExt};
use std::{env, time::{Duration, Instant}};
use crate::components::{ComponentRunner, DiskComponent, NetComponent, ProxyComponent, TimeComponent, VolComponent};
mod components;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    args.next().expect("program name");

    let components: &[ComponentRunner] = &[
        ComponentRunner::new(Box::new(ProxyComponent{
        }), 1),
        
        ComponentRunner::new(Box::new(VolComponent{
        }), 1),

        ComponentRunner::new(Box::new(NetComponent{
            interface: "wlp3s0"
        }), 5),
    
        ComponentRunner::new(Box::new(DiskComponent{
            mount: &["/", "/mnt/data"]
        }), 5),
        
        ComponentRunner::new(Box::new(TimeComponent{
        }), 1),
    ];
    
    if let Some(arg) = args.next() {
        if arg == "-s" {
            let mut res = String::from(" ");
            for (n, component) in components.iter().enumerate() {
                res.push_str(&component.update());
                if n != components.len()-1 {
                    res.push_str("| ");
                }
            }
            res.push_str(" ");
            println!("{res}");
            return Ok(())
        }
    }

    let (conn, screen_num) = x11rb::connect(None)?;
    let root = conn.setup().roots[screen_num].root;
    let begin_time = Instant::now();
    let mut status_vec: Vec<String> = vec![String::new(); components.len()];
    loop {
        let mut need_change = false;
        let t = begin_time.elapsed().as_secs();
        for (n, component) in components.iter().enumerate() {
            if t%(component.interval as u64) == 0 {
                let status = component.update();
                if status != status_vec[n] {
                    status_vec[n] = status;
                    need_change = true;
                }
            }
        }

        if need_change {
            let status = format!(" {} ", status_vec.join("| "));
            // println!("{status}");
            conn.change_property8(
                x11rb::protocol::xproto::PropMode::REPLACE, 
                root, 
                AtomEnum::WM_NAME, 
                AtomEnum::STRING, 
                status.as_bytes()
            )?;
            conn.flush()?;
        }

        std::thread::sleep(Duration::from_secs(1));
    }

}



