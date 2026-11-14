use crate::engine::EmailEngine;
use crate::models::{Category, EmailRecord, ProcessedEmail};
use std::sync::OnceLock;

static GLOBAL_ENGINE: OnceLock<EmailEngine> = OnceLock::new();

fn get_engine() -> &'static EmailEngine {
    GLOBAL_ENGINE.get_or_init(|| {
        EmailEngine::new().expect("Failed to initialize static EmailEngine for WASM")
    })
}

fn html_to_plain_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    let mut in_script_or_style = false;
    let mut current_tag = String::new();

    let chars: Vec<char> = html.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if in_script_or_style {
            if chars[i] == '<' && i + 1 < chars.len() && chars[i + 1] == '/' {
                let rest: String = chars[i..std::cmp::min(i + 15, chars.len())].iter().collect();
                let lower = rest.to_lowercase();
                if lower.starts_with("</script>") {
                    in_script_or_style = false;
                    i += "</script>".len();
                    continue;
                } else if lower.starts_with("</style>") {
                    in_script_or_style = false;
                    i += "</style>".len();
                    continue;
                }
            }
            i += 1;
            continue;
        }

        if chars[i] == '<' {
            in_tag = true;
            current_tag.clear();
            i += 1;
            continue;
        }

        if in_tag {
            if chars[i] == '>' {
                in_tag = false;
                let lower_tag = current_tag.to_lowercase();
                let tag_name = lower_tag.split_whitespace().next().unwrap_or("");
                if tag_name == "script" || tag_name == "style" {
                    in_script_or_style = true;
                } else if tag_name == "br"
                    || tag_name == "p"
                    || tag_name == "/p"
                    || tag_name == "div"
                    || tag_name == "/div"
                    || tag_name == "tr"
                    || tag_name == "/tr"
                    || tag_name == "li"
                    || tag_name == "h1"
                    || tag_name == "h2"
                    || tag_name == "h3"
                {
                    out.push('\n');
                }
            } else {
                current_tag.push(chars[i]);
            }
            i += 1;
            continue;
        }

        out.push(chars[i]);
        i += 1;
    }

    let decoded = out
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'");

    let mut cleaned = String::with_capacity(decoded.len());
    let mut newline_count = 0;
    for line in decoded.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if newline_count < 2 {
                cleaned.push('\n');
                newline_count += 1;
            }
        } else {
            cleaned.push_str(trimmed);
            cleaned.push('\n');
            newline_count = 0;
        }
    }
    cleaned
}

fn parse_from_value(val: &serde_json::Value) -> (String, String) {
    match val {
        serde_json::Value::Array(arr) => {
            if let Some(first) = arr.first() {
                parse_from_value(first)
            } else {
                ("unknown@example.com".to_string(), "Unknown Sender".to_string())
            }
        }
        serde_json::Value::Object(obj) => {
            let email = obj
                .get("email")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown@example.com")
                .to_string();
            let name = obj
                .get("name")
                .and_then(|v| v.as_str())
                .filter(|s| !s.trim().is_empty())
                .unwrap_or(&email)
                .to_string();
            (email, name)
        }
        serde_json::Value::String(s) => {
            if let Some(open) = s.find('<')
                && let Some(close) = s.find('>')
                && open < close
            {
                let name = s[..open].trim().to_string();
                let email = s[open + 1..close].trim().to_string();
                let final_name = if name.is_empty() { email.clone() } else { name };
                (email, final_name)
            } else {
                (s.clone(), s.clone())
            }
        }
        _ => ("unknown@example.com".to_string(), "Unknown Sender".to_string()),
    }
}

/// Helper input structure for processing arbitrary emails from JavaScript / JSON.
#[derive(serde::Deserialize, Default, Debug)]
pub struct WasmEmailInput {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub sender: Option<String>,
    #[serde(default, alias = "senderName")]
    pub sender_name: Option<String>,
    #[serde(default)]
    pub from: Option<serde_json::Value>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(alias = "textBody", alias = "content", default)]
    pub body: Option<String>,
    #[serde(alias = "htmlBody", default)]
    pub html_body: Option<String>,
    #[serde(alias = "date", alias = "receivedAt", default)]
    pub received_at: Option<String>,
}

impl WasmEmailInput {
    pub fn into_email_record(self) -> EmailRecord {
        let (sender, sender_name) = match (&self.sender, &self.sender_name, &self.from) {
            (Some(s), Some(sn), _) => (s.clone(), sn.clone()),
            (Some(s), None, _) => (s.clone(), s.clone()),
            (None, Some(sn), _) => ("unknown@example.com".to_string(), sn.clone()),
            (None, None, Some(from_val)) => parse_from_value(from_val),
            (None, None, None) => ("unknown@example.com".to_string(), "Unknown Sender".to_string()),
        };

        let body = match (self.body, self.html_body) {
            (Some(b), _) if !b.trim().is_empty() => b,
            (_, Some(h)) if !h.trim().is_empty() => html_to_plain_text(&h),
            (Some(b), _) => b,
            _ => String::new(),
        };

        EmailRecord {
            id: self.id.unwrap_or_else(|| "email_custom".to_string()),
            sender,
            sender_name,
            subject: self.subject.unwrap_or_default(),
            received_at: self
                .received_at
                .unwrap_or_else(|| "2026-11-05T00:00:00Z".to_string()),
            body,
            expected_category: Category::General,
            expected_extraction: serde_json::Value::Null,
        }
    }
}

/// Process an arbitrary email represented as a JSON string.
/// Returns the ProcessedEmail serialized to JSON.
pub fn process_email_json_internal(email_json: &str) -> Result<String, String> {
    let input: WasmEmailInput =
        serde_json::from_str(email_json).map_err(|e| format!("Invalid email JSON input: {e}"))?;
    let record = input.into_email_record();
    let processed: ProcessedEmail = get_engine().process(&record);
    serde_json::to_string(&processed).map_err(|e| format!("Serialization error: {e}"))
}

/// Process a JSON array of emails: `[ { "subject": "...", "body": "..." }, ... ]`
/// Returns a serialized JSON array of ProcessedEmail.
pub fn process_batch_json_internal(batch_json: &str) -> Result<String, String> {
    let inputs: Vec<WasmEmailInput> =
        serde_json::from_str(batch_json).map_err(|e| format!("Invalid batch JSON input: {e}"))?;
    let records: Vec<EmailRecord> = inputs
        .into_iter()
        .map(|inp| inp.into_email_record())
        .collect();
    let processed: Vec<ProcessedEmail> = get_engine().process_batch(&records);
    serde_json::to_string(&processed).map_err(|e| format!("Serialization error: {e}"))
}

/// Process an arbitrary email provided as separate string parameters.
pub fn process_raw_email_internal(
    id: &str,
    sender: &str,
    sender_name: &str,
    subject: &str,
    body: &str,
) -> Result<String, String> {
    let record = EmailRecord {
        id: id.to_string(),
        sender: sender.to_string(),
        sender_name: sender_name.to_string(),
        subject: subject.to_string(),
        received_at: "2026-11-05T00:00:00Z".to_string(),
        body: body.to_string(),
        expected_category: Category::General,
        expected_extraction: serde_json::Value::Null,
    };
    let processed = get_engine().process(&record);
    serde_json::to_string(&processed).map_err(|e| format!("Serialization error: {e}"))
}

/// Fast category detection for any arbitrary subject and body.
pub fn classify_email_internal(subject: &str, body: &str) -> String {
    let record = EmailRecord {
        id: "temp".to_string(),
        sender: String::new(),
        sender_name: String::new(),
        subject: subject.to_string(),
        received_at: String::new(),
        body: body.to_string(),
        expected_category: Category::General,
        expected_extraction: serde_json::Value::Null,
    };
    let cats = get_engine().detect_categories(&record);
    cats.first()
        .copied()
        .unwrap_or(Category::General)
        .as_str()
        .to_string()
}

/// Fast contextual preview generation for any arbitrary subject and body.
pub fn generate_email_preview_internal(subject: &str, body: &str) -> String {
    let record = EmailRecord {
        id: "temp".to_string(),
        sender: String::new(),
        sender_name: String::new(),
        subject: subject.to_string(),
        received_at: String::new(),
        body: body.to_string(),
        expected_category: Category::General,
        expected_extraction: serde_json::Value::Null,
    };
    let processed = get_engine().process(&record);
    processed.preview
}

// -----------------------------------------------------------------------------
// wasm-bindgen FFI Exports
// -----------------------------------------------------------------------------
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn process_email_json(email_json: &str) -> Result<String, JsValue> {
    process_email_json_internal(email_json).map_err(|e| JsValue::from_str(&e))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn process_raw_email(
    id: &str,
    sender: &str,
    sender_name: &str,
    subject: &str,
    body: &str,
) -> Result<String, JsValue> {
    process_raw_email_internal(id, sender, sender_name, subject, body)
        .map_err(|e| JsValue::from_str(&e))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn classify_email(subject: &str, body: &str) -> String {
    classify_email_internal(subject, body)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn generate_email_preview(subject: &str, body: &str) -> String {
    generate_email_preview_internal(subject, body)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn process_batch_json(batch_json: &str) -> Result<String, JsValue> {
    process_batch_json_internal(batch_json).map_err(|e| JsValue::from_str(&e))
}

// -----------------------------------------------------------------------------
// Universal C-ABI WASM Exports (Zero-dependency, pure linear memory)
// -----------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn rover_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn rover_dealloc(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, len, len);
        }
    }
}

unsafe fn read_utf8(ptr: *const u8, len: usize) -> &'static str {
    if ptr.is_null() || len == 0 {
        return "";
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    std::str::from_utf8(slice).unwrap_or("")
}

fn package_string_response(s: String) -> *const u8 {
    let bytes = s.into_bytes();
    let len = bytes.len() as u32;
    let mut out = Vec::with_capacity(4 + bytes.len());
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(&bytes);
    let ptr = out.as_ptr();
    std::mem::forget(out);
    ptr
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn rover_classify(
    subject_ptr: *const u8,
    subject_len: usize,
    body_ptr: *const u8,
    body_len: usize,
) -> *const u8 {
    let subject = unsafe { read_utf8(subject_ptr, subject_len) };
    let body = unsafe { read_utf8(body_ptr, body_len) };
    let category = classify_email_internal(subject, body);
    package_string_response(category)
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn rover_preview(
    subject_ptr: *const u8,
    subject_len: usize,
    body_ptr: *const u8,
    body_len: usize,
) -> *const u8 {
    let subject = unsafe { read_utf8(subject_ptr, subject_len) };
    let body = unsafe { read_utf8(body_ptr, body_len) };
    let preview = generate_email_preview_internal(subject, body);
    package_string_response(preview)
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn rover_process_email(
    subject_ptr: *const u8,
    subject_len: usize,
    body_ptr: *const u8,
    body_len: usize,
) -> *const u8 {
    let subject = unsafe { read_utf8(subject_ptr, subject_len) };
    let body = unsafe { read_utf8(body_ptr, body_len) };
    let json =
        process_raw_email_internal("msg_wasm", "unknown@relay.mail", "Sender", subject, body)
            .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
    package_string_response(json)
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn rover_process_email_json(
    json_ptr: *const u8,
    json_len: usize,
) -> *const u8 {
    let json_str = unsafe { read_utf8(json_ptr, json_len) };
    let out =
        process_email_json_internal(json_str).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
    package_string_response(out)
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn rover_process_batch_json(
    json_ptr: *const u8,
    json_len: usize,
) -> *const u8 {
    let json_str = unsafe { read_utf8(json_ptr, json_len) };
    let out =
        process_batch_json_internal(json_str).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
    package_string_response(out)
}

#[unsafe(no_mangle)]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn rover_html_to_text(html_ptr: *const u8, html_len: usize) -> *const u8 {
    let html = unsafe { read_utf8(html_ptr, html_len) };
    let text = html_to_plain_text(html);
    package_string_response(text)
}
