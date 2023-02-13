import { invoke } from "@tauri-apps/api/tauri";
import { TConfig } from "../types/config";

async function getPreferences() {
    const preferences : TConfig = await invoke("cmd_get_config");
    return preferences; 
}

export {getPreferences}