pub mod Rutewall{

   type Mode = u8; 

   #[derive(Debug,Clone)]
    pub struct FileSystem{
        pub mode: Mode,
        pub path: String
    }


    #[derive(Debug,Clone)]
    pub struct Rules{
        pub allows: Vec<String>,
        pub deny: Vec<String>,
        pub fs: Vec<FileSystem>,
        pub network: bool
    }

    impl Rules



}