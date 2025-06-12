import { load } from '@tauri-apps/plugin-store';
import type { Store } from '@tauri-apps/plugin-store';
import { controllerState } from "$lib/components/controller/controller.svelte";

let store: Store;

(async () => {
    store = await load('__grid_clicker_internal.json', { autoSave: true });

    let lastControllerState = await storeGet('controllerState', controllerState);
    if (lastControllerState) {
        for (const key in lastControllerState) {
            // @ts-ignore
            if (lastControllerState[key] !== undefined) {
                // @ts-ignore
                controllerState[key] = lastControllerState[key];
            }
        }
        controllerState.cursorClickingInProgress = false;
    }
})();

export function storeGet<T>(key: string, defaultValue: T): T {
    const value = store.get(key);
    if (value === undefined) {
        return defaultValue;
    }
    return value as T;
}

export function storeSet<T>(key: string, value: T) {
    store.set(key, value);
}