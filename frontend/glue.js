/*
 * JavaScript glue needed to run the Tauri commands.
 * Created on 2022-07-03
 */

/***** Setup *****/
/* Imports */
const invoke = window.__TAURI__.invoke;
const dialog = window.__TAURI__.dialog;

export async function invokeReadClipboard() {
	return invoke("read_clipboard", {});
}

export async function invokeSaveImage(path) {
	return invoke("save_image", {path: path});
}

export async function getSavePath() {
	return dialog.save({
		multiple: false,
		filters: [
			{
				name: "Image",
				extensions: ["png"],
			},
		],
	});
}
