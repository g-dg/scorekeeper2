export interface User {
  id: string,
  username: string,
  new_password: string | null,
  enabled: boolean,
  permission_user_admin: boolean,
  permission_data_admin: boolean,
  permission_view_results: boolean,
  permission_view_scores: boolean,
  permission_enter_scores: boolean,
  permission_view_registration: boolean,
  permission_enter_registration: boolean,
}
