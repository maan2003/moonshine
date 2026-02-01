// Gamepad support disabled - inputtino dependency removed
use strum_macros::FromRepr;
use tokio::sync::mpsc;

use crate::session::stream::control::FeedbackCommand;

#[derive(Debug, FromRepr)]
#[repr(u8)]
pub enum GamepadKind {
	Unknown = 0x00,
	Xbox = 0x01,
	PlayStation = 0x02,
	Nintendo = 0x03,
}

#[derive(Debug)]
pub struct GamepadInfo {
	pub index: u8,
	kind: GamepadKind,
	capabilities: u16,
	_supported_buttons: u32,
}

impl GamepadInfo {
	pub fn from_bytes(buffer: &[u8]) -> Result<Self, ()> {
		const EXPECTED_SIZE: usize = 8;

		if buffer.len() < EXPECTED_SIZE {
			tracing::warn!(
				"Expected at least {EXPECTED_SIZE} bytes for GamepadInfo, got {} bytes.",
				buffer.len()
			);
			return Err(());
		}

		Ok(Self {
			index: buffer[0],
			kind: GamepadKind::from_repr(buffer[1])
				.ok_or_else(|| tracing::warn!("Unknown gamepad kind: {}", buffer[1]))?,
			capabilities: u16::from_le_bytes(buffer[2..4].try_into().unwrap()),
			_supported_buttons: u32::from_le_bytes(buffer[4..8].try_into().unwrap()),
		})
	}
}

#[derive(Debug)]
pub struct GamepadTouch {
	pub index: u8,
	_event_type: u8,
	pointer_id: u32,
	pub x: f32,
	pub y: f32,
	pub pressure: f32,
}

impl GamepadTouch {
	pub fn from_bytes(buffer: &[u8]) -> Result<Self, ()> {
		const EXPECTED_SIZE: usize = 20;

		if buffer.len() < EXPECTED_SIZE {
			tracing::warn!(
				"Expected at least {EXPECTED_SIZE} bytes for GamepadTouch, got {} bytes.",
				buffer.len()
			);
			return Err(());
		}

		Ok(Self {
			index: buffer[0],
			_event_type: buffer[1],
			pointer_id: u32::from_le_bytes(buffer[4..8].try_into().unwrap()),
			x: f32::from_le_bytes(buffer[8..12].try_into().unwrap()).clamp(0.0, 1.0),
			y: f32::from_le_bytes(buffer[12..16].try_into().unwrap()).clamp(0.0, 1.0),
			pressure: f32::from_le_bytes(buffer[16..20].try_into().unwrap()).clamp(0.0, 1.0),
		})
	}
}

#[derive(Debug)]
pub struct GamepadUpdate {
	pub index: u16,
	pub active_gamepad_mask: u16,
	button_flags: u32,
	left_trigger: u8,
	right_trigger: u8,
	left_stick: (i16, i16),
	right_stick: (i16, i16),
}

impl GamepadUpdate {
	pub fn from_bytes(buffer: &[u8]) -> Result<Self, ()> {
		const EXPECTED_SIZE: usize = 26;

		if buffer.len() < EXPECTED_SIZE {
			tracing::warn!(
				"Expected at least {EXPECTED_SIZE} bytes for GamepadUpdate, got {} bytes.",
				buffer.len()
			);
			return Err(());
		}

		Ok(Self {
			index: u16::from_le_bytes(buffer[2..4].try_into().unwrap()),
			active_gamepad_mask: u16::from_le_bytes(buffer[4..6].try_into().unwrap()),
			button_flags: u16::from_le_bytes(buffer[8..10].try_into().unwrap()) as u32
				| (u16::from_le_bytes(buffer[22..24].try_into().unwrap()) as u32) << 16,
			left_trigger: buffer[10],
			right_trigger: buffer[11],
			left_stick: (
				i16::from_le_bytes(buffer[12..14].try_into().unwrap()),
				i16::from_le_bytes(buffer[14..16].try_into().unwrap()),
			),
			right_stick: (
				i16::from_le_bytes(buffer[16..18].try_into().unwrap()),
				i16::from_le_bytes(buffer[18..20].try_into().unwrap()),
			),
		})
	}
}

#[derive(Debug)]
pub struct GamepadMotion {
	pub index: u8,
	motion_type: u8,
	x: f32,
	y: f32,
	z: f32,
}

impl GamepadMotion {
	pub fn from_bytes(buffer: &[u8]) -> Result<Self, ()> {
		const EXPECTED_SIZE: usize = 16;

		if buffer.len() < EXPECTED_SIZE {
			tracing::warn!(
				"Expected at least {EXPECTED_SIZE} bytes for GamepadMotion, got {} bytes.",
				buffer.len()
			);
			return Err(());
		}

		Ok(Self {
			index: buffer[0],
			motion_type: buffer[1],
			x: f32::from_le_bytes(buffer[4..8].try_into().unwrap()),
			y: f32::from_le_bytes(buffer[8..12].try_into().unwrap()),
			z: f32::from_le_bytes(buffer[12..16].try_into().unwrap()),
		})
	}
}

#[derive(Debug)]
pub struct GamepadBattery {
	pub index: u8,
	battery_state: u8,
	battery_percentage: u8,
}

impl GamepadBattery {
	pub fn from_bytes(buffer: &[u8]) -> Result<Self, ()> {
		const EXPECTED_SIZE: usize = 4;

		if buffer.len() < EXPECTED_SIZE {
			tracing::warn!(
				"Expected at least {EXPECTED_SIZE} bytes for GamepadBattery, got {} bytes.",
				buffer.len()
			);
			return Err(());
		}

		Ok(Self {
			index: buffer[0],
			battery_state: buffer[1],
			battery_percentage: buffer[2],
		})
	}
}

pub struct Gamepad {
	_index: u8,
}

impl Gamepad {
	pub async fn new(info: &GamepadInfo, _feedback_tx: mpsc::Sender<FeedbackCommand>) -> Result<Self, ()> {
		tracing::warn!("Gamepad support is disabled (inputtino not available)");
		Err(())
	}

	pub fn update(&mut self, _update: &GamepadUpdate) {}

	pub fn touch(&mut self, _touch: &GamepadTouch) {}

	pub fn set_motion(&self, _motion: &GamepadMotion) {}

	pub fn set_battery(&self, _gamepad_battery: &GamepadBattery) {}
}
