use std::collections::HashMap;
use std::io::Read;

use base64;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

pub use errors::{
	ExportError,
	ExportResult,
	ImportError,
	ImportResult,
};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum BlueprintObject {
	Blueprint(Blueprint),
	BlueprintBook(BlueprintBook),
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BlueprintBook {
	/// The name of the item that was saved ("blueprint-book" in vanilla).
	pub item: String,

	/// The name of the blueprint set by the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,

	/// The color of the label of this book.
	pub label_color: Option<Color>,

	/// The actual content of the book.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub blueprints: Vec<BlueprintBookEntry>,

	/// The index of the currently selected blueprint. (0 Based)
	pub active_index: i32,

	// The map version the blueprint was created in.
	pub version: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BlueprintBookEntry {
	pub index: i32,
	pub blueprint: Blueprint,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Blueprint {
	/// The name of the item that was saved ("blueprint" in vanilla)
	pub item: String,

	/// The name of the blueprint set by the user.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,

	/// The color of the label of this blueprint.
	pub label_color: Option<Color>,

	/// The actual content of the blueprint.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub entities: Vec<Entity>,

	/// The tiles included in the blueprint. ()
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub tiles: Vec<Tile>,

	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub icons: Vec<Icon>,

	pub version: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Entity {
	pub entity_number: i32,
	pub name: String,
	pub position: Position,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub direction: Option<u8>,

	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	pub connections: HashMap<u8, ConnectionPoint>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub control_behavior: Option<ControlBehavior>,

	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	pub items: HashMap<String, u32>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub recipe: Option<String>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub bar: Option<u8>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub infinity_settings: Option<InfinitySettings>,

	/// If this entity is an underground belt or loader, this is used to determine the input/output state.
	#[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
	pub underground_type: Option<UndergroundBeltOrLoaderType>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub input_priority: Option<SplitterDirection>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub output_priority: Option<SplitterDirection>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub filter: Option<String>,

	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub filters: Vec<ItemFilter>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub override_stack_size: Option<u8>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub drop_position: Option<Position>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub pickup_position: Option<Position>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub request_filters: Option<Vec<LogisticFilter>>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub request_from_buffers: Option<bool>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub parameters: Option<SpeakerParameter>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub alert_parameters: Option<SpeakerAlertParameter>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub auto_launch: Option<bool>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub color: Option<Color>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub station: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Tile {
	/// Prototype name of the tile (e.g. "concrete")
	pub name: String,

	/// The position of the tile within the blueprint.
	pub position: Position,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Icon {
	/// The index of the icon. (1 based)
	pub index: i32,

	/// The name of the prototype that will be used as the icon.
	pub signal: SignalID,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignalID {
	/// The name of the signal prototype. (e.g. "signal-black")
	pub name: String,

	/// Reflects the prototype's type.
	///
	/// For example, `signal-black` is a `virtual` signal.
	/// whereas `water` is `fluid` and `stone-wall` is `item`.
	#[serde(rename = "type")]
	pub signal_type: SignalType,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SignalType {
	Item,
	Fluid,
	Virtual,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Position {
	/// The x position within the blueprint. (0 is the centre)
	pub x: f64,

	/// The y position within the blueprint. (0 is the centre)
	pub y: f64,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ConnectionPoint {
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub red: Vec<ConnectionData>,
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub green: Vec<ConnectionData>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConnectionData {
	pub entity_id: u8,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub circuit_id: Option<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ControlBehavior {
	/// Accumulator:
	/// 	The signal the accumulator emits as a percentage of its current charge. (0 - 100)
	///
	/// Gate:
	/// 	The signal to send when the player gets near. (`1` if the player is nearby)
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub output_signal: Option<SignalID>,

	// Arithmetic combinator.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub arithmetic_conditions: Option<ArithmeticConditions>,

	// Decider combinator.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub decider_conditions: Option<DeciderConditions>,

	// This one is kinda random...
	// Some things just declare it because that's just
	// where they store their circuit condition.
	// The use of the condition itself depends on the entity.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub circuit_condition: Option<CircuitCondition>,

	/// At the time of writing, the only use is
	/// enabling/disabling the inserter based on the condition.
	///
	/// Used with `connect_to_logistic_network`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub logistic_condition: Option<CircuitCondition>,

	/// If true, the entity should try connecting to a logistics network.
	///
	/// Used with `logistic_condition`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub connect_to_logistic_network: Option<bool>,

	/// If true, the inserter will read the contents of its hand.
	/// Used in conjunction with `circuit_hand_read_mode`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub circuit_read_hand_contents: Option<bool>,

	/// Determines the mode the inserter uses when emitting the signals.
	/// Pulse: Only pulses them for a single tick.
	/// Hold: Emits a continuous signal while the item is still in its hand.
	///
	/// Only used when `circuit_read_hand_contents` is set to `Some(true)`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub circuit_hand_read_mode: Option<HandReadMode>,

	/// If true, the inserter will use `stack_control_input_signal` to determine its stack size.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub circuit_set_stack_size: Option<bool>,

	/// Determines what signal the inserter should use to determine the stack size.
	/// Only works with `circuit_set_stack_size`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub stack_control_input_signal: Option<SignalID>,

	/// Inserter:
	/// Determines what the `circuit_condition` does with the inserter.
	/// I couldn't actually confirm if this works correctly.
	/// It might be an older field.
	/// Use `circuit_read_hand_contents` or `circuit_set_stack_size` or `circuit_hand_read_mode` or `stack_control_input_signal` instead...
	/// I think it's just the presence of the `circuit_condition` that determines if it should enable or disable it...
	/// I'll need to test this.
	///
	/// Roboport:
	/// It uses this to determine which mode it's in.
	/// The roboport uses both `EnableDisable` and `SetFilters`.
	/// `EnableDisable`: if it should read the logistic network and send it to the circuit network.
	/// `SetFilters`: It will emit signals for robots within the roboport itself (NOT THE NETWORK) using the values from:
	///   `available_logistic_output_signal`
	///   `total_logistic_output_signal`
	///   `available_construction_output_signal`
	///   `total_construction_output_signal`
	///
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub circuit_mode_of_operation: Option<CircuitModeOfOperation>,

	/// The values the constant combinator will output.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub filters: Vec<Filter>,

	/// Determines if the constant combinator is on when created.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub is_on: Option<bool>,

	/// If the lamp should use colours...
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub use_colors: Option<bool>,

	/// If set, the `circuit_condition` determines
	/// if the signal should stop trains from going through.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	circuit_close_signal: Option<bool>,

	/// If the game should output a signal based on the state of the signal.
	/// red_output_signal is the signal when the rail signal is red, etc.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	circuit_read_signal: Option<bool>,

	/// Look at `circuit_read_signal`
	#[serde(default, skip_serializing_if = "Option::is_none")]
	red_output_signal: Option<SignalID>,

	/// Look at `circuit_read_signal`
	#[serde(default, skip_serializing_if = "Option::is_none")]
	orange_output_signal: Option<SignalID>,

	/// Look at `circuit_read_signal`
	#[serde(default, skip_serializing_if = "Option::is_none")]
	green_output_signal: Option<SignalID>,

	/// The signal the roboport uses when in `SetFilters` mode to represent the available
	/// logistic robots within itself (Not the network).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	available_logistic_output_signal: Option<SignalID>,

	/// The signal the roboport uses when in `SetFilters` mode to represent the total
	/// logistic robots within itself (Not the network).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	total_logistic_output_signal: Option<SignalID>,

	/// The signal the roboport uses when in `SetFilters` mode to represent the available
	/// construction robots within itself (Not the network).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	available_construction_output_signal: Option<SignalID>,

	/// The signal the roboport uses when in `SetFilters` mode to represent the total
	/// construction robots within itself (Not the network).
	#[serde(default, skip_serializing_if = "Option::is_none")]
	total_construction_output_signal: Option<SignalID>,

	/// If true, the station will be enabled based off of the `circuit_condition`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	circuit_enable_disable: Option<bool>,

	/// If true, the contents of the circuit are sent to the train.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	send_to_train: Option<bool>,

	/// Reads the contents of the train that is currently stopped at the station.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	read_from_train: Option<bool>,

	/// If true, the station will emit a signal when a train is stopped at the station.
	/// Used with `train_stopped_signal`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	read_stopped_train: Option<bool>,

	/// When a train is stopped, this signal will be emitted.
	/// The number that will be emitted is unique for that train.
	/// Used with `read_stopped_train`.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	train_stopped_signal: Option<SignalID>,

	/// Gate will remain open as long as the `circuit_condition` is true.
	///
	/// Used with `circuit_condition`
	#[serde(default, skip_serializing_if = "Option::is_none")]
	circuit_open_gate: Option<bool>,

	/// Sends a signal when the player is nearby and the gate should open.
	///
	/// Used with `output_signal`
	#[serde(default, skip_serializing_if = "Option::is_none")]
	circuit_read_sensor: Option<bool>,
}


#[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u32)]
pub enum CircuitModeOfOperation {
	EnableDisable = 0,
	SetFilters = 1,
	ReadHandContents = 2,
	None = 3,
}

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug)]
#[repr(u32)]
pub enum HandReadMode {
	PulseMode = 0,
	HoldMode = 1,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ArithmeticConditions {
	#[serde(rename = "first_signal", skip_serializing_if = "Option::is_none")]
	pub left: Option<SignalID>,
	#[serde(flatten)]
	pub right: SignalOrConstant,
	pub operation: Operation,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_signal: Option<SignalID>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DeciderConditions {
	#[serde(rename = "first_signal", skip_serializing_if = "Option::is_none")]
	pub left: Option<SignalID>,
	#[serde(flatten)]
	pub right: SignalOrConstant,
	pub comparator: Comparator,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_signal: Option<SignalID>,
	pub copy_count_from_input: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CircuitCondition {
	#[serde(rename = "first_signal", skip_serializing_if = "Option::is_none")]
	pub left: Option<SignalID>,
	#[serde(flatten)]
	pub right: Option<SignalOrConstant>,
	pub comparator: Comparator,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Filter {
	pub signal: SignalID,
	pub count: i32,
	pub index: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SignalOrConstant {
	#[serde(rename = "second_signal")]
	Signal(SignalID),
	/// Serde Bug: #1504
	/// Aliases for flattened structs or enums don't work.
	#[serde(rename = "constant", alias = "second_constant")]
	Constant(i32),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Comparator {
	#[serde(rename = "=")]
	Equal,
	#[serde(rename = "<")]
	LessThan,
	#[serde(rename = "≤")]
	LessThanEqual,
	#[serde(rename = ">")]
	GreaterThan,
	#[serde(rename = "≥")]
	GreaterThanEqual,
	#[serde(rename = "≠")]
	NotEqual,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Operation {
	#[serde(rename = "+")]
	Add,
	#[serde(rename = "-")]
	Sub,
	#[serde(rename = "*")]
	Mul,
	#[serde(rename = "/")]
	Div,
	#[serde(rename = "%")]
	Mod,
	#[serde(rename = "^")]
	Exp,
	#[serde(rename = "<<")]
	LeftBitShift,
	#[serde(rename = ">>")]
	RightBitShift,
	#[serde(rename = "AND")]
	BitAnd,
	#[serde(rename = "OR")]
	BitOr,
	#[serde(rename = "XOR")]
	BitXor,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InfinitySettings {}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UndergroundBeltOrLoaderType {
	Input,
	Output,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemFilter {
	name: String,
	index: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LogisticFilter {
	/// Name of the item prototype this filter is set to.
	name: String,

	/// Index of the filter. (1 based)
	index: u32,

	/// The number the filter is set to. (Set to 0 for storage chests.)
	count: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpeakerParameter {
	// The volume of the speaker.
	playback_volume: f64,

	// Ignore proximity checks and play sound for all players.
	playback_globally: bool,

	// Allow 10 different sounds to be played at once.
	// Otherwise, only one can be played.
	allow_polyphony: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SpeakerAlertParameter {
	/// Display the alert in the GUI.
	show_alert: bool,

	/// If an alert is shown on the map.
	show_on_map: bool,

	/// The icon that will be shown on the map if configured.
	icon_signal_id: Option<SignalID>,

	/// The message that will be displayed with the alert.
	alert_message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Color {
	#[serde(rename = "r")]
	red: f32,
	#[serde(rename = "g")]
	green: f32,
	#[serde(rename = "b")]
	blue: f32,
	#[serde(rename = "a")]
	alpha: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SplitterDirection {
	Left,
	Right,
}

fn import_as<T: DeserializeOwned>(input: &str) -> ImportResult<T> {
	let (version, input) = input.split_at(1);
	// We only support the first version... for now..
	assert_eq!(version, "0");
	let bytes: Vec<u8> = base64::decode(input).unwrap();

	if cfg!(debug_assertions) {
		let mut buf = String::new();
		ZlibDecoder::new(&bytes[..]).read_to_string(&mut buf).unwrap();
		println!("========================================================================================");
		println!("{}", buf);
		println!("========================================================================================");
	}

	Ok(serde_json::from_reader(ZlibDecoder::new(&bytes[..]))?)
}

pub fn import(input: &str) -> ImportResult<BlueprintObject> {
	import_as(input)
}

pub fn import_as_value(input: &str) -> ImportResult<Value> {
	import_as(input)
}

pub fn export<T: ?Sized + Serialize>(value: &T) -> ExportResult<String> {
	let mut bytes = Vec::new();
	serde_json::to_writer(ZlibEncoder::new(&mut bytes, Compression::best()), value)?;
	let mut result = "0".to_string();
	base64::encode_config_buf(&bytes, base64::STANDARD, &mut result);
	Ok(result)
}

impl BlueprintObject {
	pub fn export(&self) -> ExportResult<String> {
		export(self)
	}

	pub fn visit_blueprints(&mut self, mut visitor: impl FnMut(&mut Blueprint)) {
		match self {
			BlueprintObject::Blueprint(blueprint) => (visitor)(blueprint),
			BlueprintObject::BlueprintBook(book) => {
				for BlueprintBookEntry { blueprint, .. } in &mut book.blueprints {
					(visitor)(blueprint)
				}
			}
		}
	}
}

const CURRENT_MAP_VERSION: u64 = 73018966017;

impl Blueprint {
	pub fn export(&self) -> ExportResult<String> {
		export(self)
	}

	pub fn simple(name: String, entities: Vec<Entity>) -> Blueprint {
		Blueprint {
			item: "blueprint".into(),
			label: Some(name),
			entities,
			version: CURRENT_MAP_VERSION,
			..Default::default()
		}
	}
}

impl BlueprintBook {
	pub fn simple(name: String, blueprints: Vec<Blueprint>) -> BlueprintBook {
		BlueprintBook {
			item: "blueprint-book".into(),
			label: Some(name),
			blueprints: blueprints
				.into_iter()
				.enumerate()
				.map(|(index, blueprint)| BlueprintBookEntry {
					index: index as i32 + 1,
					blueprint,
				})
				.collect(),
			active_index: 0,
			version: CURRENT_MAP_VERSION,
			..Default::default()
		}
	}
}

pub mod errors {
	use base64::DecodeError;
	use serde_json::Error as JsonError;
	use thiserror::Error;

	pub type ImportResult<T> = std::result::Result<T, crate::errors::ImportError>;
	pub type ExportResult<T> = std::result::Result<T, crate::errors::ExportError>;

	#[derive(Error, Debug)]
	pub enum ImportError {
		#[error("Unable to import json. [Invalid json]")]
		JsonError(#[source] JsonError),
		#[error("Unable to decode base64.")]
		DecodeError(#[source] DecodeError),
		#[error("Unknown error.")]
		Unknown,
	}

	impl From<JsonError> for ImportError {
		fn from(err: JsonError) -> Self {
			ImportError::JsonError(err)
		}
	}

	impl From<DecodeError> for ImportError {
		fn from(err: DecodeError) -> Self {
			ImportError::DecodeError(err)
		}
	}

	#[derive(Error, Debug)]
	pub enum ExportError {
		#[error("Unable to export json.")]
		JsonError(#[source] JsonError),
		#[error("Unknown error.")]
		Unknown,
	}

	impl From<JsonError> for ExportError {
		fn from(err: JsonError) -> Self {
			ExportError::JsonError(err)
		}
	}
}
