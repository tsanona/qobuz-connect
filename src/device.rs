use std::hash::{DefaultHasher, Hash as _, Hasher as _};

use uuid::Uuid;

use crate::proto::qconnect::{
    AudioQuality, DeviceCapabilities, DeviceInfo, DeviceType, VolumeRemoteControl,
};

fn device_uuid(name: &str) -> uuid::Uuid {
    let mut uuid = [0; 16];
    for (salt, half) in uuid.as_chunks_mut::<8>().0.iter_mut().enumerate() {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        salt.hash(&mut hasher);
        half.copy_from_slice(&hasher.finish().to_be_bytes());
    }
    uuid::Uuid::from_bytes(uuid)
}

/// How a device presents itself to a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub uuid: uuid::Uuid,
    pub name: String,
    pub brand: String,
    pub model: String,
    pub kind: DeviceType,
    pub max_audio_quality: AudioQuality,
    pub volume_remote_control: bool,
    pub software_version: String,
}

impl Device {
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        kind: DeviceType,
        max_audio_quality: AudioQuality, 
        volume_remote_control: bool
    ) -> Self {
        let name = name.into();
        Self { 
            uuid: device_uuid(&name),
            name,
            brand: "qobuz-connect".to_string(),
            model: String::new(),
            kind,
            max_audio_quality, 
            volume_remote_control,
            software_version: env!("CARGO_PKG_VERSION").to_string()
        }
    }

    #[must_use]
    pub fn with_uuid(mut self, uuid: impl Into<Uuid>) -> Self {
        self.uuid = uuid.into();
        self
    }

    #[must_use]
    pub fn with_brand(mut self, brand: &'static str) -> Self {
        self.brand = brand.to_string();
        self
    }

    #[must_use]
    pub fn with_model(mut self, model: &'static str) -> Self {
        self.model = model.to_string();
        self
    }

    #[must_use]
    pub fn with_version(mut self, version: &'static str) -> Self {
        self.software_version = version.to_string();
        self
    }
}

impl From<Device> for DeviceInfo {
    fn from(value: Device) -> Self {
        DeviceInfo {
            device_uuid: value.uuid.as_bytes().to_vec(),
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
            uuid: uuid::Uuid::from_slice(&value.device_uuid).unwrap_or_default(),
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
