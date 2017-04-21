scaleless_music
=====

[![Join the chat at https://gitter.im/tiborgats/scaleless_music](https://badges.gitter.im/tiborgats/scaleless_music.svg)](https://gitter.im/tiborgats/scaleless_music?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
*A library for building music with precise harmonies, without the limitations of any musical scale.*

[![Crates.io](https://img.shields.io/crates/v/scaleless_music.svg)](https://crates.io/crates/scaleless_music) [![Linux & OS X build status](https://img.shields.io/travis/tiborgats/scaleless_music.svg?label=Linux%20build)](https://travis-ci.org/tiborgats/scaleless_music) [![Windows build status](https://img.shields.io/appveyor/ci/tiborgats/scaleless-music.svg?label=Windows%20build)](https://ci.appveyor.com/project/tiborgats/scaleless-music) [![Dependency Status](https://dependencyci.com/github/tiborgats/scaleless_music/badge)](https://dependencyci.com/github/tiborgats/scaleless_music) [![Crates.io](https://img.shields.io/crates/l/scaleless_music.svg)](https://github.com/tiborgats/scaleless_music/blob/master/COPYING) [![Documentation](https://docs.rs/scaleless_music/badge.svg)](https://docs.rs/scaleless_music) [![Join the chat at https://gitter.im/tiborgats/scaleless_music](https://badges.gitter.im/tiborgats/scaleless_music.svg)](https://gitter.im/tiborgats/scaleless_music?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

:construction: It is in a very early stage, and will change a lot as soon as I have some free time. I would be happy to work on it full-time, but I need money to make a living. If you have any idea how to fund this project, please share it with me.

# Overview

I was always annoyed by the slightly false notes that came out from electronic musical instruments. When I wanted to compose some overtone flute music I also realized, that music composing software (MIDI editors) are limited to the western chromatic music scale. They miss not only some harmonies of the overtone flute, but also many of those, which are present in other types of music (e.g. Arabic). And so this project was born...

The aim of this software is to create music according to the following perfectionist principles:

1. Music shall contain only [harmonic](https://en.wikipedia.org/wiki/Harmony) intervals between notes.
2. All harmonic interval variations shall have the possibility to be created.

The first rule means that the music shall not contain false sounds.
> Pure intervals are important in music because they naturally tend to be perceived by humans as "consonant": pleasing or satisfying. Intervals not satisfying this criterion, conversely, tend to be perceived as unpleasant or as creating dissatisfaction or tension. ([Wikipedia](https://en.wikipedia.org/wiki/Just_intonation))

The second rule provides the freedom to use harmonies which are missing from the [chromatic scale](https://en.wikipedia.org/wiki/Chromatic_scale).

The realization of these conditions is mathematically impossible when using musical [scales](https://en.wikipedia.org/wiki/Scale_(music)). In this new "scaleless" concept, the frequencies of the notes are not chosen from a fixed set (or scale), they are calculated dynamically, based on the relation of the previous notes to achieve pure intervals.

Counterexample: the popular [equal temperament](https://en.wikipedia.org/wiki/Equal_temperament) (used by [MIDI](https://en.wikipedia.org/wiki/MIDI)) contains only one type of pure harmonic interval: the octave, all the other frequency intervals are slightly disharmonic in it (multiplies of ¹²√2). It is also limited to a small set of intervals. For a deeper understanding of the problem of equal temperament and musical scales in general, you can read about the [just intonation](https://en.wikipedia.org/wiki/Just_intonation) approach.

## Sound synthesis
**scaleless_music** uses [additive synthesis](https://en.wikipedia.org/wiki/Additive_synthesis), with an additional rule:
* Frequency can be time-varying, but the intervals must remain harmonic.

Note: this kind of synthesis is very resource demanding. So, for real-time sound generation smaller sample rate (eg. 48kHz) and lower number of overtones are desirable (to prevent buffer underrun). This can change after the speed optimization of the algorithm.

Later on I would like to complement it with [sample-based synthesis](https://en.wikipedia.org/wiki/Sample-based_synthesis). But it must be used carefully, because of the following possible problems:
* It can contain noise and disharmonic intervals.
* It can contain echoes, effects of the recording space (which interfere with the space we want to add to it).

Henceforward, I plan to create a tool for analyzing recorded samples, finding closest mathematical representation, and building harmonic sound structures with similar output. This way we can eliminate noise and have the option to use precise 3D spacial effects.

# Installation
**scaleless_music** can be built with different sound output backends:
- without backend: `cargo build`
- with PortAudio: `cargo build --features "be-portaudio"`. If for some reason the building of [rust-portaudio](https://github.com/RustAudio/rust-portaudio) fails, you can check it's [README](https://github.com/RustAudio/rust-portaudio/blob/master/README.md) for further instructions.
- with SDL2: `cargo build --features "be-sdl2"`
- ~~with [rsoundio](https://github.com/klingtnet/rsoundio) ([libsoundio](http://libsound.io/)):~~ `cargo build --features "be-rsoundio"` - not available yet

## [Examples](https://github.com/tiborgats/scaleless_music/tree/master/examples)

## [Documentation](https://tiborgats.github.io/scaleless_music/)

# Todo Items
- [ ] thread safety
- [ ]  basic effects, building blocks of music structure
  - [ ] note
  - [ ] amplitude functions
    - [x] [exponential decay](https://en.wikipedia.org/wiki/Exponential_decay)
    - [x] faders
    - [x] [tremolo](https://en.wikipedia.org/wiki/Tremolo) - as sinusoidal variation of amplitude
    - [ ] [equal-loudness contour](https://en.wikipedia.org/wiki/Equal-loudness_contour)
  - [ ] frequency functions
    - [x] [vibrato](https://en.wikipedia.org/wiki/Vibrato)
    - [ ] linearly changing (ascending or descending) pitch
  - [ ] sequences of notes, rhythm functions
  - [x] polyphony (mixer)
  - [ ] smooth start and end of sine waves, which are physically possible (no zero time jumps in amplitude, to avoid infinite acceleration of the speaker membrane)
- [ ] speed optimization of the playback routines
  - [x] benchmark routines
  - [ ] parallel processing, SIMD
- [ ] backends for sound output
  - [x] [rust-portaudio](https://github.com/RustAudio/rust-portaudio)
  - [x] [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2)
  - [ ] [rsoundio](https://github.com/klingtnet/rsoundio)
- [ ] OS support
  - [x] Linux
  - [x] Windows
  - [ ] OS X
  - [ ] Android
  - [ ] iOS
- [ ] test coverages
- [ ] more examples

# Future work
- [ ] file format, parser
- [ ] converter from chromatic scale (eg. midi file) formats to scaleless music structure
- [ ] a software for finding the closest mathematical representation of sound samples (from real instruments)
- [ ] editor, with a correct, user friendly visual representation of music structure (not sheet music)
- [ ] handling some low-latency input devices
  - [ ] midi keyboard
- [ ] 3D audio space, instrument location effects, echo
- [ ] a lot of other stuff
