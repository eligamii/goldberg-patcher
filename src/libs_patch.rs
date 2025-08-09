use std::path::{Path, PathBuf};

use crate::{cross::*, helpers::{print_at_current_line, find_file_path}};

pub fn replace_libs(game_path: &Path) {
    for lib_name in STEAM_LIBS {
        let Some(original_file) = find_file_path(lib_name, &game_path) else {
            print_at_current_line(format!("\rCould not find {lib_name}, continuing..."));
            continue;
        };
        
        
        let gbe_file = PathBuf::from_iter([GBE_EXTRACTED_FOLDER, "release", "regular", ARCH_FOLDER, lib_name]);
        
        // Backup the original files first if not already backed up
        let mut backup_file = original_file.clone();
        backup_file.pop();
        backup_file.push(format!("non_gbe_patched_{lib_name}.backup").as_str());
        
        if !backup_file.exists() { 
            // Avoid doing the backup of a previously patched lib
            std::fs::rename(original_file.clone(), backup_file).unwrap();
        } 
        
        std::fs::write(original_file, std::fs::read(gbe_file).unwrap()).unwrap();
    }
}