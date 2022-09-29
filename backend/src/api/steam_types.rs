#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};

// list of these is second callback param for SteamClient.Downloads.RegisterForDownloadItems
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamDownloadInfo {
    pub active: bool,
    pub appid: usize,
    pub buildid: usize,
    pub target_buildid: usize,
    pub paused: bool,
}

// only callback param for SteamClient.Downloads.RegisterForDownloadOverview
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamDownloadOverview {
    pub paused: bool,
    pub throttling_suspended: bool,
    pub update_appid: usize,
    pub update_bytes_downloaded: usize,
    pub update_bytes_processed: usize,
    pub update_bytes_staged: usize,
    pub update_bytes_to_download: usize,
    pub update_bytes_to_process: usize,
    pub update_bytes_to_stage: usize,
    pub update_disc_bytes_per_second: usize,
    pub update_is_install: bool,
    pub update_is_prefetch_estimate: bool,
    pub update_is_shader: bool,
    pub update_is_workshop: bool,
    pub update_network_bytes_per_second: usize,
    pub update_peak_network_bytes_per_second: usize,
    pub update_seconds_remaining: isize, // -1 seems to indicate not-applicable
    pub update_start_time: usize,
    pub update_state: String,
}

// only param of callback for SteamClient.GameSessions.RegisterForAchievementNotification
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamAchievementNotification {
    pub achievement: SteamAchievement,
    pub nCurrentProgress: usize,
    pub nMaxProgress: usize,
    pub unAppID: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SteamAchievement {
    pub bAchieved: bool,
    pub bHidden: bool,
    pub flAchieved: f64,
    pub flCurrentProgress: f64,
    pub flMaxProgress: f64,
    pub flMinProgress: f64,
    pub rtUnlocked: usize, // time since unix epoch
    pub strDescription: String,
    pub strID: String,
    pub strImage: String,
    pub strName: String,
}

// only param of callback for SteamClient.System.Bluetooth.RegisterForStateChanges
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamBluetoothState {
    pub bEnabled: bool,
    pub vecAdapters: Vec<SteamBluetoothAdapter>,
    pub vecDevices: Vec<SteamBluetoothDevice>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SteamBluetoothDevice {
    pub bConnected: bool,
    pub bPaired: bool,
    pub eType: usize, // enum, idk the options
    pub nAdapterId: usize, // corresponds to SteamBluetoothAdapter.nId
    pub nStrengthRaw: usize, // units???
    pub sMAC: String,
    pub sName: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SteamBluetoothAdapter {
    pub bDiscovering: bool,
    pub bEnabled: bool,
    pub nId: usize,
    pub sMAC: String,
    pub sName: String,
}

// only param of callback for SteamClient.System.Network.RegisterForConnectivityTestChanges
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamConnectivityTestChange {
    pub bChecking: bool,
    pub eConnectivityTestResult: usize, // enum, idk the options
    pub eFakeState: usize, // enum, idk the options
}

// only param of callback for SteamClient.System.Network.RegisterForNetworkDiagnostics
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamNetworkDiagnostic {
    pub status: bool,
    pub total_bytes: usize,
}

// only param of callback for SteamClient.System.Audio.RegisterForDeviceAdded
// and SteamClient.System.Audio.RegisterForDeviceAdded
// Also type of vecDevices of await SteamClient.System.Audio.GetDevices()
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamAudioDevice {
    pub bHasInput: bool,
    pub bHasOutput: bool,
    pub bIsDefaultInputDevice: bool,
    pub bIsDefaultOutputDevice: bool,
    pub flInputVolume: f64,
    pub flOutputVolume: f64,
    pub id: usize,
    pub sName: String,
}

// only param of callback for SteamClient.System.Display.RegisterForBrightnessChanges
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamBrightness {
    pub flBrightness: f64,
}

// not a callback; await SteamClient.System.GetSystemInfo()
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamSystemInfo {
    pub nCPUHz: usize,
    pub nCPULogicalCores: usize,
    pub nCPUPhysicalCores: usize,
    pub nSteamVersion: usize,
    pub nSystemRAMSizeMB: usize,
    pub nVideoRAMSizeMB: usize,
    pub sBIOSVersion: String,
    pub sCPUName: String,
    pub sCPUVendor: String,
    pub sHostname: String,
    pub sKernelVersion: String,
    pub sOSBuildId: String,
    pub sOSCodename: String,
    pub sOSName: String,
    pub sOSVariantId: String,
    pub sOSVersionId: String,
    pub sSteamAPI: String,
    pub sSteamBuildDate: String,
    pub sVideoCardName: String,
    pub sVideoDriverVersion: String,
}

// only param of callback for SteamClient.System.RegisterForAirplaneModeChanges
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamAirplane {
    pub bEnabled: bool,
}

// only param of callback for SteamClient.System.RegisterForBatteryStateChanges
// periodic
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamBattery {
    pub bHasBattery: bool,
    pub bShutdownRequested: bool,
    pub eACState: usize,
    pub eBatteryState: usize,
    pub flLevel: f64,
    pub nSecondsRemaining: usize,
}

// only param of callback for SteamClient.GameSessions.RegisterForScreenshotNotification
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamScreenshotNotification {
    pub details: SteamScreenshot,
    pub hScreenshot: usize,
    pub strOperation: String,
    pub unAppID: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SteamScreenshot {
    pub bSpoilers: bool,
    pub bUploaded: bool,
    pub ePrivacy: usize, // enum
    pub hHandle: usize,
    pub nAppID: usize,
    pub nCreated: usize,
    pub nHeight: usize,
    pub nWidth: usize,
    pub strCaption: String,
    pub strUrl: String,
}

// list of these is only param of callback for SteamClient.Input.RegisterForControllerInputMessages
#[derive(Serialize, Deserialize, Clone)]
pub struct SteamControllerInputMessage {
    pub bState: bool,
    pub nController: usize,
    pub strActionName: String,
}
