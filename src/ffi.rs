use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// ---------------------------------------------------------------------------
// UniFFI Function Implementations (matched by src/rover.udl)
// ---------------------------------------------------------------------------

pub fn classify(subject: String, body: String) -> String {
    crate::wasm::classify_email_internal(&subject, &body)
}

pub fn generate_preview(subject: String, body: String) -> String {
    crate::wasm::generate_email_preview_internal(&subject, &body)
}

pub fn process_email(subject: String, body: String) -> String {
    crate::wasm::process_raw_email_internal(
        "msg_uniffi",
        "unknown@relay.mail",
        "Sender",
        &subject,
        &body,
    )
    .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
}

pub fn process_email_json(email_json: String) -> String {
    crate::wasm::process_email_json_internal(&email_json)
        .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
}

pub fn process_batch_json(batch_json: String) -> String {
    crate::wasm::process_batch_json_internal(&batch_json)
        .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
}

pub fn html_to_text(html: String) -> String {
    crate::wasm::html_to_plain_text(&html)
}

pub fn clean_email_body(body_or_html: String) -> String {
    if body_or_html.contains('<') && body_or_html.contains('>') {
        crate::wasm::html_to_plain_text(&body_or_html)
    } else {
        body_or_html.replace("\r\n", "\n").trim().to_string()
    }
}

// UniFFI scaffolding is included in src/lib.rs crate root

// ---------------------------------------------------------------------------
// Direct C-ABI FFI Exports (For zero-overhead Hermes C++ JSI integration)
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn rover_native_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

fn to_c_string(s: String) -> *mut c_char {
    CString::new(s)
        .unwrap_or_else(|_| CString::new("").unwrap())
        .into_raw()
}

unsafe fn c_str_to_str<'a>(ptr: *const c_char) -> &'a str {
    if ptr.is_null() {
        ""
    } else {
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap_or("")
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rover_native_classify(
    subject: *const c_char,
    body: *const c_char,
) -> *mut c_char {
    let s = unsafe { c_str_to_str(subject) };
    let b = unsafe { c_str_to_str(body) };
    to_c_string(crate::wasm::classify_email_internal(s, b))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rover_native_generate_preview(
    subject: *const c_char,
    body: *const c_char,
) -> *mut c_char {
    let s = unsafe { c_str_to_str(subject) };
    let b = unsafe { c_str_to_str(body) };
    to_c_string(crate::wasm::generate_email_preview_internal(s, b))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rover_native_process_email(
    subject: *const c_char,
    body: *const c_char,
) -> *mut c_char {
    let s = unsafe { c_str_to_str(subject) };
    let b = unsafe { c_str_to_str(body) };
    let res = crate::wasm::process_raw_email_internal(
        "msg_native",
        "unknown@relay.mail",
        "Sender",
        s,
        b,
    )
    .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
    to_c_string(res)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rover_native_process_email_json(email_json: *const c_char) -> *mut c_char {
    let json = unsafe { c_str_to_str(email_json) };
    let res = crate::wasm::process_email_json_internal(json)
        .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
    to_c_string(res)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rover_native_process_batch_json(batch_json: *const c_char) -> *mut c_char {
    let json = unsafe { c_str_to_str(batch_json) };
    let res = crate::wasm::process_batch_json_internal(json)
        .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
    to_c_string(res)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rover_native_html_to_text(html: *const c_char) -> *mut c_char {
    let h = unsafe { c_str_to_str(html) };
    to_c_string(crate::wasm::html_to_plain_text(h))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rover_native_clean_email_body(text: *const c_char) -> *mut c_char {
    let t = unsafe { c_str_to_str(text) };
    let res = if t.contains('<') && t.contains('>') {
        crate::wasm::html_to_plain_text(t)
    } else {
        t.replace("\r\n", "\n").trim().to_string()
    };
    to_c_string(res)
}
