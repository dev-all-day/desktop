import { invoke } from "@tauri-apps/api/tauri";
import { TConfig } from "../types/config";

export async function getPreferences() {
    const preferences : TConfig = await invoke("cmd_get_config");
    return preferences; 
}

export async function changeWindowTitle(title:string) {
    await invoke("change_window_title",{ title });
}

export const checkAppUpdate = async () => {
    await invoke('run_check_update', { silent: false, hasMsg: true });
  };


// TODO: make sure to try catch all invocations
// invoke('set_title', { title: 'New Window Title' })
//   .then(() => console.log('Window title updated successfully'))
//   .catch((error) => console.error('Error updating window title', error));