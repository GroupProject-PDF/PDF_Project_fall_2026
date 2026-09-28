# Project Context: Local PDF Manager

## Tech Stack
- Backend: Rust (Tauri v2)
- Frontend: React + TypeScript + Tailwind CSS (Vite)
- Local DB: SQLite (rusqlite)
- PDF Crates: lopdf, pdfium-render
- Primary Rule: 100% local desktop operation. NO cloud APIs or network calls.

## Directory Layout
- src-tauri/src/pdf/: Binary PDF operations (merge.rs, split.rs, render.rs)
- src-tauri/src/db/: Local SQLite schema (schema.rs)
- src-tauri/src/commands.rs: Bridge between Rust functions and React UI
- src/components/: React UI components (FileUploader.tsx, PDFViewer.tsx)
