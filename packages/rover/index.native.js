/**
 * @rhq/rover (Relay Rover)
 * First-Class React Native & Expo implementation powered by Rust Core + UniFFI + JSI.
 * Zero-dependency, native-speed in-memory execution for Hermes without Node.js built-ins.
 */

let nativeEngine = null;

/**
 * Discover and acquire the native Rover engine in React Native or Expo runtime.
 * Order of discovery:
 * 1. Direct JSI global host object (installed synchronously into Hermes runtime)
 * 2. Expo Modules API (RelayRover module)
 * 3. React Native NativeModules bridge
 * 
 * @returns {object|null} Native engine binding
 */
export function getNativeEngine() {
  if (nativeEngine) {
    return nativeEngine;
  }

  // 1. Direct JSI global installed by RoverHostObject (highest performance, zero bridge overhead)
  if (typeof globalThis !== 'undefined' && globalThis.__RELAY_ROVER_NATIVE__) {
    nativeEngine = globalThis.__RELAY_ROVER_NATIVE__;
    return nativeEngine;
  }

  // 2. Expo Modules API: globalThis.expo.modules.RelayRover or dynamic requireNativeModule
  if (typeof globalThis !== 'undefined' && globalThis.expo?.modules?.RelayRover) {
    nativeEngine = globalThis.expo.modules.RelayRover;
    return nativeEngine;
  }

  try {
    // Dynamically resolve Expo Modules if running in Expo Go or custom dev client
    const { requireNativeModule } = require('expo-modules-core');
    if (typeof requireNativeModule === 'function') {
      const expoModule = requireNativeModule('RelayRover');
      if (expoModule) {
        nativeEngine = expoModule;
        return nativeEngine;
      }
    }
  } catch (_) { }

  // 3. React Native NativeModules (vanilla React Native bridge)
  try {
    const { NativeModules } = require('react-native');
    const rnModule = NativeModules?.RelayRover || NativeModules?.Rover;
    if (rnModule) {
      if (typeof rnModule.installJsi === 'function') {
        rnModule.installJsi();
        if (typeof globalThis !== 'undefined' && globalThis.__RELAY_ROVER_NATIVE__) {
          nativeEngine = globalThis.__RELAY_ROVER_NATIVE__;
          return nativeEngine;
        }
      }
      nativeEngine = rnModule;
      return nativeEngine;
    }
  } catch (_) { }

  return null;
}

/**
 * Explicitly initialize or inject the native Rover engine.
 * Maintains complete signature parity with RelayRover.init().
 * 
 * @param {any} [customEngineOrBytes] Optional injected engine or binary buffer
 * @returns {object} Native engine reference
 */
export function init(customEngineOrBytes) {
  if (customEngineOrBytes && typeof customEngineOrBytes === 'object' && !customEngineOrBytes.byteLength) {
    nativeEngine = customEngineOrBytes;
    return nativeEngine;
  }

  const engine = getNativeEngine();
  if (engine) {
    return engine;
  }

  return {
    status: 'initialized',
    platform: 'react-native-hermes',
  };
}

/**
 * Convert HTML email content to clean, readable plain text.
 * Strips script tags, style blocks, HTML tags, unescapes entities, and normalizes whitespace.
 * 
 * @param {string} html Raw HTML email string
 * @returns {string} Clean plain-text message
 */
export function htmlToText(html) {
  if (!html || typeof html !== 'string') return '';

  const engine = getNativeEngine();
  if (engine && typeof engine.htmlToText === 'function') {
    try {
      return engine.htmlToText(html);
    } catch (_) { }
  }

  // Hermes-safe regex fallback
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

  const textCandidate = email.textBody || email.body || email.content || '';
  if (typeof textCandidate === 'string' && textCandidate.trim().length > 0) {
    if (/<[a-z][\s\S]*>/i.test(textCandidate)) {
      return htmlToText(textCandidate);
    }
    return textCandidate.replace(/\r\n/g, '\n').trim();
  }

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
   * Explicitly initialize the native engine (optional).
   */
  init,

  /**
   * Fast category detection for any arbitrary email subject and body.
   * @param {string} subject 
   * @param {string} body 
   * @returns {string} Category name (e.g. 'Payment', 'OTP', 'Jobs', 'Travel', etc.)
   */
  classify(subject, body) {
    const engine = getNativeEngine();
    if (engine && typeof engine.classify === 'function') {
      return engine.classify(subject || '', body || '');
    }
    return 'General';
  },

  /**
   * Generate concise contextual preview line (< 100 characters).
   * @param {string} subject 
   * @param {string} body 
   * @returns {string} Preview text
   */
  generatePreview(subject, body) {
    const engine = getNativeEngine();
    if (engine && typeof engine.generatePreview === 'function') {
      return engine.generatePreview(subject || '', body || '');
    }
    return '';
  },

  /**
   * Process email with full category detection, typed extraction, and preview generation.
   * @param {string} subject 
   * @param {string} body 
   * @returns {object} ProcessedEmail result
   */
  processEmail(subject, body) {
    const engine = getNativeEngine();
    if (engine && typeof engine.processEmail === 'function') {
      const res = engine.processEmail(subject || '', body || '');
      return typeof res === 'string' ? JSON.parse(res) : res;
    }
    throw new Error('Relay Rover native engine not available in this environment.');
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
      const engine = getNativeEngine();
      let res;
      if (engine && typeof engine.processEmailJson === 'function') {
        const jsonStr = JSON.stringify(normalized);
        const out = engine.processEmailJson(jsonStr);
        res = typeof out === 'string' ? JSON.parse(out) : out;
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
   * Process multiple emails in a single in-memory native Rust engine call.
   * Accepts an array of email objects (including raw Relay Mail with htmlBody) or a serialized JSON string.
   * 
   * @param {Array<object>|string} emails Array of email records
   * @returns {Array<object>} Array of ProcessedEmail results with cleanBody
   */
  processBatch(emails) {
    const engine = getNativeEngine();
    const emailsArray = typeof emails === 'string' ? JSON.parse(emails) : emails;

    const normalizedArray = emailsArray.map(m => {
      if (typeof m === 'object' && m !== null) {
        return normalizeEmail(m);
      }
      return m;
    });

    let parsed;
    if (engine && typeof engine.processBatchJson === 'function') {
      const jsonString = JSON.stringify(normalizedArray);
      const out = engine.processBatchJson(jsonString);
      parsed = typeof out === 'string' ? JSON.parse(out) : out;
    } else {
      parsed = normalizedArray.map(m => this.process(m));
    }

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
   * Asynchronously process a single email off the UI thread.
   * @param {object|string} input Email object or text
   * @returns {Promise<object>} ProcessedEmail result
   */
  async processAsync(input) {
    const engine = getNativeEngine();
    if (typeof input === 'string') {
      const clean = cleanEmailBody(input);
      if (engine && typeof engine.processEmailAsync === 'function') {
        const out = await engine.processEmailAsync(JSON.stringify({ subject: '', body: clean }));
        const res = typeof out === 'string' ? JSON.parse(out) : out;
        return { ...res, cleanBody: clean };
      }
      return this.process(input);
    }
    if (input && typeof input === 'object') {
      const normalized = normalizeEmail(input);
      if (engine && typeof engine.processEmailAsync === 'function') {
        const jsonStr = JSON.stringify(normalized);
        const out = await engine.processEmailAsync(jsonStr);
        const res = typeof out === 'string' ? JSON.parse(out) : out;
        return {
          ...res,
          id: normalized.id,
          cleanBody: normalized.body,
          sender: normalized.sender,
          senderName: normalized.senderName,
        };
      }
      return this.process(input);
    }
    throw new TypeError('Expected email object or string');
  },

  /**
   * Asynchronously process a batch of emails off the UI thread without blocking Hermes.
   * @param {Array<object>|string} emails Array of email records
   * @returns {Promise<Array<object>>}
   */
  async processBatchAsync(emails) {
    const engine = getNativeEngine();
    const emailsArray = typeof emails === 'string' ? JSON.parse(emails) : emails;
    const normalizedArray = emailsArray.map(m => {
      if (typeof m === 'object' && m !== null) {
        return normalizeEmail(m);
      }
      return m;
    });

    if (engine && typeof engine.processBatchAsync === 'function') {
      const jsonString = JSON.stringify(normalizedArray);
      const out = await engine.processBatchAsync(jsonString);
      const parsed = typeof out === 'string' ? JSON.parse(out) : out;
      if (Array.isArray(parsed)) {
        return parsed.map((item, idx) => ({
          ...item,
          cleanBody: normalizedArray[idx]?.body || '',
        }));
      }
      return parsed;
    }

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
