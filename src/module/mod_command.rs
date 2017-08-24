pub struct ModCommand {
	note : Note,
	instr: u8,
	volcmd: VolumeCommand,
	command: EffectCommand,
}

impl ModCommand {
	fn new(note : Note, instr : u8, volcmd : u8, vol : u8, command : u8, param : u8) -> ModCommand {
		unimplemented!();
	}
}

// Middle C (C4) is 60 - 1
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