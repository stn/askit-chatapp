import { invoke } from "@tauri-apps/api/core";

export async function loadSettingsCmd(): Promise<void> {
  await invoke("load_settings_cmd");
}
