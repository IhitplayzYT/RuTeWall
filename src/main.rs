use std::{ error::Error, fs};

use crate::helper::Helper::CLI;

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

    let conf = fs::read_to_string(&clargs.path.unwrap())?;
    let rules = parse_conf(&conf);



Ok(())
}
