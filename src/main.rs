use std::{env, fs::{remove_file, File}, io::Write, path::Path};
use futures::StreamExt;

mod cross;
mod libs_patch;
mod helpers;

use cross::*;

use crate::{helpers::print_at_current_line, libs_patch::replace_libs};


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
    
    if !Path::new(GBE_EXTRACTED_FOLDER).exists() {
        download_goldberg().await;
        print_at_current_line("Extracting golberg...");
        decompress();
    } else {
        println!("> Goldberg already downloaded (at '.{}{}')", std::path::MAIN_SEPARATOR, GBE_EXTRACTED_FOLDER);
    }
    
    
    replace_libs(&Path::new(&game_path));
    
    
    print_at_current_line("Cleanup...");
    print_at_current_line("");
    
    // Dont care if this fail
    _ = remove_file(GBE_FILE);
}

async fn download_goldberg() {
    let Ok(mut gbe_file) = File::create_new(GBE_FILE) else {
        return;
    };
    
    let mut download_url = GBE_GITHUB_DL.to_string();
    download_url.push_str(GBE_FILE);
    
    let mut gbe_download = reqwest::get(download_url)
        .await.expect("Could not download goldberg emulator")
        .bytes_stream();
    
    
    let mut chunks = 1;
    while let Some(Ok(chunk)) = gbe_download.next().await {
        gbe_file.write_all(&chunk).unwrap();
        print_at_current_line(format!("Downloading latest gbe_fork release: {chunks} chunk(s) downloaded..."));
        std::io::stdout().flush().unwrap();
        
        chunks += 1; 
    }
}




