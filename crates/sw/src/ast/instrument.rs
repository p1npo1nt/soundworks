//instrumentation def for SW

use serde::{Serialize, Deserialize};

//TYPES:
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnvelopeCurve {
    Linear,
    Exponential,
    Logarithmic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OscillatorType {
    Sine,
    Square,
    Saw,
    Triangle,
    Noise,
}
//NEED TO ADD PitchEnvelope etc

#[derive(Debug, Clone)]
pub enum Instrument {
    Drum(DrumInstrument),
    Synth(SynthInstrument),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrumInstrument { //definition of drum instrument
    pub name: String,
    pub body: DrumBody,                 //(base frequency, decay)
    pub click: Option<DrumClick>,       //click/attack parameters
    pub distortion: Option<f32>,        //distortion amount
    pub compression: Option<f32>,
    pub variants: Vec<DrumVariant>,     //variation of instrument
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthInstrument { //definition of synth instrument
    pub name: String,
    pub oscillators: Vec<Oscillator>,
    pub filter: Option<Filter>,
    pub amp_env: Envelope,
    pub variants: Vec<SynthVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrumBody {
    pub freq: f32,
    pub pitch_env: Option<PitchEnvelope>,
    pub decay: f32,
    pub curve: Option<EnvelopeCurve>,       //envelope curve type
}


//OSCILLATOR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Oscillator {
    pub osc_type: OscillatorType,
    pub freq: f32,
    pub voices: Option<u8>,
    pub detune: Option<f32>,
    pub level: f32,
}

//constructor
impl DrumInstrument {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            body: DrumBody {
                freq: 60.0,
                pitch_env: None,
                decay: 300.0,
                curve: Some(EnvelopeCurve::Exponential),
            },
            click: None,
            distortion: None,
            compression: None,
            variants: Vec::new(),
        }
    }
    
    // ... more methods to come.....
}