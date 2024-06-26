import { api } from "./api";

export interface ScoreCalculator {
  id: string | null;
  name: string;
  description: string;
  script: string;
  default_config: string;
  supports_seasons: boolean;
  supports_competitions: boolean;
  supports_events: boolean;
  score_fields: string | null;
}

export type ScoreFieldType = "Number" | "Time" | "Boolean";

export class ScoreCalculatorsClient {
  static async listScoreCalculators(): Promise<ScoreCalculator[]> {
    const response = await api("score_calculators", "GET");
    return response as ScoreCalculator[];
  }

  static async getScoreCalculator(id: string): Promise<ScoreCalculator> {
    const response = await api(
      `score_calculators/${encodeURIComponent(id)}`,
      "GET"
    );
    return response as ScoreCalculator;
  }

  static async createScoreCalculator(
    score_calculator: ScoreCalculator
  ): Promise<string> {
    const response = await api("score_calculators", "POST", score_calculator);
    return response as string;
  }

  static async updateScoreCalculator(
    id: string,
    score_calculator: ScoreCalculator
  ): Promise<void> {
    await api(
      `score_calculators/${encodeURIComponent(id)}`,
      "PUT",
      score_calculator
    );
  }

  static async deleteScoreCalculator(id: string): Promise<void> {
    await api(`score_calculators/${encodeURIComponent(id)}`, "DELETE");
  }
}
