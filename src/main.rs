// -------------- CROSS-PLATFORM STUFF -------------- 

use std::{arch, env::{self, Args}, fmt::Display, fs::{self, remove_dir_all, remove_file, DirBuilder, File}, io::{self, Write}, path::{Path, PathBuf}};
use futures::StreamExt;


#[cfg(any(
    not(any(target_os = "windows", target_os = "linux")),
    not(any(target_arch = "x86", target_arch = "x86_64"))
))]
compile_error!("The Goldberg emulator only suppoorts Windows and Linux x86/x86_64");

#[cfg(target_os = "windows")]
mod consts {
    #[cfg(target_arch = "x86")] 
    pub const STEAM_LIBS: [&str; 2] = ["steam_api.dll", "steamclient.dll"];
    #[cfg(target_arch = "x86_64")]
    pub const STEAM_LIBS: [&str; 2] = ["steam_api64.dll", "steamclient64.dll"];
    
    pub const GBE_FILE: &str = "emu-win-release.7z";
    
    pub const USAGE_EXAMPLE: &str = "goldberg-patcher \"D:\\Games\\My Game\"";
}

#[cfg(target_os = "linux")]
mod consts {
    pub const STEAM_LIBS: [&str; 2] = ["libsteam_api.so", "steamclient.so"];
    pub const GBE_FILE: &str = "emu-linux-release.tar.bz2";
    pub const USAGE_EXAMPLE: &str = "goldberg-patcher \"~/games/My Game\"";
}

#[cfg(target_arch = "x86")]
const ARCH_FOLDER: &str = "x32";
#[cfg(target_arch = "x86_64")]
const ARCH_FOLDER: &str = "x64";

use crate::consts::*;

// -------------- ACTUAL PROGRAM -------------- 

const GBE_GITHUB_DL: &str = "https://github.com/Detanup01/gbe_fork/releases/latest/download/";


#[tokio::main]
async fn main() {
    let mut args = env::args();
    args.next();
    
    let game_path = if let Some(path) = args.next() {
        path.clone()
    } else {
        panic!("Please specify the path of the game you want to patch: `{USAGE_EXAMPLE}`.");
    };
    
    if let Some(_) = args.next() {
        panic!("This command supports only one argument.");
    }
    
    download_goldberg().await;
    print("Extracting golberg...");
    
    decompress();
    
    replace_libs(&Path::new(&game_path));
    print("Cleanup...");
    print("");
    
    _ = remove_dir_all("gbe_fork_extracted");
    _ = remove_file(GBE_FILE);
}

async fn download_goldberg() {
    let mut gbe_file = File::create_new(GBE_FILE).unwrap();
    
    let mut download_url = GBE_GITHUB_DL.to_string();
    download_url.push_str(GBE_FILE);
    
    let mut gbe_download = reqwest::get(download_url)
        .await.expect("Could not download goldberg emulator")
        .bytes_stream();
    
    
    let mut chunks = 1;
    while let Some(Ok(chunk)) = gbe_download.next().await {
        gbe_file.write_all(&chunk).unwrap();
        print(format!("Downloading latest gbe_fork release: {chunks} chunk(s) downloaded..."));
        std::io::stdout().flush().unwrap();
        
        chunks += 1; 
    }
}

fn replace_libs(game_path: &Path) {
    for lib_name in STEAM_LIBS {
        let Some(original_file) = find_file_path(lib_name, &game_path) else {
            print(format!("\r> Could not find {lib_name}, continuing..."));
            continue;
        };
        
        let gbe_file = PathBuf::from_iter(["gbe_fork_extracted", "release", "regular", ARCH_FOLDER, lib_name]);
        fs::write(original_file, fs::read(gbe_file).unwrap()).unwrap();
    }
}

fn find_file_path(file_name: &str, path: &Path) -> Option<PathBuf> {
    print(format!("> Searching for {file_name} in '{}'", path.file_name().unwrap().to_str().unwrap()));
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            if let Some(buf) = find_file_path(file_name, &entry.path()) {
                return Some(buf);
            } 
        } else {
            if entry.file_name() == file_name {
                return Some(entry.path());
            }
        }
    }
    
    None
}

#[cfg(target_os = "windows")]
fn uncompress() {
    // TODO: Test this
    sevenz_rust2::decompress_file(GBE_FILE, "gbe_fork_extracted").unwrap();
}

#[cfg(target_os = "linux")]
fn decompress() {
    use decompress::{decompressors::tarbz::Tarbz, Decompressor, ExtractOptsBuilder};
    Tarbz::default()
        .decompress(
            Path::new(GBE_FILE),
            Path::new("gbe_fork_extracted"),
            &ExtractOptsBuilder::default().build().unwrap()
        ).unwrap();
}
//


fn print(log: impl Display) {
    print!("\u{1b}[2K\r{log}");
    io::stdout().flush().unwrap();
}
