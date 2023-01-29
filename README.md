# Rust audio player

Personal project of an audio manager & player written in Rust


# Installation
Environment requirements :
 - Rustup - Toolchain management : https://www.rust-lang.org/tools/install

## On Ubuntu 22.04 :

`sudo apt  install -y build-essential`

restart then:

```sudo apt-get install -y pkg-config libfontconfig-dev libpulse-dev```

# Launch
To get a prod optimized executable, in project root :

    cargo build --release

Or for a quick iterative test, simply launch :

    cargo run
    
# Crates
Each internal crates are meant to be testable with a command line interface.
As project is not even in a 0.0.1 version, crates are subject to breaking changes :D
|        Crate     |                                  Description                       |
|------------------|--------------------------------------------------------------------|
|[`audio-player`]  |Interface for external audio player crate & metadata / tags parser  |
|[`audio-manager`] |File system visitor & tag aggregator                                |
|[`audio-analyzer`]|Audio spectrum analyzer                                             |


# Main Dependencies

 - [Symphonia](https://github.com/pdeljanov/Symphonia) Audio player & metadata parser
 - [Plotters](https://github.com/plotters-rs/plotters) Drawing data library

# Special Thanks

 - [spectrum-analyzer](https://github.com/phip1611/spectrum-analyzer) FFT educationals to go further on signal analysis
 - [audio-visualizer](https://github.com/phip1611/audio-visualizer) Plotter live examples
