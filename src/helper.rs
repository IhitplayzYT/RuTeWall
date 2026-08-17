pub mod Helper{
    use std::process::exit;



    const DBG_STR: &str = "";
    const OK:i32 = 0;
    const ERR:i32 = -1;


    #[derive(Debug,Clone)]
    pub struct CLI{
        pub dbg: bool,
        pub path : Option<String>,
        pub fifo:Option<String>,
    }

    pub fn Help(){
        println!("{DBG_STR}");
        exit(OK);
    }


    impl CLI{
        pub fn new() -> Self{
            Self { dbg: false, path: None, fifo: None }
        }

        pub fn Parse_Args(&mut self){
            let args: Vec<String> = std::env::args().skip(1).collect();
            for i in args.iter().skip(1){
                if i == "-d" || i == "--debug" || i == " --DEBUG" || i == "-D"{
                    self.dbg = true;
                } else if i == "-h" || i == "--help" || i == " --HELP" || i == "-H"{
                    Help();
                } else if i.starts_with("--path=") || i.starts_with("--conf"){  
                    self.path = Some(i[i.find("=").unwrap()+1..].to_string());
                } else if i.starts_with("--fifo") || i.starts_with("--pipe"){
                    self.fifo = Some(i[i.find("=").unwrap()+1..].to_string());
                }
                else{
                    Help();
                }
            } 


        }



    }


    





}