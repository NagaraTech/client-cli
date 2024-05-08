use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

const BIN_NAME_O: Option<&str> = option_env!("CARGO_BIN_NAME");
const BIN_NAME: &str = "ZClient";
const CONFIG_FILE_DEFAULT: &str = "config.json";


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub KeypairPath: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Config> {
        let path = get_config_path();
        let reader = File::open(path).expect("Can not find config.json, please create new keypair");
        Config::set_permissions(&reader)?;
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let path =get_config_path();
        let path_parent = path.parent().unwrap();
        fs::create_dir_all(&path_parent)?;
        let writer = File::create(path)?;
        serde_json::to_writer_pretty(&writer, self)?;
        Config::set_permissions(&writer)?;
        Ok(())
    }
    #[cfg(unix)]
    fn set_permissions(file: &File) -> anyhow::Result<()> {
        use std::os::unix::fs::PermissionsExt;
        let perms = file.metadata()?.permissions();
        // is the file world-readable? if so, reset the permissions to 600
        if perms.mode() & 0o4 == 0o4 {
            file.set_permissions(fs::Permissions::from_mode(0o600))
                .unwrap();
        }
        Ok(())
    }
}

pub fn get_config_path() -> PathBuf {
    let dir = ProjectDirs::from_path(PathBuf::from(get_prog_without_ext())).unwrap();
    // fs::create_dir_all(dir.data_dir());
    let dp = dir.data_dir().join("config").join(CONFIG_FILE_DEFAULT);
    dp
}

pub fn get_keypairs_path() -> PathBuf {
    let dir = ProjectDirs::from_path(PathBuf::from(get_prog_without_ext())).unwrap();
    // fs::create_dir_all(dir.data_dir());
    let dp = dir.data_dir().join("keypairs");
    dp
}

pub fn get_prog_without_ext() -> &'static str {
    get_bin_name() // with -rs suffix
    // get_pkg_name() // without -rs suffix
}

pub fn get_bin_name() -> &'static str {
    BIN_NAME_O.unwrap_or(BIN_NAME)
}