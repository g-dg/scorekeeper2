import { api } from "./api";

export interface SeasonCompetition {
  id: string | null;
  season_id: string;
  competition_id: string;
  description: string;
  score_calculator: String | null;
  enabled: boolean;
}

export class SeasonCompetitionsClient {
  static async listSeasonCompetitions(): Promise<SeasonCompetition[]> {
    const response = await api("season_competitions", "GET");
    return response as SeasonCompetition[];
  }

  static async getSeasonCompetition(id: string): Promise<SeasonCompetition> {
    const response = await api(
      `season_competitions/${encodeURIComponent(id)}`,
      "GET"
    );
    return response as SeasonCompetition;
  }

  static async createSeasonCompetition(
    season_competition: SeasonCompetition
  ): Promise<string> {
    const response = await api(
      "season_competitions",
      "POST",
      season_competition
    );
    return response as string;
  }

  static async updateSeasonCompetition(
    id: string,
    season_competition: SeasonCompetition
  ): Promise<void> {
    await api(
      `season_competitions/${encodeURIComponent(id)}`,
      "PUT",
      season_competition
    );
  }

  static async deleteSeasonCompetition(id: string): Promise<void> {
    await api(`season_competitions/${encodeURIComponent(id)}`, "DELETE");
  }
}
