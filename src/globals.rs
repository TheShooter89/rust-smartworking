pub const APP_NAME: &str = "rust-smartworking";
pub const APP_VERSION: &str = "0.1.0-prealpha";

// to server only to this computer locally
//pub const HOST: &str = "127.0.0.1";
//pub const HOST: &str = "localhost";

// to access the webapp from the same network
// replace '192.168.57' with actual address of server pc over the network'
// from terminal 'ip addr show'
pub const HOST: &str = "192.168.1.57";
pub const PORT: u16 = 3000;

pub const APP_DATA_ROOT: &str = "_DATA/";

pub const DATABASE_FOLDER: &str = "_DATA/database/";
pub const DATABASE_NAME: &str = "rsw.db";
pub const DATABASE_PATH: &str = "_DATA/database/rsw.db";
