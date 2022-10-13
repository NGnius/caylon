import * as backend from "./backend";

type Unregisterer = {
    unregister: () => void;
}

let callbacks: Unregisterer[] = [];

export function register_for_steam_events() {
    unregister_for_steam_events();

    //@ts-ignore
    SteamClient.Apps.RegisterForGameActionStart((p0, p1, p2) => {
       backend.onSteamEvent( {
           event_type: "game-action-start",
           event_data: {
               param0: p0,
               gameID: p1,
               action: p2,
            }
        });
    });

    //@ts-ignore
    SteamClient.GameSessions.RegisterForAppLifetimeNotifications((p0) => {
        backend.onSteamEvent( {
           event_type: "app-lifetime-notification",
           event_data: p0
        });
    });

    //@ts-ignore
    SteamClient.GameSessions.RegisterForAchievementNotification((p0) => {
        backend.onSteamEvent( {
           event_type: "achievement-notification",
           event_data: p0
        });
    });

    //@ts-ignore
    SteamClient.System.Bluetooth.RegisterForStateChanges((p0) => {
        backend.onSteamEvent( {
           event_type: "bluetooth-state",
           event_data: p0
        });
    });

    //@ts-ignore
    SteamClient.System.Display.RegisterForBrightnessChanges((p0) => {
        backend.onSteamEvent( {
           event_type: "brightness",
           event_data: p0
        });
    });

    //@ts-ignore
    SteamClient.System.RegisterForAirplaneModeChanges((p0) => {
        backend.onSteamEvent( {
           event_type: "airplane",
           event_data: p0
        });
    });

    //@ts-ignore
    SteamClient.GameSessions.RegisterForScreenshotNotification((p0) => {
        backend.onSteamEvent( {
           event_type: "screenshot-notification",
           event_data: p0
        });
    });

    // TODO add more events
}

export function unregister_for_steam_events() {
    for (let i = 0; i < callbacks.length; i++) {
        callbacks[i].unregister();
    }
    callbacks = [];
}
