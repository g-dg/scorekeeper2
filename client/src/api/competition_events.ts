import { api } from "./api";
import type { ScoreType } from "./scores";

export interface CompetitionEvent {
  id: string | null;
  season_competition_id: string;
  event_id: string;
  description: string;
  score_calculator: string | null;
  enabled: boolean;
  score_type: ScoreType;
  calculator_config: string;
}

export class CompetitionEventsClient {
  static async listCompetitionEvents(): Promise<CompetitionEvent[]> {
    const response = await api("competition_events", "GET");
    return response as CompetitionEvent[];
  }

  static async getCompetitionEvent(id: string): Promise<CompetitionEvent> {
    const response = await api(
      `competition_events/${encodeURIComponent(id)}`,
      "GET"
    );
    return response as CompetitionEvent;
  }

  static async createCompetitionEvent(
    competition_event: CompetitionEvent
  ): Promise<string> {
    const response = await api("competition_events", "POST", competition_event);
    return response as string;
  }

  static async updateCompetitionEvent(
    id: string,
    competition_event: CompetitionEvent
  ): Promise<void> {
    await api(
      `competition_events/${encodeURIComponent(id)}`,
      "PUT",
      competition_event
    );
  }

  static async deleteCompetitionEvent(id: string): Promise<void> {
    await api(`competition_events/${encodeURIComponent(id)}`, "DELETE");
  }
}
