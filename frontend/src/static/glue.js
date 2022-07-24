/*
 * JavaScript glue needed to run the Tauri commands.
 * Created on 2022-07-03
 */

/***** Setup *****/
/* Imports */
const invoke = window.__TAURI__.invoke;
const dialog = window.__TAURI__.dialog;

/***** Bridge functions *****/
export async function invokeReadClipboard() {
	return invoke("read_clipboard", {});
}

export async function invokeSaveImage(path) {
	return invoke("save_image", {path: path});
}

export async function invokeGetSavePath(format) {
	const format_filter = [{name: format.toUpperCase(), extensions: [format.toLowerCase()]}];
	return dialog.save({
		multiple: false,
		filters: format_filter,
	});
}

export async function invokeReadSettings() {
	return invoke("read_settings", {});
}

export async function invokeSaveSettings(settings) {
	return invoke("save_settings", {settings: settings});
}
