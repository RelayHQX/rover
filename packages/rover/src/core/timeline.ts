import type { ProcessedEmail, TimelineBuildResult } from '../types/index.js';

export const RoverIndex = {
  /**
   * Group processed emails into connected timelines using their deterministic timeline keys.
   */
  buildTimelines(processedEmails: ProcessedEmail[]): TimelineBuildResult {
    const timelines: Record<string, any> = {};
    const unlinked: ProcessedEmail[] = [];

    for (const email of processedEmails) {
      const key = email.timeline_key || (email as any).timelineKey;
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
