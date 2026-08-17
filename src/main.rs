use std::{ collections::HashSet, error::Error, fs};

use crate::{helper::Helper::CLI, rutewall::Rutewall::Rules};

use std::{fs::OpenOptions,io::{self, Read},os::unix::fs::OpenOptionsExt};


mod helper;
mod rutewall;

fn main() -> Result<(), Box<dyn Error>>{
    let mut clargs = CLI::new();
    clargs.Parse_Args();

    if clargs.dbg{
        println!("{clargs:?}");
    }

    if clargs.path.is_none(){
        panic!("Conf path is required for RuTeWall config");
    }

    let rules = Rules::from_path(&clargs.path.unwrap());
    let allow_map:HashSet<String> = rules.allows.into_iter().collect();
    let deny_map:HashSet<String> = rules.deny.into_iter().collect();
    let allow_map:HashSet<String> = rules.fs.into_iter().map(|x| x.path).collect();


    let fifo_path = clargs.fifo.unwrap_or("/tmp/fifo".to_string());

    let c_path = std::ffi::CString::new(fifo_path.clone()).unwrap();
    let ret = unsafe { libc::mkfifo(c_path.as_ptr(), 0o644)};
    if ret == -1 {
        let err = io::Error::last_os_error();
        if err.kind() != io::ErrorKind::AlreadyExists {
            return Err(Box::new(err));
        }
    }
    let mut fifo = OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK).open(&fifo_path)?;

    let mut buf = [0u8; 4096];
    loop {
        match fifo.read(&mut buf) {
            Ok(0) => {std::thread::sleep(std::time::Duration::from_millis(10));}
            Ok(n) => {
                // TODO:
                let tokens = String::from_utf8(buf[..n].to_vec()).unwrap().split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
                for i in &tokens{


                }



                



            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {std::thread::sleep(std::time::Duration::from_millis(10));}
            Err(e) => return Err(Box::new(e)),
        }
    }
    






Ok(())
}
