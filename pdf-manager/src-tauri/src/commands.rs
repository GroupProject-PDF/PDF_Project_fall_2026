use crate::pdf::merge::merge_pdfs;

#[tauri::command]
pub fn merge_pdfs_command(paths: Vec<String>, output_path: String) -> Result<(), String> {
    merge_pdfs(paths, output_path).map_err(|e| e.to_string())
}
