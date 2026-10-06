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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShelfConfig {
    pub id: String,
    pub name: String,
    pub pinned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Shelf {
    pub id: String,
    pub name: String,
    pub pinned: bool,
    pub items: Vec<StashItem>,
}

impl Shelf {
    #[allow(dead_code)]
    pub fn new(id: String, name: String, pinned: bool) -> Self {
        Self {
            id,
            name,
            pinned,
            items: Vec::new(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 Б");
        assert_eq!(format_file_size(500), "500 Б");
        assert_eq!(format_file_size(1024), "1.0 КБ");
        assert_eq!(format_file_size(1536), "1.5 КБ");
        assert_eq!(format_file_size(1024 * 1024), "1.0 МБ");
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.0 ГБ");
        assert_eq!(format_file_size(5 * 1024 * 1024 * 1024), "5.0 ГБ");
    }

    #[test]
    fn test_sanitize_id() {
        assert_eq!(sanitize_id("hello-world.txt"), "helloworldtx");
        assert_eq!(sanitize_id("test_123!@#"), "test123");
        assert_eq!(sanitize_id(""), "");
    }

    #[test]
    fn test_stash_item_from_text() {
        let plain = "Hello, world!\nLine 2";
        let item = StashItem::from_text(plain);
        assert_eq!(item.kind, ItemKind::Text);
        assert_eq!(item.name, "Hello, world!");
        assert_eq!(item.extension, "txt");
        assert_eq!(item.text_preview.as_deref(), Some(plain));
        assert!(item.path.is_none());
        assert_eq!(item.size_bytes, Some(plain.len() as u64));
    }

    #[test]
    fn test_stash_item_from_url() {
        let url = "https://github.com/kobaltgit/StashIt";
        let item = StashItem::from_text(url);
        assert_eq!(item.kind, ItemKind::Url);
        assert_eq!(item.name, "github.com");
        assert_eq!(item.extension, "url");
        assert_eq!(item.text_preview.as_deref(), Some(url));
        assert!(item.path.is_none());
    }

    #[test]
    fn test_stash_item_from_path() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("stashit_unit_test_img.png");
        std::fs::write(&test_file, b"fake png data").unwrap();

        let item = StashItem::from_path(test_file.to_str().unwrap());
        assert_eq!(item.kind, ItemKind::Image);
        assert_eq!(item.name, "stashit_unit_test_img.png");
        assert_eq!(item.extension, "png");
        assert!(item.path.is_some());
        assert!(item.size_bytes.unwrap() > 0);

        let _ = std::fs::remove_file(&test_file);
    }

    #[test]
    fn test_stash_item_from_dir_path() {
        let temp_dir = std::env::temp_dir();
        let item = StashItem::from_path(temp_dir.to_str().unwrap());
        assert_eq!(item.kind, ItemKind::Folder);
        assert_eq!(item.extension, "folder");
        assert_eq!(item.formatted_size, "Папка");
    }

    #[test]
    fn test_shelf_creation() {
        let mut shelf = Shelf::new("test_id".into(), "Проект".into(), true);
        assert_eq!(shelf.id, "test_id");
        assert_eq!(shelf.name, "Проект");
        assert!(shelf.pinned);
        assert!(shelf.items.is_empty());

        shelf.items.push(StashItem::from_text("Sample"));
        assert_eq!(shelf.items.len(), 1);
    }

    #[test]
    fn test_shelf_config_serde() {
        let cfg = ShelfConfig {
            id: "work".into(),
            name: "Работа".into(),
            pinned: true,
        };
        let json = serde_json::to_string(&cfg).unwrap();
        let deserialized: ShelfConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, cfg);
    }
}
