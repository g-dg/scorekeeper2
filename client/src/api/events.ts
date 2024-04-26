import { api } from "./api";

export interface Event {
  id: string | null;
  competition_id: string;
  name: string;
  description: string;
  enabled: boolean;
}

export class EventsClient {
  static async listEvents(): Promise<Event[]> {
    const response = await api("events", "GET");
    return response as Event[];
  }

  static async getEvent(id: string): Promise<Event> {
    const response = await api(`events/${encodeURIComponent(id)}`, "GET");
    return response as Event;
  }

  static async createEvent(event: Event): Promise<string> {
    const response = await api("events", "POST", event);
    return response as string;
  }

  static async updateEvent(id: string, event: Event): Promise<void> {
    await api(`events/${encodeURIComponent(id)}`, "PUT", event);
  }

  static async deleteEvent(id: string): Promise<void> {
    await api(`events/${encodeURIComponent(id)}`, "DELETE");
  }
}
