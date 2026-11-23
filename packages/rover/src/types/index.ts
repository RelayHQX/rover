import type { FullEmail, InboxMessage } from './mail.js';

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
  | 'Developer'
  | 'General';

export type RoverCategory = Category;

export type GeneralCategory =
  | 'primary'
  | 'social'
  | 'promotions'
  | 'updates'
  | 'forums'
  | 'spam';

export type RoverGeneralCategory = GeneralCategory;

export type Importance = 'urgent' | 'high' | 'normal' | 'low';
export type RoverImportance = Importance;

export type SecurityClassification =
  | 'authentication'
  | 'password_reset'
  | 'security_alert'
  | 'account_change'
  | 'build_failure'
  | 'action_required'
  | 'none';

export type RoverSecurityClassification = SecurityClassification;

export interface ConfidenceFactors {
  authenticated_sender: boolean;
  has_structured_entities: boolean;
  pattern_specificity: string;
  subject_body_aligned: boolean;
}

export interface EmailClassification {
  general_category: RoverGeneralCategory;
  purpose: RoverCategory;
  importance: RoverImportance;
  is_alert: boolean;
  security: RoverSecurityClassification;
  confidence: number;
  factors: ConfidenceFactors;
  secondary_purposes: RoverCategory[];
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

export interface EmailTimelineGroup {
  timelineKey: string;
  primaryCategory: string;
  events: ProcessedEmail[];
}

export interface TimelineBuildResult {
  timelines: Record<string, EmailTimelineGroup>;
  unlinked: ProcessedEmail[];
}

export * from './mail.js';
