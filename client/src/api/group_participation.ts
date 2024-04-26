import { api } from "./api";

export interface GroupParticipation {
  id: string | null;
  group_id: string;
  season_id: string;
  description: string;
  enabled: boolean;
}

export class GroupParticipationsClient {
  static async listGroupParticipations(): Promise<GroupParticipation[]> {
    const response = await api("group_participations", "GET");
    return response as GroupParticipation[];
  }

  static async getGroupParticipation(id: string): Promise<GroupParticipation> {
    const response = await api(
      `group_participations/${encodeURIComponent(id)}`,
      "GET"
    );
    return response as GroupParticipation;
  }

  static async createGroupParticipation(
    group_participation: GroupParticipation
  ): Promise<string> {
    const response = await api(
      "group_participations",
      "POST",
      group_participation
    );
    return response as string;
  }

  static async updateGroupParticipation(
    id: string,
    group_participation: GroupParticipation
  ): Promise<void> {
    await api(
      `group_participations/${encodeURIComponent(id)}`,
      "PUT",
      group_participation
    );
  }

  static async deleteGroupParticipation(id: string): Promise<void> {
    await api(`group_participations/${encodeURIComponent(id)}`, "DELETE");
  }
}
