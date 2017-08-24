// All the consts and enums here are from openmpt's soundlib/modcommand.h
// They are not part of the public API and change to OpenMPT may break those without warning.
// They are only available here for the sake of conveinience and completeness.

const NOTE_NONE:u8 = 0;
const NOTE_MIN:u8 = 1;
const NOTE_MAX:u8 = 120;
const NOTE_MIDDLEC:u8 = 5 * 12 + NOTE_MIN;
const NOTE_KEYOFF:u8 = 0xFF;
const NOTE_NOTECUT:u8 = 0xFE;
const NOTE_FADE:u8 = 0xFD;
const NOTE_PC:u8 = 0xFC;
const NOTE_PCS:u8 = 0xFB;

pub struct ModCommand {
	note : Note,
	instr: u8,
	volcmd: VolumeCommand,
	command: EffectCommand,
}

impl ModCommand {
	pub fn new(note : u8, instr : u8, volcmd : u8, command : u8, vol : u8, param : u8) -> Result<ModCommand, String> {
		let note_type = ModCommand::note_from_value(note);
		let note_type = match note_type {
			Ok(n) => n,
			Err(e) => return Err(e),
		};

		let vol_type = ModCommand::volume_from_command_param(volcmd, vol);
		let vol_type = match vol_type {
			Ok(v) => v,
			Err(e) => return Err(e),
		};

		let effect_type = ModCommand::effect_from_command_param(command, param);
		let effect_type = match effect_type {
			Ok(c) => c,
			Err(e) => return Err(e),
		};

		Ok(ModCommand {
			note: note_type,
			instr: instr,
			volcmd: vol_type,
			command: effect_type,
		})
	}

	fn note_from_value(note_val : u8) -> Result<Note, String> {
		match note_val {
			NOTE_NONE => Ok(Note::None),
			NOTE_MIN...NOTE_MAX => Ok(Note::Note(note_val)),
			NOTE_KEYOFF => Ok(Note::Special(SpecialNote::KeyOff)),
			NOTE_NOTECUT => Ok(Note::Special(SpecialNote::NoteCut)),
			NOTE_FADE => Ok(Note::Special(SpecialNote::Fade)),
			NOTE_PC => Ok(Note::Special(SpecialNote::ParamControl)),
			NOTE_PCS => Ok(Note::Special(SpecialNote::ParamControlSmooth)),
			_ => Err("Invalid note".to_owned()),
		}
	}

	fn effect_from_command_param(cmd : u8, param : u8) -> Result<EffectCommand, String> {
		unimplemented!();
		match cmd {
			0 => Ok(EffectCommand::None),
			_ => Err("Invalid effect".to_owned()),
		}
	}

	fn volume_from_command_param(cmd : u8, param : u8) -> Result<VolumeCommand, String> {
		match cmd {
			0  => Ok(VolumeCommand::None),
			1  => Ok(VolumeCommand::Volume(param)),
			2  => Ok(VolumeCommand::Panning(param)),
			3  => Ok(VolumeCommand::VolSlideUp(param)),
			4  => Ok(VolumeCommand::VolSlideDown(param)),
			5  => Ok(VolumeCommand::FineVolUp(param)),
			6  => Ok(VolumeCommand::FineVolDown(param)),
			7  => Ok(VolumeCommand::VibratoSpeed(param)),
			8  => Ok(VolumeCommand::VibratoDepth(param)),
			9  => Ok(VolumeCommand::PanSlideLeft(param)),
			10 => Ok(VolumeCommand::PanSlideRight(param)),
			11 => Ok(VolumeCommand::TonePortamento(param)),
			12 => Ok(VolumeCommand::PortaUp(param)),
			13 => Ok(VolumeCommand::PortaDown(param)),
			14 => Ok(VolumeCommand::DelayCut(param)),
			15 => Ok(VolumeCommand::Offset(param)),
			_  => Err("Invalid volume command".to_owned()),
		}
	}
}

pub enum Note {
	None,
	Note(u8),
	Special(SpecialNote),
}

pub enum SpecialNote {
	KeyOff,
	NoteCut,
	Fade,
	ParamControl,
	ParamControlSmooth,
}

pub enum VolumeCommand {
	None,
	Volume(u8),
	Panning(u8),
	VolSlideUp(u8),
	VolSlideDown(u8),
	FineVolUp(u8),
	FineVolDown(u8),
	VibratoSpeed(u8),
	VibratoDepth(u8),
	PanSlideLeft(u8),
	PanSlideRight(u8),
	// Equivalent to the effect, but may be 4 or 16 times less precise
	TonePortamento(u8),
	// Equivalent to the effect, but may be 4 or 16 times less precise
	PortaUp(u8),
	// Equivalent to the effect, but may be 4 or 16 times less precise
	PortaDown(u8),
	// Unused
	DelayCut(u8),
	Offset(u8),
}

pub enum EffectCommand {
	None,
	// Cycle between note, note+x and note+y on each tick
	Arpeggio (u8, u8),
	// Raise pitch by xy per tick, sometimes including the first
	// Slide fraction is generally 1/16th of a semitone
	PortamentoUp (u8),
	// Lower pitch by xy per tick, sometimes including the first
	// Slide fraction is generally 1/16th of a semitone
	PortamentoDown (u8),
	// Slide pitch of old note towards new note by xy per tick and stop once reached.
	// Slide fraction is generally 1/16th of a semitone
	TonePortamento (u8),
	// Modulates frequency at a speed of x steps (of 64) *PER ROW* and depth y
	// Depth is generally in 1/16th of a semitone
	Vibrato (u8, u8),
	// Volume Slide + Continue portamento
	TonePortaVol (u8, u8),
	// Volume Slide + Continue vibrato
	VibratoVol (u8, u8),
	// Modulates sample volume at a speed of x steps (of 64) *PER ROW* and depth y
	Tremolo (u8, u8),
	// Set panning from 0x0 to 0xF
	Panning8 (u8),
	// Start playing sample at position xy * 256
	Offset(u8),
	// Raise sample volume by x or lower by y on each tick but the first
	VolumeSlide(u8, u8),
	// Jump to pattern at order xy
	PositionJump(u8),
	// Set sample volume at xy (between 0 and 0x40)
	Volume(u8),
	// Jump to row xy of pattern set to play next
	PatternBreak(u8),
	// Retrigger every y ticks, x affects retrigger volume when set
	Retrig(u8, u8),
	// Set speed at xy ticks per row
	Speed(u8),
	// Set tempo at xy beats per minute
	Tempo(u8),
	// Turn volume on for x+1 ticks and mute for y+1 ticks repeatedly
	Tremor(u8, u8),
	// (Mod and XM) Super command, with x the subcommand and y the parameter.
	ModCmdEX(u8, u8),
	// (S3M and IT) Super command, with x the subcommand and y the parameter.
	S3MCmdEX(u8, u8),
	// Set channel volume at xy (between 0 and 0x40)
	ChannelVolume(u8),
	// Raise channel volume by x or lower by y on each tick but the first
	ChannelVolSlide(u8, u8),
	// Set global volume at xy (between 0 and 0x40)
	GlobalVolume(u8),
	// Raise global volume by x or lower by y on each tick but the first
	GlobalVolSlide(u8, u8),
	// Trigger Note Off after xy ticks
	KeyOff(u8),
	// Same as vibrato, but depth is 4 times finer
	FineVibrato(u8, u8),
	// Modulate panning at a speed of x steps (of 64) *PER ROW* and depth y
	Panbrello(u8, u8),
	// (XM only) Super command, with x the subcommand and y the parameter.
	XFinePortaUpDown(u8, u8),
	// Slide panning position right by x or left by y on each tick but the first
	// Depending on format and settings, it could also be apply on the first tick only or on every tick.
	PanningSlide(u8, u8),
	// Sets the volume envelope position to xy ticks
	SetEnvPosition(u8),
	// Execute a midi macro
	Midi(u8),
	// Execute an interpolated midi macro
	SmoothMidi(u8),
	// Delay note for x ticks and cut after another y ticks.
	// If the row ends before either effect is applied (speed is greater than x or x+y), the effect won't be applied.
	DelayCut(u8, u8),
	// Combines the parameter value with the one on the row above it
	XParam(u8),
	NoteSlideUp(u8, u8),
	NoteSlideDown(u8, u8),
	NoteSlideUpRetrig(u8, u8),
	NoteSlideDownRetrig(u8, u8),
	ReverseOffset(u8),
	// x : chns, y: enable
	DBMEcho(u8, u8),
	OffsetPercentage(u8),
}