import { api } from "./api";

export interface Group {
  id: string | null;
  name: string;
  description: string;
  enabled: boolean;
}

export class GroupsClient {
  static async listGroups(): Promise<Group[]> {
    const response = await api("groups", "GET");
    return response as Group[];
  }

  static async getGroup(id: string): Promise<Group> {
    const response = await api(`groups/${encodeURIComponent(id)}`, "GET");
    return response as Group;
  }

  static async createGroup(group: Group): Promise<string> {
    const response = await api("groups", "POST", group);
    return response as string;
  }

  static async updateGroup(id: string, group: Group): Promise<void> {
    await api(`groups/${encodeURIComponent(id)}`, "PUT", group);
  }

  static async deleteGroup(id: string): Promise<void> {
    await api(`groups/${encodeURIComponent(id)}`, "DELETE");
  }
}
