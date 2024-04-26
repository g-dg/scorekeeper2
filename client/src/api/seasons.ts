import { api } from "./api";

export interface Season {
  id: string | null;
  name: string;
  description: string;
  score_calculator: string | null;
  enabled: boolean;
}

export class SeasonsClient {
  static async listSeasons(): Promise<Season[]> {
    const response = await api("seasons", "GET");
    return response as Season[];
  }

  static async getSeason(id: string): Promise<Season> {
    const response = await api(`seasons/${encodeURIComponent(id)}`, "GET");
    return response as Season;
  }

  static async createSeason(season: Season): Promise<string> {
    const response = await api("seasons", "POST", season);
    return response as string;
  }

  static async updateSeason(id: string, season: Season): Promise<void> {
    await api(`seasons/${encodeURIComponent(id)}`, "PUT", season);
  }

  static async deleteSeason(id: string): Promise<void> {
    await api(`seasons/${encodeURIComponent(id)}`, "DELETE");
  }
}
