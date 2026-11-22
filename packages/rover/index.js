/**
 * @rhq/rover (Relay Rover)
 * High-performance WebAssembly Email Intelligence Engine for React Native and JavaScript.
 * Zero-dependency, in-memory execution powered by compiled Rust WASM.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

let wasmInstance = null;
let wasmExports = null;
let memory = null;

/**
 * Ensure the WASM engine is loaded and initialized.
 * Can be called explicitly with custom bytes or lazy-initializes on first call.
 * 
 * @param {ArrayBuffer|Uint8Array} [customBytes] Optional custom WASM binary buffer
 */
export function init(customBytes) {
  if (wasmExports) {
    return wasmExports;
  }

  let bytes = customBytes;

  if (!bytes) {
    // 1. Try reading from eng/relay_rover.wasm relative to this module
    try {
      const __filename = fileURLToPath(import.meta.url);
      const __dirname = path.dirname(__filename);
      const wasmPath = path.resolve(__dirname, 'eng/relay_rover.wasm');
      if (fs.existsSync(wasmPath)) {
        bytes = fs.readFileSync(wasmPath);
      }
    } catch (_) { }
  }

  if (!bytes) {
    // 2. Try global WASM binary if injected by React Native or bundler
    if (typeof globalThis !== 'undefined' && globalThis.__RELAY_ROVER_WASM__) {
      bytes = globalThis.__RELAY_ROVER_WASM__;
    }
  }

  if (!bytes) {
    throw new Error(
      'Relay Rover WASM engine binary not found. Call RelayRover.init(wasmBytes) or ensure eng/relay_rover.wasm is present.'
    );
  }

  const module = new WebAssembly.Module(bytes);
  wasmInstance = new WebAssembly.Instance(module, {});
  wasmExports = wasmInstance.exports;
  memory = wasmExports.memory;
  return wasmExports;
}

function getExports() {
  if (!wasmExports) {
    init();
  }
  return wasmExports;
}

function writeStringToMemory(exp, mem, str) {
  const encoder = new TextEncoder();
  const encoded = encoder.encode(str || '');
  const ptr = exp.rover_alloc(encoded.length);
  const heap = new Uint8Array(mem.buffer, ptr, encoded.length);
  heap.set(encoded);
  return { ptr, len: encoded.length };
}

function readStringFromMemory(exp, mem, ptr) {
  const view = new DataView(mem.buffer, ptr);
  const len = view.getUint32(0, true);
  const bytes = new Uint8Array(mem.buffer, ptr + 4, len);
  const text = new TextDecoder('utf-8').decode(bytes);
  exp.rover_dealloc(ptr, len + 4);
  return text;
}

/**
 * Convert HTML email content to clean, readable plain text.
 * Strips script tags, style blocks, HTML tags, unescapes entities, and collapses excessive whitespace.
 * 
 * @param {string} html Raw HTML email string
 * @returns {string} Clean plain-text message
 */
export function htmlToText(html) {
  if (!html || typeof html !== 'string') return '';

  // Use fast Rust WASM HTML-to-text if engine is loaded
  try {
    const exp = getExports();
    if (exp && typeof exp.rover_html_to_text === 'function') {
      const input = writeStringToMemory(exp, memory, html);
      const resPtr = exp.rover_html_to_text(input.ptr, input.len);
      const text = readStringFromMemory(exp, memory, resPtr);
      exp.rover_dealloc(input.ptr, input.len);
      return text;
    }
  } catch (_) { }

  // JavaScript fallback
  return html
    .replace(/<style[^>]*>[\s\S]*?<\/style>/gi, ' ')
    .replace(/<script[^>]*>[\s\S]*?<\/script>/gi, ' ')
    .replace(/<head[^>]*>[\s\S]*?<\/head>/gi, ' ')
    .replace(/<\/(p|div|tr|h[1-6]|li|blockquote)>/gi, '\n')
    .replace(/<(br|hr)\s*\/?>/gi, '\n')
    .replace(/<[^>]+>/g, ' ')
    .replace(/&nbsp;/gi, ' ')
    .replace(/&amp;/gi, '&')
    .replace(/&lt;/gi, '<')
    .replace(/&gt;/gi, '>')
    .replace(/&quot;/gi, '"')
    .replace(/&#39;/gi, "'")
    .replace(/&apos;/gi, "'")
    .replace(/[ \t]+/g, ' ')
    .replace(/\r?\n\s*\r?\n+/g, '\n\n')
    .trim();
}

/**
 * Extract clean, readable normal message text from an email object or text.
 * Handles massive HTML emails, raw multipart text bodies, and synthetic email formats.
 * 
 * @param {object|string} email Email record or body string
 * @returns {string} Clean message text
 */
export function cleanEmailBody(email) {
  if (!email) return '';
  if (typeof email === 'string') {
    if (/<[a-z][\s\S]*>/i.test(email)) {
      return htmlToText(email);
    }
    return email.replace(/\r\n/g, '\n').trim();
  }

  // 1. Check textBody / body / content
  const textCandidate = email.textBody || email.body || email.content || '';
  if (typeof textCandidate === 'string' && textCandidate.trim().length > 0) {
    if (/<[a-z][\s\S]*>/i.test(textCandidate)) {
      return htmlToText(textCandidate);
    }
    return textCandidate.replace(/\r\n/g, '\n').trim();
  }

  // 2. Fallback to htmlBody if textBody is empty (common in marketing / newsletters)
  if (email.htmlBody && typeof email.htmlBody === 'string') {
    return htmlToText(email.htmlBody);
  }

  return '';
}

/**
 * Normalize an email of any format (Relay Mail, RFC822-style, or synthetic)
 * into a standardized structured object ready for processing and display.
 * 
 * @param {object} email Raw email object
 * @returns {object} Normalized email object
 */
export function normalizeEmail(email) {
  if (!email || typeof email !== 'object') {
    throw new TypeError('Expected an email object');
  }

  let sender = 'unknown@example.com';
  let senderName = '';

  if (email.from) {
    if (Array.isArray(email.from) && email.from.length > 0) {
      sender = email.from[0].email || sender;
      senderName = email.from[0].name || '';
    } else if (typeof email.from === 'object') {
      sender = email.from.email || sender;
      senderName = email.from.name || '';
    } else if (typeof email.from === 'string') {
      const match = email.from.match(/(.*?)\s*<([^>]+)>/);
      if (match) {
        senderName = match[1].trim();
        sender = match[2].trim();
      } else {
        sender = email.from.trim();
        senderName = '';
      }
    }
  } else {
    if (email.sender) sender = email.sender;
    if (email.sender_name) senderName = email.sender_name;
  }

  const subject = (email.subject || '').replace(/\r?\n/g, ' ').trim();
  const body = cleanEmailBody(email);
  const receivedAt = email.date || email.received_at || new Date().toISOString();
  const id = email.id || `email_${Date.now()}`;

  return {
    id,
    sender,
    senderName,
    subject,
    body,
    receivedAt,
    isHtml: Boolean(!email.textBody && email.htmlBody),
  };
}

/**
 * Generate a concise snippet/summary from an email body for inbox listings.
 * 
 * @param {object|string} email Email record or text
 * @param {number} [maxLength=100] Maximum character length
 * @returns {string} Truncated message summary
 */
export function extractSummary(email, maxLength = 100) {
  const text = typeof email === 'string' ? email : cleanEmailBody(email);
  const clean = text.replace(/\s+/g, ' ').trim();
  if (clean.length <= maxLength) return clean;
  return clean.slice(0, maxLength - 3).trim() + '...';
}

/**
 * Rover Intelligence Index
 * Organizes emails into connected purchase, project, and activity timelines.
 */
export const RoverIndex = {
  /**
   * Group processed emails into connected timelines using their deterministic timeline keys.
   * @param {Array<object>} processedEmails 
   * @returns {{ timelines: Record<string, object>, unlinked: Array<object> }}
   */
  buildTimelines(processedEmails) {
    const timelines = {};
    const unlinked = [];

    for (const email of processedEmails) {
      const key = email.timeline_key || email.timelineKey;
      if (key) {
        if (!timelines[key]) {
          timelines[key] = {
            timelineKey: key,
            primaryCategory: email.primary_category,
            events: [],
          };
        }
        timelines[key].events.push(email);
      } else {
        unlinked.push(email);
      }
    }

    return { timelines, unlinked };
  },
};

export const RelayRover = {
  /**
   * Explicitly initialize the WASM engine with a binary buffer (optional).
   */
  init,

  /**
   * Fast category detection for any arbitrary email subject and body.
   * @param {string} subject 
   * @param {string} body 
   * @returns {string} Category name (e.g. 'Payment', 'OTP', 'Jobs', 'Travel', etc.)
   */
  classify(subject, body) {
    const exp = getExports();
    const sub = writeStringToMemory(exp, memory, subject);
    const bod = writeStringToMemory(exp, memory, body);
    const resPtr = exp.rover_classify(sub.ptr, sub.len, bod.ptr, bod.len);
    const category = readStringFromMemory(exp, memory, resPtr);
    exp.rover_dealloc(sub.ptr, sub.len);
    exp.rover_dealloc(bod.ptr, bod.len);
    return category;
  },

  /**
   * Generate concise contextual preview line (< 100 characters).
   * @param {string} subject 
   * @param {string} body 
   * @returns {string} Preview text
   */
  generatePreview(subject, body) {
    const exp = getExports();
    const sub = writeStringToMemory(exp, memory, subject);
    const bod = writeStringToMemory(exp, memory, body);
    const resPtr = exp.rover_preview(sub.ptr, sub.len, bod.ptr, bod.len);
    const preview = readStringFromMemory(exp, memory, resPtr);
    exp.rover_dealloc(sub.ptr, sub.len);
    exp.rover_dealloc(bod.ptr, bod.len);
    return preview;
  },

  /**
   * Process email with full category detection, typed extraction, and preview generation.
   * @param {string} subject 
   * @param {string} body 
   * @returns {object} ProcessedEmail result
   */
  processEmail(subject, body) {
    const exp = getExports();
    const sub = writeStringToMemory(exp, memory, subject);
    const bod = writeStringToMemory(exp, memory, body);
    const resPtr = exp.rover_process_email(sub.ptr, sub.len, bod.ptr, bod.len);
    const jsonStr = readStringFromMemory(exp, memory, resPtr);
    exp.rover_dealloc(sub.ptr, sub.len);
    exp.rover_dealloc(bod.ptr, bod.len);
    return JSON.parse(jsonStr);
  },

  /**
   * Process a single email from any email object (Relay Mail, HTML, or synthetic) or string.
   * Automatically normalizes headers, cleans HTML if present, and attaches cleanBody.
   * 
   * @param {object|string} input Email object or text
   * @returns {object} ProcessedEmail result with cleanBody
   */
  process(input) {
    if (typeof input === 'string') {
      const clean = cleanEmailBody(input);
      const res = this.processEmail('', clean);
      return { ...res, cleanBody: clean };
    }
    if (input && typeof input === 'object') {
      const normalized = normalizeEmail(input);
      const exp = getExports();
      let res;
      if (exp && typeof exp.rover_process_email_json === 'function') {
        const jsonStr = JSON.stringify(normalized);
        const inMem = writeStringToMemory(exp, memory, jsonStr);
        const resPtr = exp.rover_process_email_json(inMem.ptr, inMem.len);
        const outStr = readStringFromMemory(exp, memory, resPtr);
        exp.rover_dealloc(inMem.ptr, inMem.len);
        res = JSON.parse(outStr);
      } else {
        res = this.processEmail(normalized.subject, normalized.body);
      }
      return {
        ...res,
        id: normalized.id,
        cleanBody: normalized.body,
        sender: normalized.sender,
        senderName: normalized.senderName,
      };
    }
    throw new TypeError('Expected email object or string');
  },

  /**
   * Process multiple emails in a single in-memory WebAssembly engine call.
   * Accepts an array of email objects (including raw Relay Mail with htmlBody) or a serialized JSON string.
   * 
   * @param {Array<object>|string} emails Array of email records
   * @returns {Array<object>} Array of ProcessedEmail results with cleanBody
   */
  processBatch(emails) {
    const exp = getExports();
    const emailsArray = typeof emails === 'string' ? JSON.parse(emails) : emails;

    const normalizedArray = emailsArray.map(m => {
      if (typeof m === 'object' && m !== null) {
        return normalizeEmail(m);
      }
      return m;
    });

    const jsonString = JSON.stringify(normalizedArray);
    const input = writeStringToMemory(exp, memory, jsonString);
    const resPtr = exp.rover_process_batch_json(input.ptr, input.len);
    const resultsJson = readStringFromMemory(exp, memory, resPtr);
    exp.rover_dealloc(input.ptr, input.len);

    const parsed = JSON.parse(resultsJson);
    if (Array.isArray(parsed)) {
      return parsed.map((item, idx) => ({
        ...item,
        cleanBody: normalizedArray[idx]?.body || '',
      }));
    }
    return parsed;
  },

  /**
   * Alias for processBatch to process an array of JSON email records.
   * @param {Array<object>|string} emails 
   * @returns {Array<object>}
   */
  processEmailsJson(emails) {
    return this.processBatch(emails);
  },

  /**
   * Convert HTML email content to clean, readable plain text.
   */
  htmlToText,

  /**
   * Extract clean, readable normal message text from an email object or text.
   */
  cleanEmailBody,

  /**
   * Normalize an email of any format into a standardized structured object.
   */
  normalizeEmail,

  /**
   * Generate a concise summary snippet from an email body.
   */
  extractSummary,

  /**
   * Rover Intelligence Index timeline grouping engine.
   */
  index: RoverIndex,
};

export default RelayRover;
