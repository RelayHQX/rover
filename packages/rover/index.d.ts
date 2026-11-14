export type Category =
  | 'Payment'
  | 'Otp'
  | 'Job'
  | 'Shopping'
  | 'Invoice'
  | 'Subscription'
  | 'Travel'
  | 'Newsletter'
  | 'Social'
  | 'SecurityAlert'
  | 'General';

export type GeneralCategory =
  | 'primary'
  | 'social'
  | 'promotions'
  | 'updates'
  | 'forums'
  | 'spam';

export type Importance = 'urgent' | 'high' | 'normal' | 'low';

export type SecurityClassification =
  | 'authentication'
  | 'password_reset'
  | 'security_alert'
  | 'account_change'
  | 'build_failure'
  | 'action_required'
  | 'none';

export interface ConfidenceFactors {
  authenticated_sender: boolean;
  has_structured_entities: boolean;
  pattern_specificity: string;
  subject_body_aligned: boolean;
}

export interface EmailClassification {
  general_category: GeneralCategory;
  purpose: Category;
  importance: Importance;
  is_alert: boolean;
  security: SecurityClassification;
  confidence: number;
  factors: ConfidenceFactors;
  secondary_purposes: Category[];
}

export interface ProcessedEmail<T = any> {
  id: string;
  primary_category: string;
  matched_categories: string[];
  classification?: EmailClassification;
  extraction: {
    category: string;
    data: T;
  };
  preview: string;
  timeline_key?: string;
  cleanBody?: string;
  sender?: string;
  senderName?: string;
}

export interface EmailTimelineGroup {
  timelineKey: string;
  primaryCategory: string;
  events: ProcessedEmail[];
}

export interface TimelineBuildResult {
  timelines: Record<string, EmailTimelineGroup>;
  unlinked: ProcessedEmail[];
}

export declare const RoverIndex: {
  buildTimelines(processedEmails: ProcessedEmail[]): TimelineBuildResult;
};

export interface EmailRecordInput {
  id?: string;
  sender?: string;
  sender_name?: string;
  from?: any;
  subject?: string;
  body?: string;
  textBody?: string;
  htmlBody?: string;
  content?: string;
  date?: string;
  received_at?: string;
  headers?: any;
}

export interface NormalizedEmail {
  id: string;
  sender: string;
  senderName: string;
  subject: string;
  body: string;
  receivedAt: string;
  isHtml: boolean;
}

/**
 * Convert HTML email content to clean, readable plain text.
 * Strips scripts, styles, HTML tags, unescapes entities, and normalizes whitespace.
 */
export declare function htmlToText(html: string): string;

/**
 * Extract clean, readable normal message text from an email object or text string.
 * Handles massive HTML emails, raw multipart text bodies, and synthetic email formats.
 */
export declare function cleanEmailBody(email: EmailRecordInput | string): string;

/**
 * Normalize an email of any format into a standardized structured object.
 */
export declare function normalizeEmail(email: EmailRecordInput): NormalizedEmail;

/**
 * Generate a concise snippet/summary from an email body for inbox listings.
 */
export declare function extractSummary(email: EmailRecordInput | string, maxLength?: number): string;

export declare const RelayRover: {
  /**
   * Explicitly initialize the WebAssembly engine with custom bytes (optional).
   */
  init(customBytes?: ArrayBuffer | Uint8Array): any;

  /**
   * Fast category detection for any arbitrary email subject and body.
   */
  classify(subject: string, body: string): string;

  /**
   * Generate concise contextual preview line (< 100 characters).
   */
  generatePreview(subject: string, body: string): string;

  /**
   * Process email with full category detection, typed extraction, and preview generation.
   */
  processEmail(subject: string, body: string): ProcessedEmail;

  /**
   * Process a single email record object or string (auto-handles HTML and extracts cleanBody).
   */
  process(email: EmailRecordInput | string): ProcessedEmail;

  /**
   * Process multiple emails in a SINGLE in-memory WebAssembly call.
   * Accepts an array of email objects (including raw Relay Mail with htmlBody) or a serialized JSON string.
   */
  processBatch(emails: EmailRecordInput[] | string): ProcessedEmail[];

  /**
   * Alias for processBatch to process an array of JSON email records.
   */
  processEmailsJson(emails: EmailRecordInput[] | string): ProcessedEmail[];

  /**
   * Convert HTML email content to clean, readable plain text.
   */
  htmlToText(html: string): string;

  /**
   * Extract clean, readable normal message text from an email object or text.
   */
  cleanEmailBody(email: EmailRecordInput | string): string;

  /**
   * Normalize an email of any format into a standardized structured object.
   */
  normalizeEmail(email: EmailRecordInput): NormalizedEmail;

  /**
   * Generate a concise summary snippet from an email body.
   */
  extractSummary(email: EmailRecordInput | string, maxLength?: number): string;

  /**
   * Rover Intelligence Index timeline grouping engine.
   */
  index: typeof RoverIndex;
};

export default RelayRover;
