import Database from "@tauri-apps/plugin-sql";
import type { KeyboardConfig } from "~/composables/useKeyboard";

export interface Profile {
  id: string;
  name: string;
  keyboard_id: {
    vid: number;
    pid: number;
  };
  config: KeyboardConfig;
  updated_at?: number;
}

let db: Database | null = null;

async function getDb(): Promise<Database> {
  if (!db) {
    db = await Database.load("sqlite:rk_configurator.db");
  }
  return db;
}

export const useDatabase = () => {
  const initDatabase = async (): Promise<void> => {
    // Migrations are handled automatically by the Tauri plugin
    // Just ensure database connection is established
    await getDb();
  };

  const saveKeyboardConfig = async (
    keyboardVid: number,
    keyboardPid: number,
    config: KeyboardConfig
  ): Promise<void> => {
    const database = await getDb();
    const configJson = JSON.stringify(config);
    const now = Math.floor(Date.now() / 1000);

    // Preserve selected_profile_id when updating config
    const existing = await database.select<
      Array<{ selected_profile_id: string | null }>
    >(
      "SELECT selected_profile_id FROM keyboard_configs WHERE keyboard_vid = $1 AND keyboard_pid = $2",
      [keyboardVid, keyboardPid]
    );

    const selectedProfileId =
      existing.length > 0 ? existing[0]?.selected_profile_id || null : null;

    await database.execute(
      "INSERT OR REPLACE INTO keyboard_configs (keyboard_vid, keyboard_pid, config_json, selected_profile_id, updated_at) VALUES ($1, $2, $3, $4, $5)",
      [keyboardVid, keyboardPid, configJson, selectedProfileId, now]
    );
  };

  const getKeyboardConfig = async (
    keyboardVid: number,
    keyboardPid: number
  ): Promise<KeyboardConfig | null> => {
    const database = await getDb();
    const result = await database.select<Array<{ config_json: string }>>(
      "SELECT config_json FROM keyboard_configs WHERE keyboard_vid = $1 AND keyboard_pid = $2",
      [keyboardVid, keyboardPid]
    );

    if (result.length > 0) {
      return JSON.parse(result[0]?.config_json ?? "{}") as KeyboardConfig;
    }
    return null;
  };

  const getProfiles = async (
    keyboardVid: number,
    keyboardPid: number
  ): Promise<Profile[]> => {
    const database = await getDb();
    const result = await database.select<
      Array<{
        id: string;
        name: string;
        keyboard_vid: number;
        keyboard_pid: number;
        config_json: string;
        created_at: number;
        updated_at: number;
      }>
    >(
      "SELECT id, name, keyboard_vid, keyboard_pid, config_json, created_at, updated_at FROM profiles WHERE keyboard_vid = $1 AND keyboard_pid = $2 ORDER BY updated_at DESC",
      [keyboardVid, keyboardPid]
    );

    return result.map((row) => ({
      id: row.id,
      name: row.name,
      keyboard_id: {
        vid: row.keyboard_vid,
        pid: row.keyboard_pid,
      },
      config: JSON.parse(row.config_json) as KeyboardConfig,
      updated_at: row.updated_at,
    }));
  };

  const saveProfile = async (profile: Profile): Promise<void> => {
    const database = await getDb();
    const configJson = JSON.stringify(profile.config);
    const now = Math.floor(Date.now() / 1000);

    // Check if profile exists to preserve created_at
    const existing = await database.select<Array<{ created_at: number }>>(
      "SELECT created_at FROM profiles WHERE id = $1",
      [profile.id]
    );

    const createdAt =
      existing.length > 0 ? existing[0]?.created_at ?? now : now;

    await database.execute(
      "INSERT OR REPLACE INTO profiles (id, name, keyboard_vid, keyboard_pid, config_json, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
      [
        profile.id,
        profile.name,
        profile.keyboard_id.vid,
        profile.keyboard_id.pid,
        configJson,
        createdAt,
        now,
      ]
    );
  };

  const deleteProfile = async (profileId: string): Promise<void> => {
    const database = await getDb();
    await database.execute("DELETE FROM profiles WHERE id = $1", [profileId]);
  };

  const saveSelectedProfile = async (
    keyboardVid: number,
    keyboardPid: number,
    profileId: string | null
  ): Promise<void> => {
    const database = await getDb();
    // Update the selected_profile_id in keyboard_configs
    // If the row doesn't exist, create it with empty config
    const existing = await database.select<Array<{ config_json: string }>>(
      "SELECT config_json FROM keyboard_configs WHERE keyboard_vid = $1 AND keyboard_pid = $2",
      [keyboardVid, keyboardPid]
    );

    if (existing.length > 0) {
      // Update existing row
      await database.execute(
        "UPDATE keyboard_configs SET selected_profile_id = $1 WHERE keyboard_vid = $2 AND keyboard_pid = $3",
        [profileId || null, keyboardVid, keyboardPid]
      );
    } else {
      // Create new row with empty config
      const now = Math.floor(Date.now() / 1000);
      await database.execute(
        "INSERT INTO keyboard_configs (keyboard_vid, keyboard_pid, config_json, selected_profile_id, updated_at) VALUES ($1, $2, $3, $4, $5)",
        [keyboardVid, keyboardPid, "{}", profileId || null, now]
      );
    }
  };

  const getSelectedProfile = async (
    keyboardVid: number,
    keyboardPid: number
  ): Promise<string | null> => {
    const database = await getDb();
    const result = await database.select<
      Array<{ selected_profile_id: string | null }>
    >(
      "SELECT selected_profile_id FROM keyboard_configs WHERE keyboard_vid = $1 AND keyboard_pid = $2",
      [keyboardVid, keyboardPid]
    );

    if (result.length > 0 && result[0]?.selected_profile_id) {
      return result[0].selected_profile_id;
    }
    return null;
  };

  return {
    initDatabase,
    saveKeyboardConfig,
    getKeyboardConfig,
    getProfiles,
    saveProfile,
    deleteProfile,
    saveSelectedProfile,
    getSelectedProfile,
  };
};
