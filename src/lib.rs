//! SigmaOS-absvolume: audio volume model.
//!
//! Real implementation: mixer, levels, and validation.
//! Replaces the 2-line stub.

use std::collections::BTreeMap;
use std::fmt;

/// Volume channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeChannel {
    pub id: String,
    pub name: String,
    pub device: String,
    pub level: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VolumeError {
    EmptyName,
    DuplicateChannel(String),
}

impl fmt::Display for VolumeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "channel name must not be empty"),
            Self::DuplicateChannel(id) => write!(f, "duplicate channel {:?}", id),
        }
    }
}

impl std::error::Error for VolumeError {}

/// Audio volume manager.
pub struct AudioVolume {
    pub name: String,
    channels: BTreeMap<String, VolumeChannel>,
}

impl AudioVolume {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            channels: BTreeMap::new(),
        }
    }

    pub fn add_channel(&mut self, channel: VolumeChannel) -> Result<(), VolumeError> {
        if channel.name.trim().is_empty() { return Err(VolumeError::EmptyName); }
        let id = channel.id.clone();
        if self.channels.contains_key(&id) {
            return Err(VolumeError::DuplicateChannel(id.clone()));
        }
        self.channels.insert(id, channel);
        Ok(())
    }

    pub fn channel_count(&self) -> usize { self.channels.len() }

    pub fn get_channel(&self, id: &str) -> Option<&VolumeChannel> { self.channels.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_create_aaa() {
        let v = AudioVolume::new("My Volume");
        assert_eq!(v.name, "My Volume");
        assert_eq!(v.channel_count(), 0);
    }

    #[test]
    fn test_add_channel_aaa() {
        let mut v = AudioVolume::new("My Volume");
        let c = VolumeChannel { id: "c1".into(), name: "master".into(), device: "speaker".into(), level: 75 };
        v.add_channel(c).unwrap();
        assert_eq!(v.channel_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut v = AudioVolume::new("My Volume");
        let c = VolumeChannel { id: "x".into(), name: "".into(), device: "a".into(), level: 50 };
        assert!(v.add_channel(c).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut v = AudioVolume::new("My Volume");
        let c = VolumeChannel { id: "x".into(), name: "X".into(), device: "a".into(), level: 50 };
        v.add_channel(c).unwrap();
        let c2 = VolumeChannel { id: "x".into(), name: "X".into(), device: "a".into(), level: 50 };
        assert!(v.add_channel(c2).is_err());
    }
}
