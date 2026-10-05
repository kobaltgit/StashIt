use crate::{models::StashItem, AppState};
use qrcode::{render::svg, QrCode};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    net::{IpAddr, UdpSocket},
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Emitter, Manager, State};
use tiny_http::{Header, Response, Server, StatusCode};

const MOBILE_HTML: &str = include_str!("mobile.html");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalDropInfo {
    pub url: String,
    pub qr_svg: String,
    pub port: u16,
    pub token: String,
}

pub struct LocalDropManager {
    running_flag: Arc<AtomicBool>,
    current_info: Mutex<Option<LocalDropInfo>>,
}

impl Default for LocalDropManager {
    fn default() -> Self {
        Self {
            running_flag: Arc::new(AtomicBool::new(false)),
            current_info: Mutex::new(None),
        }
    }
}

/// Находит локальный IPv4 адрес в текущей Wi-Fi/LAN сети
pub fn get_local_ip() -> Option<IpAddr> {
    // Способ 1: через UDP роутинг (быстро и без реального трафика)
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                let ip = addr.ip();
                if !ip.is_loopback() {
                    return Some(ip);
                }
            }
        }
    }

    // Способ 2: роутинг к локальному шлюзу
    for test_target in &["192.168.1.1:80", "192.168.0.1:80", "10.0.0.1:80"] {
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            if socket.connect(test_target).is_ok() {
                if let Ok(addr) = socket.local_addr() {
                    let ip = addr.ip();
                    if !ip.is_loopback() {
                        return Some(ip);
                    }
                }
            }
        }
    }

    None
}

/// Генерация случайного буквенно-цифрового токена без сторонних зависимостей
fn generate_token() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let pid = std::process::id();
    format!("{:x}{:x}", now, pid)
}

/// Путь к временной папке для загрузок с мобильного
fn get_mobile_upload_dir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push("StashIt");
    dir.push("mobile_uploads");
    let _ = fs::create_dir_all(&dir);
    dir
}

/// MIME тип по расширению
fn mime_by_ext(ext: &str) -> &'static str {
    match ext {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "txt" => "text/plain; charset=utf-8",
        "json" => "application/json",
        "zip" => "application/zip",
        _ => "application/octet-stream",
    }
}

/// Запуск сессии Local Drop
#[tauri::command]
pub fn start_local_drop(
    app: AppHandle,
    _state: State<'_, AppState>,
    manager: State<'_, LocalDropManager>,
) -> Result<LocalDropInfo, String> {
    // Если уже запущено, возвращаем текущую информацию
    if manager.running_flag.load(Ordering::Relaxed) {
        if let Some(info) = manager.current_info.lock().unwrap().clone() {
            return Ok(info);
        }
    }

    // Определяем IP в локальной сети
    let local_ip = get_local_ip().unwrap_or(IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));

    // Биндимся на любой свободный порт
    let server = Server::http("0.0.0.0:0").map_err(|e| format!("Не удалось запустить сервер: {}", e))?;
    let port = server.server_addr().to_ip().map(|a| a.port()).unwrap_or(0);
    if port == 0 {
        return Err("Не удалось определить порт сервера".to_string());
    }

    let token = generate_token();
    let url = format!("http://{}:{}/?token={}", local_ip, port, token);

    // Генерируем SVG QR-код
    let qr = QrCode::new(url.as_bytes()).map_err(|e| format!("QR ошибка: {}", e))?;
    let qr_svg = qr
        .render::<svg::Color>()
        .min_dimensions(200, 200)
        .quiet_zone(false)
        .build();

    let info = LocalDropInfo {
        url,
        qr_svg,
        port,
        token: token.clone(),
    };

    *manager.current_info.lock().unwrap() = Some(info.clone());

    let running_flag = manager.running_flag.clone();
    running_flag.store(true, Ordering::Relaxed);

    let last_activity = Arc::new(AtomicU64::new(
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    ));

    let app_handle = app.clone();
    let server_token = token;

    // Запускаем фоновый поток обработки HTTP-запросов
    thread::spawn(move || {
        let timeout_duration = Duration::from_millis(500);

        while running_flag.load(Ordering::Relaxed) {
            // Проверка 5-минутного таймаута неактивности
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let last = last_activity.load(Ordering::Relaxed);
            if now.saturating_sub(last) > 300 {
                // 5 минут бездействия — автовыключение сервера
                break;
            }

            match server.recv_timeout(timeout_duration) {
                Ok(Some(request)) => {
                    last_activity.store(now, Ordering::Relaxed);
                    handle_request(request, &server_token, &app_handle);
                }
                Ok(None) => {
                    // Таймаут poll, продолжаем цикл проверки running_flag
                }
                Err(_) => {
                    break;
                }
            }
        }

        running_flag.store(false, Ordering::Relaxed);
    });

    Ok(info)
}

/// Остановка сессии Local Drop
#[tauri::command]
pub fn stop_local_drop(manager: State<'_, LocalDropManager>) -> Result<(), String> {
    manager.running_flag.store(false, Ordering::Relaxed);
    *manager.current_info.lock().unwrap() = None;
    Ok(())
}

/// Обработка входящего HTTP-запроса от мобильного устройства
fn handle_request(mut request: tiny_http::Request, valid_token: &str, app: &AppHandle) {
    let url = request.url().to_string();
    let method = request.method().clone();

    // CORS заголовки
    let cors_header = Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap();
    let cors_methods = Header::from_bytes(
        &b"Access-Control-Allow-Methods"[..],
        &b"GET, POST, OPTIONS"[..],
    )
    .unwrap();
    let cors_headers = Header::from_bytes(
        &b"Access-Control-Allow-Headers"[..],
        &b"Content-Type, Authorization"[..],
    )
    .unwrap();

    if method == tiny_http::Method::Options {
        let resp = Response::empty(StatusCode(200))
            .with_header(cors_header)
            .with_header(cors_methods)
            .with_header(cors_headers);
        let _ = request.respond(resp);
        return;
    }

    // Проверка токена безопасности
    let has_valid_token = url.contains(&format!("token={}", valid_token));
    if !has_valid_token {
        let resp = Response::from_string("403 Forbidden: Invalid Token")
            .with_status_code(StatusCode(403))
            .with_header(cors_header);
        let _ = request.respond(resp);
        return;
    }

    let path = url.split('?').next().unwrap_or("/");

    let no_cache = Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, no-store, must-revalidate"[..]).unwrap();
    let pragma = Header::from_bytes(&b"Pragma"[..], &b"no-cache"[..]).unwrap();

    // 1. Главная страница mobile.html
    if path == "/" {
        let header = Header::from_bytes(
            &b"Content-Type"[..],
            &b"text/html; charset=utf-8"[..],
        )
        .unwrap();
        let resp = Response::from_string(MOBILE_HTML)
            .with_header(header)
            .with_header(cors_header)
            .with_header(no_cache)
            .with_header(pragma);
        let _ = request.respond(resp);
        return;
    }

    // 2. Список файлов в кармане StashIt (GET /api/items)
    if path == "/api/items" {
        let state: State<'_, AppState> = app.state();
        let items = state.items.lock().unwrap();
        let json = serde_json::to_string(&*items).unwrap_or_else(|_| "[]".to_string());
        let header = Header::from_bytes(
            &b"Content-Type"[..],
            &b"application/json; charset=utf-8"[..],
        )
        .unwrap();
        let resp = Response::from_string(json)
            .with_header(header)
            .with_header(cors_header)
            .with_header(no_cache)
            .with_header(pragma);
        let _ = request.respond(resp);
        return;
    }

    // 3. Скачивание файла с ПК на телефон (GET /api/download/<id>)
    if path.starts_with("/api/download/") {
        let raw_id = path.trim_start_matches("/api/download/");
        let decoded_id = percent_decode(raw_id).unwrap_or_else(|_| raw_id.to_string());
        let state: State<'_, AppState> = app.state();
        let items = state.items.lock().unwrap();
        if let Some(item) = items.iter().find(|it| it.id == decoded_id || it.id == raw_id) {
            if let Some(file_path_str) = &item.path {
                let file_path = Path::new(file_path_str);
                if file_path.is_file() {
                    if let Ok(file) = File::open(file_path) {
                        let file_size = file_path.metadata().map(|m| m.len()).unwrap_or(0);
                        let mime = mime_by_ext(&item.extension);
                        let content_type = Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap();

                        // Имя файла в заголовке с поддержкой Unicode (RFC 5987 / RFC 6266)
                        let ascii_name: String = item
                            .name
                            .chars()
                            .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
                            .collect();
                        let utf8_name = percent_encode(&item.name);
                        let disposition_str = format!("attachment; filename=\"{}\"; filename*=UTF-8''{}", ascii_name, utf8_name);
                        let disposition = Header::from_bytes(&b"Content-Disposition"[..], disposition_str.as_bytes()).unwrap();

                        let resp = Response::new(
                            StatusCode(200),
                            vec![cors_header, content_type, disposition],
                            file,
                            Some(file_size as usize),
                            None,
                        );
                        let _ = request.respond(resp);
                        return;
                    }
                }
            } else if let Some(text) = &item.text_preview {
                // Если это текст
                let header = Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=utf-8"[..]).unwrap();
                let resp = Response::from_string(text.clone())
                    .with_header(header)
                    .with_header(cors_header);
                let _ = request.respond(resp);
                return;
            }
        }

        let resp = Response::from_string("File Not Found")
            .with_status_code(StatusCode(404))
            .with_header(cors_header);
        let _ = request.respond(resp);
        return;
    }

    // 4. Превью картинок для мобильного интерфейса (GET /api/thumb/<id>)
    if path.starts_with("/api/thumb/") {
        let raw_id = path.trim_start_matches("/api/thumb/");
        let decoded_id = percent_decode(raw_id).unwrap_or_else(|_| raw_id.to_string());
        let state: State<'_, AppState> = app.state();
        let items = state.items.lock().unwrap();
        if let Some(item) = items.iter().find(|it| it.id == decoded_id || it.id == raw_id) {
            if let Some(file_path_str) = &item.path {
                let file_path = Path::new(file_path_str);
                if file_path.is_file() {
                    if let Ok(file) = File::open(file_path) {
                        let file_size = file_path.metadata().map(|m| m.len()).unwrap_or(0);
                        let mime = mime_by_ext(&item.extension);
                        let content_type = Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap();

                        let resp = Response::new(
                            StatusCode(200),
                            vec![cors_header, content_type],
                            file,
                            Some(file_size as usize),
                            None,
                        );
                        let _ = request.respond(resp);
                        return;
                    }
                }
            }
        }

        let resp = Response::from_string("Thumb Not Found")
            .with_status_code(StatusCode(404))
            .with_header(cors_header);
        let _ = request.respond(resp);
        return;
    }

    // 5. Загрузка файла с мобильного на ПК (POST /api/upload?name=...)
    if path == "/api/upload" && method == tiny_http::Method::Post {
        let query = url.split('?').nth(1).unwrap_or("");
        let mut file_name = "mobile_upload".to_string();
        for param in query.split('&') {
            if let Some(val) = param.strip_prefix("name=") {
                if let Ok(decoded) = percent_decode(val) {
                    if !decoded.trim().is_empty() {
                        file_name = decoded;
                    }
                }
            }
        }

        let upload_dir = get_mobile_upload_dir();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let safe_name: String = file_name
            .chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                other => other,
            })
            .collect();

        let target_path = upload_dir.join(format!("{}_{}", timestamp, safe_name));

        if let Ok(mut dest_file) = File::create(&target_path) {
            let reader = request.as_reader();
            let mut buffer = [0u8; 16384];
            let mut success = true;

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        if dest_file.write_all(&buffer[..n]).is_err() {
                            success = false;
                            break;
                        }
                    }
                    Err(_) => {
                        success = false;
                        break;
                    }
                }
            }

            if success {
                let path_str = target_path.to_string_lossy().to_string();
                let state: State<'_, AppState> = app.state();
                let mut items = state.items.lock().unwrap();
                let new_item = StashItem::from_path(&path_str);
                items.push(new_item);

                // Оповещаем фронтенд Tauri о добавлении файла
                let cloned = items.clone();
                let _ = app.emit("stash-updated", cloned);

                let resp = Response::from_string("{\"success\":true}")
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                    .with_header(cors_header);
                let _ = request.respond(resp);
                return;
            }
        }

        let resp = Response::from_string("Upload failed")
            .with_status_code(StatusCode(500))
            .with_header(cors_header);
        let _ = request.respond(resp);
        return;
    }

    // 6. Отправка текста или ссылки со смартфона на ПК (POST /api/upload-text)
    if path == "/api/upload-text" && method == tiny_http::Method::Post {
        let mut body = String::new();
        let reader = request.as_reader();
        if reader.read_to_string(&mut body).is_ok() {
            let text_val = if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&body) {
                parsed["text"].as_str().unwrap_or("").to_string()
            } else {
                body
            };

            if !text_val.trim().is_empty() {
                let state: State<'_, AppState> = app.state();
                let mut items = state.items.lock().unwrap();
                let new_item = StashItem::from_text(&text_val);
                items.push(new_item);

                // Оповещаем фронтенд Tauri
                let cloned = items.clone();
                let _ = app.emit("stash-updated", cloned);

                let resp = Response::from_string("{\"success\":true}")
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                    .with_header(cors_header);
                let _ = request.respond(resp);
                return;
            }
        }

        let resp = Response::from_string("Bad Request")
            .with_status_code(StatusCode(400))
            .with_header(cors_header);
        let _ = request.respond(resp);
        return;
    }

    // Default 404
    let resp = Response::from_string("Not Found")
        .with_status_code(StatusCode(404))
        .with_header(cors_header);
    let _ = request.respond(resp);
}

/// Декодирование URL процентов (например %20 -> пробел)
fn percent_decode(input: &str) -> Result<String, ()> {
    let mut bytes = Vec::new();
    let mut chars = input.bytes();

    while let Some(b) = chars.next() {
        if b == b'%' {
            let h1 = chars.next().ok_or(())?;
            let h2 = chars.next().ok_or(())?;
            let hex_arr = [h1, h2];
            let hex_str = std::str::from_utf8(&hex_arr).map_err(|_| ())?;
            let byte_val = u8::from_str_radix(hex_str, 16).map_err(|_| ())?;
            bytes.push(byte_val);
        } else if b == b'+' {
            bytes.push(b' ');
        } else {
            bytes.push(b);
        }
    }

    String::from_utf8(bytes).map_err(|_| ())
}

/// Кодирование имени файла в UTF-8 проценты для заголовка Content-Disposition
fn percent_encode(input: &str) -> String {
    let mut out = String::new();
    for b in input.bytes() {
        if b.is_ascii_alphanumeric() || b == b'.' || b == b'-' || b == b'_' || b == b'~' {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}
