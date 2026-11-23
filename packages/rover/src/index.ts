import type {
  EmailRecordInput,
  ProcessedEmail,
  FullEmail,
  InboxMessage,
  NormalizedEmail,
  TimelineBuildResult,
} from './types/index.js';
import { htmlToText, cleanBodyText } from './utils/html.js';
import { cleanEmailBody, normalizeEmail, toRoverInput } from './core/normalizer.js';
import { extractSummary } from './core/preview.js';
import { RoverIndex } from './core/timeline.js';
import {
  extractIntelligence,
  processWithRover,
  processBatchWithRover,
  attachRoverToFullEmail,
} from './core/extractor.js';
import { classify, generatePreview } from './core/classifier.js';

// Re-export all types
export * from './types/index.js';

// Re-export all individual utilities and extractors
export {
  htmlToText,
  cleanBodyText,
  cleanEmailBody,
  normalizeEmail,
  toRoverInput,
  extractSummary,
  RoverIndex,
  extractIntelligence,
  processWithRover,
  processBatchWithRover,
  attachRoverToFullEmail,
  classify,
  generatePreview,
};

/**
 * Initialize the Rover engine.
 * Idempotent and safe across React Native, Node, and browser runtimes.
 */
export function init(customBytes?: any): any {
  return {
    status: 'initialized',
    engine: 'typescript',
    version: '0.1.5',
  };
}

/**
 * Process email with full category detection, typed extraction, and preview generation.
 */
export function processEmail(subject: string, body: string): ProcessedEmail {
  const clean = cleanEmailBody(body || '');
  return extractIntelligence({ subject: subject || '', body: clean });
}

/**
 * Process a single email from any email object (Relay Mail, HTML, or synthetic) or string.
 * Automatically normalizes headers, cleans HTML if present, and attaches cleanBody.
 */
export function process(input: EmailRecordInput | string): ProcessedEmail {
  if (typeof input === 'string') {
    const clean = cleanEmailBody(input);
    const res = processEmail('', clean);
    return { ...res, cleanBody: clean };
  }
  if (input && typeof input === 'object') {
    const normalized = normalizeEmail(input);
    const res = extractIntelligence(normalized, input as any);
    return {
      ...res,
      id: normalized.id,
      cleanBody: normalized.body,
      sender: normalized.sender,
      senderName: normalized.senderName,
    };
  }
  throw new TypeError('Expected email object or string');
}

/**
 * Process multiple emails in a single batch call.
 * Accepts an array of email objects or a serialized JSON string.
 */
export function processBatch(emails: EmailRecordInput[] | string): ProcessedEmail[] {
  const emailsArray: EmailRecordInput[] =
    typeof emails === 'string' ? JSON.parse(emails) : emails;

  if (!Array.isArray(emailsArray)) {
    throw new TypeError('Expected an array of emails or JSON string');
  }

  return emailsArray.map(m => {
    if (typeof m === 'object' && m !== null) {
      return process(m);
    }
    return process(m as any);
  });
}

/**
 * Alias for processBatch to process an array of JSON email records.
 */
export function processEmailsJson(emails: EmailRecordInput[] | string): ProcessedEmail[] {
  return processBatch(emails);
}

/**
 * Asynchronously process a single email off the UI thread (React Native / Hermes).
 */
export async function processAsync(input: EmailRecordInput | string): Promise<ProcessedEmail> {
  return Promise.resolve().then(() => process(input));
}

/**
 * Asynchronously process a batch of emails off the UI thread (React Native / Hermes).
 */
export async function processBatchAsync(
  emails: EmailRecordInput[] | string
): Promise<ProcessedEmail[]> {
  return Promise.resolve().then(() => processBatch(emails));
}

/**
 * Relay Rover Engine API
 */
export const RelayRover = {
  init,
  classify,
  generatePreview,
  processEmail,
  process,
  processBatch,
  processEmailsJson,
  processAsync,
  processBatchAsync,
  htmlToText,
  cleanEmailBody,
  normalizeEmail,
  extractSummary,
  index: RoverIndex,
  // User-provided & Relay Mail integration helpers
  toRoverInput,
  cleanBodyText,
  extractIntelligence,
  processWithRover,
  processBatchWithRover,
  attachRoverToFullEmail,
};

export default RelayRover;
