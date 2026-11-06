use crate::proto::qconnect::{
    AudioQuality, DeviceCapabilities, DeviceInfo, DeviceType, VolumeRemoteControl,
};

/// How a device presents itself to a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub uuid: [u8; 16],
    pub name: String,
    pub brand: String,
    pub model: String,
    pub kind: DeviceType,
    pub max_audio_quality: AudioQuality,
    pub volume_remote_control: bool,
    pub software_version: String,
}

impl From<Device> for DeviceInfo {
    fn from(value: Device) -> Self {
        DeviceInfo {
            device_uuid: value.uuid.to_vec(),
            friendly_name: value.name,
            brand: value.brand,
            model: value.model,
            serial_number: String::new(),
            r#type: value.kind.into(),
            capabilities: Some(DeviceCapabilities {
                min_audio_quality: AudioQuality::Mp3.into(),
                max_audio_quality: value.max_audio_quality.into(),
                volume_remote_control: value.volume_remote_control.into(),
            }),
            software_version: value.software_version,
        }
    }
}

impl From<DeviceInfo> for Device {
    fn from(value: DeviceInfo) -> Self {
        let kind = value.r#type();
        let capabilities = value.capabilities.unwrap_or_default();
        Self {
            uuid: value.device_uuid.try_into().unwrap_or_default(),
            name: value.friendly_name,
            brand: value.brand,
            model: value.model,
            kind,
            max_audio_quality: capabilities.max_audio_quality(),
            volume_remote_control: capabilities.volume_remote_control()
                == VolumeRemoteControl::Allowed,
            software_version: value.software_version,
        }
    }
}
