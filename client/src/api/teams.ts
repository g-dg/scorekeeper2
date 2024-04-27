import { api } from "./api";

export interface Team {
  id: string | null;
  group_participation_id: string;
  name: string;
  description: string;
  enabled: boolean;
}

export class TeamsClient {
  static async listTeams(): Promise<Team[]> {
    const response = await api("teams", "GET");
    return response as Team[];
  }

  static async getTeam(id: string): Promise<Team> {
    const response = await api(`teams/${encodeURIComponent(id)}`, "GET");
    return response as Team;
  }

  static async createTeam(team: Team): Promise<string> {
    const response = await api("teams", "POST", team);
    return response as string;
  }

  static async updateTeam(id: string, team: Team): Promise<void> {
    await api(`teams/${encodeURIComponent(id)}`, "PUT", team);
  }

  static async deleteTeam(id: string): Promise<void> {
    await api(`teams/${encodeURIComponent(id)}`, "DELETE");
  }
}
