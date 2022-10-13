import {init_usdpl, target_usdpl, init_embedded, call_backend} from "usdpl-front";

const USDPL_PORT: number = 25717;

// Utility

export function resolve<T>(promise: Promise<T>, setter: (x: T) => void) {
    (async function () {
        let data = await promise;
        if (data != null) {
            console.debug("Got resolved", data);
            setter(data);
        } else {
            console.warn("Resolve failed:", data);
        }
    })();
}

export async function initBackend() {
    // init usdpl
    await init_embedded();
    init_usdpl(USDPL_PORT);
    console.log("USDPL started for framework: " + target_usdpl());
    //setReady(true);
}

export type CAbout = {
    name: string;
    version: string;
    description: string;
    url: string | null;
    authors: string[];
    license: string | null;
}

export type CButton = {
    element: string; // "button"
    title: string;
}

export type CToggle = {
    element: string; // "toggle"
    title: string;
    description: string | null;
}

export type CSlider = {
    element: string; // "slider"
    title: string;
    min: number;
    max: number;
    notches: string[] | null;
}

export type CReading = {
    element: string; // "reading"
    title: string;
    period_ms: number;
}

export type CResultDisplay = {
    element: string; // "result-display"
    title: string;
    result_of: number;
}

export type CEventDisplay = {
    element: string; // "event-display"
    title: string;
    event: string;
}

export type CSteamEvent = {
    event_type: string; // enum; see steam_types.rs
    event_data: any;
}

export type CElement = CButton | CToggle | CSlider | CReading | CResultDisplay | CEventDisplay;

export type CErrorResult = {
    result: string; // "error"
    message: string;
    exception: string;
}

export type CValueResult = {
    result: string; // "value"
    value: any;
}

export type CDisplayResponse = CValueResult | CErrorResult;

export type CJavascriptResult = {
    result: string; // "javascript"
    id: number;
    raw: string;
}

export type CJavascriptResponse = CJavascriptResult | CErrorResult;

export async function getElements(): Promise<CElement[]> {
    return (await call_backend("get_items", []))[0];
}

export async function onUpdate(index: number, value: any): Promise<boolean> {
    return (await call_backend("on_update", [index, value]))[0];
}

export async function getDisplay(index: number): Promise<CDisplayResponse> {
    return (await call_backend("get_display", [index]))[0];
}

export async function getAbout(): Promise<CAbout> {
    return (await call_backend("get_about", []))[0];
}

export async function reload(): Promise<CElement[]> {
    return (await call_backend("reload", []))[0];
}

export async function getJavascriptToRun(): Promise<CJavascriptResponse> {
    return (await call_backend("get_javascript_to_run", []))[0];
}

export async function onJavascriptResult(id: number, value: any): Promise<boolean> {
    return (await call_backend("on_javascript_result", [id, value]))[0];
}

export async function onSteamEvent(data: CSteamEvent): Promise<boolean> {
    return (await call_backend("on_steam_event", [data]))[0];
}
