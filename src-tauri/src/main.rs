#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod tree;
pub mod command_error;
use crate::tree::BinaryTree;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn create_new_tree() -> BinaryTree {
    let tree = BinaryTree::from(&[1,2,3,4,5,6]);
    tree
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_new_tree])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
