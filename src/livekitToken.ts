import { invoke } from "@tauri-apps/api/core";

export async function fetchToken(room: string, identity?: string): Promise<string> {
  const token = await invoke<string>("generate_token", { room, identity });
  return token;
}
