use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemKind {
    File,
    Folder,
    Image,
    Text,
    Url,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StashItem {
    pub id: String,
    pub kind: ItemKind,
    pub name: String,
    pub path: Option<String>,
    pub size_bytes: Option<u64>,
    pub formatted_size: String,
    pub extension: String,
    pub text_preview: Option<String>,
    pub created_at: u64,
}

impl StashItem {
    pub fn from_path(path_str: &str) -> Self {
        let p = Path::new(path_str);
        let name = p
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(path_str)
            .to_string();

        let ext = p
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let is_dir = p.is_dir();
        let metadata = p.metadata().ok();
        let size_bytes = metadata.map(|m| m.len());

        let kind = if is_dir {
            ItemKind::Folder
        } else if matches!(
            ext.as_str(),
            "png" | "jpg" | "jpeg" | "webp" | "gif" | "svg" | "bmp" | "ico"
        ) {
            ItemKind::Image
        } else {
            ItemKind::File
        };

        let formatted_size = if is_dir {
            "Папка".to_string()
        } else if let Some(bytes) = size_bytes {
            format_file_size(bytes)
        } else {
            "0 Б".to_string()
        };

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let count = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            id: format!("{}_{}_{}" , now, count, sanitize_id(&name)),
            kind,
            name,
            path: Some(path_str.to_string()),
            size_bytes,
            formatted_size,
            extension: if is_dir { "folder".into() } else { ext },
            text_preview: None,
            created_at: now,
        }
    }

    pub fn from_text(text: &str) -> Self {
        let trimmed = text.trim();
        let is_url = trimmed.starts_with("http://") || trimmed.starts_with("https://");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let name = if is_url {
            let without_proto = trimmed
                .trim_start_matches("https://")
                .trim_start_matches("http://");
            let host = without_proto.split('/').next().unwrap_or("Ссылка");
            host.to_string()
        } else {
            // Для многострочного текста берём первую непустую строку (до 30 символов)
            trimmed
                .lines()
                .map(|l| l.trim())
                .find(|l| !l.is_empty())
                .unwrap_or(trimmed)
                .chars()
                .take(30)
                .collect::<String>()
        };

        let count = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            id: format!("{}_{}_text", now, count),
            kind: if is_url { ItemKind::Url } else { ItemKind::Text },
            name,
            path: None,
            size_bytes: Some(trimmed.len() as u64),
            formatted_size: format!("{} симв.", trimmed.chars().count()),
            extension: if is_url { "url".into() } else { "txt".into() },
            text_preview: Some(trimmed.to_string()),
            created_at: now,
        }
    }
}

pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} ГБ", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} МБ", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} КБ", bytes as f64 / KB as f64)
    } else {
        format!("{} Б", bytes)
    }
}

fn sanitize_id(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .take(12)
        .collect()
}
