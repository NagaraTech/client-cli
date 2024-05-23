use std::fs;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use std::path::Path;
use std::io::BufWriter;
use ed25519_dalek::{Keypair, Signer};
use ed25519_dalek::ed25519::signature::Signature;

const BIN_NAME_O: Option<&str> = option_env!("CARGO_BIN_NAME");
const BIN_NAME: &str = "ZClient";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keystore {
    pub name: String,

    #[serde(with = "serde_bytes")]
    pub keypair: [u8; 64],
}

impl Keystore {
    pub fn load(path: String) -> anyhow::Result<Keystore> {
        let path = Path::new(&path);
        let reader = File::open(path)?;
        Keystore::set_permissions(&reader)?;
        let keystore: Keystore = serde_json::from_reader(reader)?;
        Ok(keystore)
    }
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let path_parent = path.parent().unwrap();
        fs::create_dir_all(&path_parent)?;
        let writer = File::create(path)?;
        serde_json::to_writer_pretty(&writer, self)?;
        Keystore::set_permissions(&writer)?;
        Ok(())
    }

    pub fn sign(&self, data: &Vec<u8>) -> [u8; 64] {
        let keypair = Keypair::from_bytes(&self.keypair).unwrap();
        let signature = keypair.sign(data);
        signature.to_bytes()
    }

    pub fn public_key(&self) -> String{
        let keypair = Keypair::from_bytes(&self.keypair).unwrap();
        let public_key_bytes = keypair.public.to_bytes();
        let public_key_hex = hex::encode(public_key_bytes);
        public_key_hex
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


pub fn get_keypairs_list() {
    let directory_path = get_keypairs_path;

    let entries = fs::read_dir(directory_path())
        .expect("Failed to read directory");

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            println!("{}", file_name.to_string_lossy());
        }
    }
}

pub fn is_empty() ->bool{
    let directory_path = get_keypairs_path;
    if  directory_path().as_path().exists(){
        let entries = fs::read_dir(directory_path())
            .expect("Failed to read directory");
        for entry in entries {
            if let Ok(entry) = entry {
                return false
            }
        }
        return true
    }
    return true
}


pub fn get_file_path(file_name: &String) -> PathBuf {
    let dir = ProjectDirs::from_path(PathBuf::from(get_prog_without_ext())).unwrap();
    // fs::create_dir_all(dir.data_dir());
    let dp = dir.data_dir().join("keypairs").join(file_name.to_owned() + ".json");
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