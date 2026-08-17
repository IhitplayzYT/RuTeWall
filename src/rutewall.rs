pub mod Rutewall{
    use std::fs;

    use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

   pub type Mode = u8; 

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


    pub fn check_command(command: &str,allow_map:&HashSet<String>,deny_map:&HashSet<String>,fs: &HashMap<String,u8>,network:bool) -> Result<(), String> {
        let tokens = tokenize(command)?;
        if tokens.is_empty() {
            return Err("Empty command".into());
        }

        for token in &tokens {
            if is_shell_operator(token) {
                return Err(format!(
                    "shell operator not allowed: {token}"
                ));
            }
        }
        let executable = &tokens[0];

        if has_prefix_rule(&deny_map,executable) {
            return Err(format!("Command denied: {executable}"));
        }

        if !has_prefix_rule(&allow_map, executable) {
            return Err(format!("Command not allowed: {executable}"));
        }

        if !network && uses_network(executable, &tokens[1..]) {
            return Err(format!("Network access disabled: {executable}"));
        }

        for token in &tokens[1..] {
            if looks_like_path(token) {
                check_path(token,&fs)?;
            }
        }

        Ok(())
    }

    fn check_path(path: &str,fs_map: &HashMap<String,u8>) -> Result<(), String> {
        let pth = Path::new(path);
        let path = normalize_path(&pth);
        if !fs_map.keys().any(|workspace| path.starts_with(workspace)){
            return Err(format!("Filesystem access denied: {}",path.display()));
        }
        Ok(())
    }

    fn path_is_under(path: &Path,ws: &Path) -> bool {
        path == ws || path.starts_with(ws)
    }

    fn looks_like_path(s: &str) -> bool {
        let p = Path::new(s);
        p.is_absolute() || s.starts_with("./") || s.starts_with("../") || s == "." || s == ".." || s.contains('/') || s.contains('\\')
    }


    fn normalize_path<P: AsRef<Path>>(path: &P) -> PathBuf {
        let path = path.as_ref();
        let mut result = PathBuf::new();
        for component in path.components() {
            match component {
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {result.pop();}
                component => {
                    result.push(component.as_os_str());
                }
            }
        }
        result
    }

    fn is_shell_operator(token: &str) -> bool {
        matches!(token,"|" | "||" | "&" | "&&" | ";" | "2>>")
    }

    fn tokenize(txt: &str) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut quote: Option<char> = None;

        for c in txt.chars() {
            match quote {
                Some(q) => {
                    if c == q {
                        quote = None;
                    } else {
                        current.push(c);
                    }
                }
                None => match c {
                    '\'' | '"' => {quote = Some(c);}
                    ' ' | '\t' | '\n' => {
                        if !current.is_empty() {
                            result.push(current.clone());
                        }
                    }

                    _ => current.push(c),
                },
            }
        }

        if quote.is_some() {
            return Err("Unterminated quote".into());
        }
        if !current.is_empty() {
            result.push(current);
        }

        Ok(result)
    }


    fn has_prefix_rule(rules: &HashSet<String>,command: &str) -> bool {
        rules.iter().any(|prefix| {
        command == prefix || command.starts_with(&format!("{prefix} "))
        })
    }

    fn uses_network(command: &str, args: &[String]) -> bool {
        matches!(command,"curl" | "wget" | "ssh" | "scp" | "sftp" | "nc" | "ncat" | "telnet" | "ftp" | "git")
    }

}