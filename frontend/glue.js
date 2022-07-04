/*
 * JavaScript glue needed to run the Tauri commands.
 * Created on 2022-07-03
 */

/***** Setup *****/
/* Imports */
const invoke = window.__TAURI__.invoke;

export async function invokeReadClipboard() {
    return invoke("read_clipboard", {});
}