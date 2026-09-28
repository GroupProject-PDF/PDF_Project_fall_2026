$rustFiles = @(
    "src-tauri/src/commands.rs",
    "src-tauri/src/pdf/merge.rs",
    "src-tauri/src/pdf/split.rs",
    "src-tauri/src/pdf/render.rs",
    "src-tauri/src/db/schema.rs"
)

foreach ($file in $rustFiles) {
    if (-not (Test-Path $file)) {
        New-Item -ItemType File -Path $file -Force | Out-Null
    }
}pub mod schema;
