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
pub const ARCH_FOLDER: &str = "x32";
#[cfg(target_arch = "x86_64")]
pub const ARCH_FOLDER: &str = "x64";

// The file is shipped in different formats (.7z and .tar.bz2) for each system

#[cfg(target_os = "windows")]
pub fn decompress() {
    // TODO: Test this
    sevenz_rust2::decompress_file(GBE_FILE, GBE_EXTRACTED_FOLDER).unwrap();
}

#[cfg(target_os = "linux")]
pub fn decompress() {
    use std::path::Path;

    use decompress::{decompressors::tarbz::Tarbz, Decompressor, ExtractOptsBuilder};

    Tarbz::default()
        .decompress(
            Path::new(GBE_FILE),
            Path::new(crate::GBE_EXTRACTED_FOLDER),
            &ExtractOptsBuilder::default().build().unwrap()
        ).unwrap();
}

pub const GBE_GITHUB_DL: &str = "https://github.com/Detanup01/gbe_fork/releases/latest/download/";
pub const GBE_EXTRACTED_FOLDER: &str = "gbe_extracted";

pub use consts::*;
