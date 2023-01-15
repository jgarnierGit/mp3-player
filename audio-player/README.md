# Audio-player crate

Provides a wrapper for audio player & metadata / tags parser

# Main depedency
- [Symphonia](https://github.com/pdeljanov/Symphonia)

# How to use

```bash
# Play audio file
audio-player -i /path/to/file -p

# Print all metadata / tags file
audio-player -i /path/to/file -a

# Print targeted metadata / tags file, cumulative i.e ["duration","frameRate","channels","genre"]
audio-player -i /path/to/file -t <TAG>

# Get audio data array in memory
audio-player -i /path/to/file --full-audio-sample

# Get audio data sample by sample in memory. /!\ not synchronized with a player
audio-player -i /path/to/file --live-audio-sample
```

# Known limitations
## Windows:

 - Hardcoded frame rate at 48000Hz
https://github.com/pdeljanov/Symphonia/issues/43

# Installation / Launch
see [parent README](https://github.com/jgarnierGit/mp3-player/blob/develop/README.md)