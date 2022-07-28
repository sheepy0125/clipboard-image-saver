/*
 * JavaScript glue needed to run the Tauri commands.
 * Created on 2022-07-03
 */

/***** Setup *****/
/* Imports */
const invoke = window.__TAURI__.invoke;

/***** Bridge functions *****/
export async function invokeReadClipboard() {
	return invoke("read_clipboard", {});
}

export async function invokeSaveImage(path, format) {
	return invoke("save_image", {path: path, format: format});
}

export async function invokeGetSavePath(format) {
	return invoke("get_save_path", {format: format});
}

export async function invokeReadSettings() {
	return invoke("read_settings", {});
}

export async function invokeSaveSettings(settings) {
	return invoke("save_settings", {settings: settings});
}
