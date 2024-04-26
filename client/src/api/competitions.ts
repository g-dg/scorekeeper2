import { api } from "./api";

export interface Competition {
  id: string | null;
  name: string;
  description: string;
  enabled: boolean;
}

export class CompetitionsClient {
  static async listCompetitions(): Promise<Competition[]> {
    const response = await api("competitions", "GET");
    return response as Competition[];
  }

  static async getCompetition(id: string): Promise<Competition> {
    const response = await api(`competitions/${encodeURIComponent(id)}`, "GET");
    return response as Competition;
  }

  static async createCompetition(competition: Competition): Promise<string> {
    const response = await api("competitions", "POST", competition);
    return response as string;
  }

  static async updateCompetition(
    id: string,
    competition: Competition
  ): Promise<void> {
    await api(`competitions/${encodeURIComponent(id)}`, "PUT", competition);
  }

  static async deleteCompetition(id: string): Promise<void> {
    await api(`competitions/${encodeURIComponent(id)}`, "DELETE");
  }
}
