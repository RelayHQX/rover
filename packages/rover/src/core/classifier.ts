import { extractIntelligence } from './extractor.js';
import { cleanEmailBody } from './normalizer.js';

/**
 * Fast category detection for any arbitrary email subject and body.
 */
export function classify(subject: string, body: string): string {
  if (!subject && !body) return 'General';
  const clean = cleanEmailBody(body);
  const result = extractIntelligence({ subject, body: clean });
  return result.extraction?.category || result.classification?.purpose || 'General';
}

/**
 * Generate concise contextual preview line (< 100 characters).
 */
export function generatePreview(subject: string, body: string): string {
  if (!subject && !body) return '';
  const clean = cleanEmailBody(body);
  const result = extractIntelligence({ subject, body: clean });
  return result.preview || '';
}
