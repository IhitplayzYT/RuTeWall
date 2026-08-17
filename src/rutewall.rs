pub mod Rutewall{
    use std::{fs::{self, File}, usize};


   type Mode = u8; 

   #[derive(Debug,Clone)]
    pub struct FileSystem{
        pub mode: Mode,
        pub path: String
    }

    fn parse_perms(perms: &str) -> Mode{
        let mut ret = 0;
        if perms == "read"{
            ret |= 1 << 2;
        }else if perms == "write"{
            ret |= 1 << 1;
        }else if perms == "execute"{
            ret |= 1 << 0;
        }else{
            if perms.contains("r"){
                ret |= 1 << 2;
            }else if perms.contains("w"){
                ret |= 1 << 1;
            }else if perms.contains("x"){
                ret |= 1 << 0;
            }else{}
        }
        ret    
    }

    impl Default for FileSystem{
        fn default() -> Self {
            Self { mode: 0, path: "".to_string() }
        }
    }
    


    #[derive(Debug,Clone)]
    pub struct Rules{
        pub allows: Vec<String>,
        pub deny: Vec<String>,
        pub fs: Vec<FileSystem>,
        pub network: bool
    }

    impl Rules{
        pub fn new() -> Self{
            Self { allows: vec![], deny: vec![], fs: vec![], network: false }
        }

        pub fn from_path(path: &str) -> Self{
            let conf = fs::read_to_string(path).unwrap();
            let lines = conf.split("\n\n").map(|x| x.trim().to_string()).collect::<Vec<String>>();
            let mut ret = Self::new();
            for i in &lines{
                if i.starts_with("allow"){
                    if let Some(strt) = i.find("["){
                        if let Some(end) = i.find("]"){
                            assert!(strt <= end);
                            ret.allows.append(&mut i[strt+1..end].split(",").map(|x| x.trim().to_string()).collect::<Vec<String>>());
                        }
                    }
                } else if i.starts_with("deny"){
                    if let Some(strt) = i.find("["){
                        if let Some(end) = i.find("]"){
                            assert!(strt <= end);
                            ret.deny.append(&mut i[strt+1..end].split(",").map(|x| x.trim().to_string()).collect::<Vec<String>>());
                        }
                    }
                } else if i.starts_with("[filesystem]") {
                    for perm_lists in i.split("\n"){
                        let mut mode = 0;
                        let mut paths = vec![];
                        let mut fs = FileSystem::default();
                        for z in perm_lists.split("=").map(|x| x.trim()){
                            if z.starts_with("["){
                                if let Some(ed) = z.find("]"){
                                    paths.append(&mut i[1..ed].split(",").map(|x| x.trim()).collect());
                                }
                            }else{
                                mode = parse_perms(z);
                            }
                            ret.fs.append(&mut paths.iter().map(|x| FileSystem{mode,path:x.to_string()}).collect());
                        }
                    }
                } else if i.starts_with("[network]"){
                    if let Some(x) = i.find("="){
                        ret.network = i[x+1..].to_lowercase().parse::<bool>().expect("Allow field in Network section can be only True/False");
                    }
                }else{}
            }
            ret
        }


    }



}