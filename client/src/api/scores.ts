import { api } from "./api";

export type ScoreType = "group" | "team";

export interface Score {
  id: string | null;
  competition_event_id: string;
  score_type: ScoreType;
  subject_id: string;
  score_data: string;
  timestamp: string;
  valid: boolean;
  disqualified: boolean;
  notes: string | null;
}

export class ScoresClient {
  static async listScores(): Promise<Score[]> {
    const response = await api("scores", "GET");
    return response as Score[];
  }

  static async getScore(id: string): Promise<Score> {
    const response = await api(`scores/${encodeURIComponent(id)}`, "GET");
    return response as Score;
  }

  static async createScore(score: Score): Promise<string> {
    const response = await api("scores", "POST", score);
    return response as string;
  }

  static async updateScore(id: string, score: Score): Promise<void> {
    await api(`scores/${encodeURIComponent(id)}`, "PUT", score);
  }

  static async deleteScore(id: string): Promise<void> {
    await api(`scores/${encodeURIComponent(id)}`, "DELETE");
  }
}
