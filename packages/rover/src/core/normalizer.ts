import type { EmailRecordInput, NormalizedEmail, FullEmail, InboxMessage } from '../types/index.js';
import { htmlToText } from '../utils/html.js';

/**
 * Extract clean, readable normal message text from an email object or text.
 * Handles massive HTML emails, raw multipart text bodies, and synthetic email formats.
 */
export function cleanEmailBody(email: EmailRecordInput | string): string {
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

  // 2. Fallback to htmlBody if textBody is empty
  if (email.htmlBody && typeof email.htmlBody === 'string') {
    return htmlToText(email.htmlBody);
  }

  return '';
}

/**
 * Normalize an email of any format into a standardized structured object.
 */
export function normalizeEmail(email: EmailRecordInput): NormalizedEmail {
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
 * Converts a FullEmail or InboxMessage into the normalized EmailRecordInput required by Rover.
 */
export function toRoverInput(email: FullEmail | InboxMessage): EmailRecordInput {
  const isFullEmail = 'html' in email || 'text' in email;

  if (isFullEmail) {
    const full = email as FullEmail;
    return {
      id: full.id,
      from: full.from,
      sender: full.from,
      subject: full.subject || '',
      date: full.date || (full.timestamp ? new Date(full.timestamp).toISOString() : ''),
      htmlBody: full.html || '',
      textBody: full.text || '',
      body: full.text || full.html || full.snippet || '',
      received_at: full.internalDate,
    };
  }

  const inbox = email as InboxMessage;
  return {
    id: inbox.id,
    from: inbox.from || inbox.senderEmail || '',
    sender: inbox.senderEmail || inbox.from || '',
    sender_name: inbox.senderName,
    subject: inbox.subject || '',
    date: inbox.date || '',
    body: inbox.snippet || '',
  };
}
