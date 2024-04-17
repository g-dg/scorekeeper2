import { api } from "./api";

export interface User {
  id: string | null;
  username: string;
  new_password: string | null;
  enabled: boolean;
  permissions: number;
  permission_modify_self: boolean;
  permission_user_admin: boolean;
  permission_setup_admin: boolean;
  permission_view_results: boolean;
  permission_view_scores: boolean;
  permission_enter_scores: boolean;
  permission_view_registration: boolean;
  permission_enter_registration: boolean;
}

export class UserPermission {
  static NONE = 0;
  static MODIFY_SELF = 1 << 0;
  static USER_ADMIN = 1 << 1;
  static SETUP_ADMIN = 1 << 2;
  static RESULTS_VIEW = 1 << 3;
  static SCORE_VIEW = 1 << 4;
  static SCORE_ENTRY = 1 << 5;
  static REGISTRATION_VIEW = 1 << 6;
  static REGISTRATION_ENTRY = 1 << 7;
}

export class UserClient {
  static async listUsers(): Promise<User[]> {
    let response = await api("users", "GET");
    return response as User[];
  }

  static async createUser(user: User): Promise<void> {
    await api("users", "POST", user);
  }

  static async getUser(user_id: string): Promise<User> {
    let response = await api(`users/${encodeURIComponent(user_id)}`, "GET");
    return response as User;
  }

  static async updateUser(user_id: string, user: User): Promise<void> {
    await api(`users/${encodeURIComponent(user_id)}`, "PUT", user);
  }

  static async deleteUser(user_id: string): Promise<void> {
    await api(`users/${encodeURIComponent(user_id)}`, "DELETE");
  }

  static async changePassword(
    user_id: string,
    password: string
  ): Promise<void> {
    await api(`users/${encodeURIComponent(user_id)}/password`, "PUT", password);
  }
}
