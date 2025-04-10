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