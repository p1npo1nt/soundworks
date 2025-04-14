# 🎵 Soundworks 🎵

*Work in progress*

Soundworks (SW) is a domain-specific language for electronic music production with a focus on synthesizers, bass design, and drill music.

## Project Structure

```
project/
├── CONFIG.swr        # Project configuration
├── instr/            # Instruments directory
│   ├── 808.swr       
│   ├── bass.swr      
│   └── leads.swr
├── comp/             # Compositions directory
│   ├── verse.swr     
│   ├── drop.swr      
│   └── main.swr      
└── media/            # Output directory
    ├── renders/      
    └── stems/        
```

## Language Syntax

### Project Configuration (CONFIG.swr)

```
project "Drill Beat 01" {
    author: "Producer Name"
    created: "2025-04-14"
    
    // Global project settings
    tempo: 140bpm
    time_signature: 4/4
    key: F minor
    
    // Default output configuration
    output {
        sample_rate: 48kHz
        bit_depth: 24
        format: wav
    }
    
    // Project-wide routing
    routing {
        master_fx: [limiter, maximizer]
        sidechain_source: 808:kick
    }
}
```

### Instrument Definitions

#### Drum Instruments (instr/808.swr)

```
instrument "kick" {
    type: drum
    
    // Basic parameters
    body {
        freq: 60Hz
        pitch_env: 150Hz -> 60Hz over 50ms
        decay: 300ms
        curve: exponential
    }
    
    click {
        level: 0.3
        freq: 1kHz
        decay: 10ms
    }
    
    // Processing
    distortion: 0.2
    compression: 4.0
    
    // Variants with different characteristics
    variant "bassiest" {
        body {
            freq: 45Hz
            decay: 400ms
        }
        distortion: 0.4
    }
    
    variant "short" {
        body {
            decay: 150ms
        }
        click {
            level: 0.6
        }
    }
}

instrument "snare" {
    type: drum
    
    tone {
        freq: 180Hz
        decay: 100ms
        level: 0.5
    }
    
    noise {
        type: pink
        filter: 1kHz
        decay: 200ms
        level: 0.8
    }
    
    variant "clappy" {
        noise {
            type: white
            bp_filter: 2kHz
            resonance: 0.7
        }
        reverb: 0.2
    }
}
```

#### Synth Instruments (instr/bass.swr)

```
instrument "sub" {
    type: synth
    
    osc1 {
        type: saw
        voices: 2
        detune: 0.1
        level: 1.0
    }
    
    osc2 {
        type: sine
        level: 0.7
        phase: 0.25
    }
    
    mixer {
        balance: 0.6  // 0.0 = all osc1, 1.0 = all osc2
    }
    
    filter {
        type: lowpass
        cutoff: 500Hz
        resonance: 0.7
        drive: 0.3
        
        env {
            attack: 10ms
            decay: 300ms
            amount: 0.6
            target: 2000Hz
        }
    }
    
    amp_env {
        attack: 10ms
        decay: 200ms
        sustain: 0.7
        release: 400ms
    }
    
    variant "wobble" {
        filter {
            lfo {
                type: sine
                rate: 1/8
                amount: 0.7
                target: cutoff
                range: [200Hz, 2000Hz]
            }
        }
    }
    
    variant "pluck" {
        amp_env {
            attack: 1ms
            decay: 300ms
            sustain: 0
            release: 50ms
        }
        
        filter {
            env {
                attack: 1ms
                decay: 100ms
                amount: 0.8
            }
        }
    }
}
```

### Pattern Creation

#### Basic Pattern Syntax (comp/patterns.swr)

```
pattern "bassline" {
    // Note format: [pitch][octave][duration (optional)]
    notes: [C2 ~ ~ C2 Eb2 ~ G2 ~]
    velocities: [100 0 0 80 90 0 70 0]
    length: 2 bars
    
    // Transformations
    transform {
        swing: 0.3
        shuffle: 1/16
    }
}

// Drum grid notation
pattern "drums" {
    // x marks a hit, ~ marks a rest
    kick:  [x ~ ~ x ~ ~ x ~]
    snare: [~ ~ x ~ ~ ~ x ~]
    hihat: [x x x x x x x x]
    
    length: 2 bars
}
```

#### Advanced Pattern Operations

```
pattern "complex_rhythm" {
    // Pattern combination
    base: pattern("drums")
    fill: pattern("drum_fill")
    
    // Operations
    result: base.repeat(3) + fill
    
    // Timing manipulations
    groove: 0.3
    humanize: 5ms
}
```

### Composition Files

#### Section Composition (comp/drop.swr)

```
composition "Drop Section" {
    length: 16 bars
    
    // Import instruments with specific variants
    use 808:kick.bassiest as kick
    use 808:snare.clappy as snare
    use 808:hihat.open as open_hat
    use bass:sub.wobble as bass
    
    // Pattern definitions
    pattern "kick_pattern" {
        grid: [x ~ ~ x ~ ~ x ~] * 8
        velocity: [100 0 0 90 0 0 95 0]
    }
    
    pattern "snare_pattern" {
        grid: [~ ~ s ~ ~ ~ s ~] * 8
    }
    
    pattern "bass_pattern" {
        notes: [F1 ~ ~ F1 Ab1 ~ C2 ~] * 4
        glide: [0 0 0 0 20ms 0 0 0]
    }
    
    // Sequence everything together
    sequence {
        kick: pattern("kick_pattern")
        snare: pattern("snare_pattern")
        bass: pattern("bass_pattern").swing(0.3)
        
        // Events at specific times
        at 8 bars {
            bass.param.filter.cutoff: 500Hz -> 2000Hz over 8 bars
        }
    }
    
    // Section effects
    effects {
        reverb {
            room: 0.3
            wet: 0.1
        }
    }
}
```

#### Main Composition (comp/main.swr)

```
composition "Full Track" {
    // Import section compositions
    import "intro.swr" as intro
    import "verse.swr" as verse
    import "drop.swr" as drop
    import "outro.swr" as outro
    
    // Arrange sections
    arrangement {
        play: intro + verse + drop + verse + drop*2 + outro
    }
    
    // Global automation
    automation {
        master.volume: [
            0 bars: 0.0,
            1 bar: 0.9,
            32 bars: 0.9,
            32 bars + 2 beats: 0.0
        ]
    }
    
    // Output settings
    output {
        render: "media/renders/full_track.wav"
        stems: "media/stems/"
        
        normalize: true
        limiter: -0.3dB
    }
}
```

### Effect Processing

```
effect_chain "bass_fx" {
    eq {
        low: +3dB
        mid: -2dB
        high: +1dB
    }
    
    distortion {
        type: saturation
        amount: 0.4
        mix: 0.6
    }
    
    compression {
        threshold: -12dB
        ratio: 4
        attack: 10ms
        release: 100ms
    }
    
    delay {
        time: 1/8 dotted
        feedback: 0.3
        mix: 0.2
    }
    
    reverb {
        size: 0.3
        damping: 0.6
        mix: 0.1
    }
}
```

### Modulation System

```
modulation "filter_movement" {
    source: lfo {
        type: sine
        rate: 1/4
        phase: 0.25
    }
    
    target: synth("wobble_bass").filter.cutoff
    amount: 0.7
    range: [200Hz, 2000Hz]
}

automation "volume_fade" {
    target: track("bass").volume
    points: [
        0:     0.0,
        1 bar: 0.8,
        2 bar: 0.5,
        4 bar: 0.9
    ]
    curve: exponential
}
```
