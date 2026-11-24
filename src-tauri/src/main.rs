// Logigramme, version bureau.
// L'interface (index.html) est identique à la version web ; seul le stockage change :
// tout est écrit dans des fichiers sur le disque, rien dans le stockage du navigateur.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::STANDARD, Engine};
use std::{fs, io::ErrorKind, path::PathBuf};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;

const SESSION_FILE: &str = "session.json";

/// Dossier de données local à la machine (Windows : %LOCALAPPDATA%\io.github.onlyv.logigramme).
/// Volontairement pas %APPDATA% : ce dossier peut être synchronisé avec un profil itinérant en entreprise.
fn data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

/// Écriture atomique : fichier temporaire puis renommage, pour ne jamais laisser un fichier à moitié écrit.
fn write_atomic(path: &PathBuf, bytes: &[u8]) -> Result<(), String> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, bytes).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())
}

/// Relit la dernière session (logigramme en cours, zoom, fichier ouvert).
#[tauri::command]
fn load_session(app: AppHandle) -> Result<Option<String>, String> {
    let path = data_dir(&app)?.join(SESSION_FILE);
    match fs::read_to_string(&path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Sauvegarde automatique de la session.
#[tauri::command]
fn save_session(app: AppHandle, content: String) -> Result<(), String> {
    let path = data_dir(&app)?.join(SESSION_FILE);
    write_atomic(&path, content.as_bytes())
}

/// Boîte de dialogue "Ouvrir" : renvoie [chemin, contenu] ou rien si annulé.
#[tauri::command]
async fn open_json(app: AppHandle) -> Result<Option<(String, String)>, String> {
    let picked = app
        .dialog()
        .file()
        .set_title("Ouvrir un logigramme")
        .add_filter("Logigramme", &["json"])
        .blocking_pick_file();
    let Some(fp) = picked else { return Ok(None) };
    let path = fp.into_path().map_err(|e| e.to_string())?;
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Ok(Some((path.to_string_lossy().into_owned(), content)))
}

fn ask_save_path(app: &AppHandle, default_name: &str, ext: &str) -> Result<Option<PathBuf>, String> {
    let label = match ext {
        "json" => "Logigramme",
        "svg" => "Image SVG",
        "png" => "Image PNG",
        _ => "Fichier",
    };
    let picked = app
        .dialog()
        .file()
        .set_title("Enregistrer")
        .set_file_name(default_name)
        .add_filter(label, &[ext])
        .blocking_save_file();
    match picked {
        None => Ok(None),
        Some(fp) => {
            let mut p = fp.into_path().map_err(|e| e.to_string())?;
            if p.extension().map(|e| e.to_string_lossy().to_lowercase()) != Some(ext.to_string()) {
                p.set_extension(ext);
            }
            Ok(Some(p))
        }
    }
}

/// Enregistre du texte (JSON, SVG). Si `path` est fourni, écrit directement dedans
/// (Ctrl+S sur un fichier déjà ouvert), sinon demande où enregistrer.
#[tauri::command]
async fn save_text(
    app: AppHandle,
    path: Option<String>,
    default_name: String,
    ext: String,
    content: String,
) -> Result<Option<String>, String> {
    let target = match path.map(PathBuf::from) {
        Some(p) => Some(p),
        None => ask_save_path(&app, &default_name, &ext)?,
    };
    let Some(p) = target else { return Ok(None) };
    write_atomic(&p, content.as_bytes())?;
    Ok(Some(p.to_string_lossy().into_owned()))
}

/// Enregistre un fichier binaire reçu en base64 (export PNG).
#[tauri::command]
async fn save_binary(app: AppHandle, default_name: String, ext: String, b64: String) -> Result<Option<String>, String> {
    let bytes = STANDARD.decode(b64.as_bytes()).map_err(|e| e.to_string())?;
    let Some(p) = ask_save_path(&app, &default_name, &ext)? else { return Ok(None) };
    write_atomic(&p, &bytes)?;
    Ok(Some(p.to_string_lossy().into_owned()))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_session,
            save_session,
            open_json,
            save_text,
            save_binary
        ])
        .run(tauri::generate_context!())
        .expect("impossible de démarrer Logigramme");
}
