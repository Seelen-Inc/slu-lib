use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use ts_rs::TS;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothDevice {
    pub id: String,
    pub name: String,
    pub address: u64,
    pub major_class: BluetoothMajor,
    pub minor_main_class: BluetoothMinor,
    pub minor_sub_class: BluetoothMinor,
    pub connected: bool,
    pub paired: bool,
    pub can_pair: bool,
    pub is_bluetooth_loweenergy: bool,
    pub icon_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum BluetoothMajor {
    Miscellaneous,
    Computer,
    Phone,
    NetworkAccessPoint,
    AudioVideo,
    Peripheral,
    Imaging,
    Wearable,
    Toy,
    Health,
    Unkown,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum BluetoothMinor {
    Uncategorized,
    ComputerDesktop,
    ComputerServer,
    ComputerLaptop,
    ComputerHandheld,
    ComputerPalmSize,
    ComputerWearable,
    ComputerTablet,
    PhoneCellular,
    PhoneCordless,
    PhoneSmartPhone,
    PhoneWired,
    PhoneIsdn,
    NetworkFullyAvailable,
    NetworkUsed01To17Percent,
    NetworkUsed17To33Percent,
    NetworkUsed33To50Percent,
    NetworkUsed50To67Percent,
    NetworkUsed67To83Percent,
    NetworkUsed83To99Percent,
    NetworkNoServiceAvailable,
    AudioVideoWearableHeadset,
    AudioVideoHandsFree,
    AudioVideoMicrophone,
    AudioVideoLoudspeaker,
    AudioVideoHeadphones,
    AudioVideoPortableAudio,
    AudioVideoCarAudio,
    AudioVideoSetTopBox,
    AudioVideoHifiAudioDevice,
    AudioVideoVcr,
    AudioVideoVideoCamera,
    AudioVideoCamcorder,
    AudioVideoVideoMonitor,
    AudioVideoVideoDisplayAndLoudspeaker,
    AudioVideoVideoConferencing,
    AudioVideoGamingOrToy,
    PeripheralJoystick,
    PeripheralGamepad,
    PeripheralRemoteControl,
    PeripheralSensing,
    PeripheralDigitizerTablet,
    PeripheralCardReader,
    PeripheralDigitalPen,
    PeripheralHandheldScanner,
    PeripheralHandheldGesture,
    WearableWristwatch,
    WearablePager,
    WearableJacket,
    WearableHelmet,
    WearableGlasses,
    ToyRobot,
    ToyVehicle,
    ToyDoll,
    ToyController,
    ToyGame,
    HealthBloodPressureMonitor,
    HealthThermometer,
    HealthWeighingScale,
    HealthGlucoseMeter,
    HealthPulseOximeter,
    HealthHeartRateMonitor,
    HealthHealthDataDisplay,
    HealthStepCounter,
    HealthBodyCompositionAnalyzer,
    HealthPeakFlowMonitor,
    HealthMedicationMonitor,
    HealthKneeProsthesis,
    HealthAnkleProsthesis,
    HealthGenericHealthManager,
    HealthPersonalMobilityDevice,
    PeripheralOther,
    PeripheralPointer,
    PeripheralKeyboard,
    PeripheralKeyboardAndPointer,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothDevicePairShowPinRequest {
    pub pin: String,
    pub confirmation_needed: bool,
}
