pub mod cli {
    pub mod args;
    pub mod command;
}

pub mod commands {
    pub mod init;
    pub mod add;
    pub mod rm;
    pub mod commit;
    pub mod branch;
    pub mod checkout;
    pub mod merge;
    pub mod fetch;
    pub mod pull;
    pub mod push;
}

pub mod core{
    pub mod blob;
    pub mod commit;
    pub mod object;
    pub mod tree;
    pub mod repository;
    pub mod index;
    pub mod reference;
}

pub mod utils {
    pub mod hash;
    pub mod fs;
    pub mod error;
}