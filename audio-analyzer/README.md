# Audio-analyzer crate

Spectrum analysis logic, for now focused on BPM computation.

Provide a visual debug tool chainable to audio player

# How to use

BPM computation can be obtained by external tools. For testing I personaly used [Plotters](https://aubio.org/download).

## Linux environment

installation and usage is quite straight-forward.

## Windows environment

[TL;DR]

 1. download [aubio-0.4.6-win64.zip](https://aubio.org/bin/0.4.6/aubio-0.4.6-win64.zip)
 2. convert your mp3 sample into wav using [audacity](https://www.audacityteam.org/download/)
 3. execute command line to get beats timestamp

 in aubio /bin

    aubiotrack.exe -i "MUSIC_PATH"

It is very painful experience to achieve mp3 spectrum analysis. Main reason is describe here [FR - "ffmpeg pour windows ca va couper"](https://linuxfr.org/users/roger21/journaux/ffmpeg-pour-windows-ca-va-couper).

### More details 

    aubio-0.4.6-win64-ffmpeg.zip require "Cantor" 3.4.12 dll ffmpeg version max whose are: 
    			* libavutil-55.dll
    			* libavformat-57.dll
    			* libswresample-2.dll
    			* libavcodec-57.dll

From there you have no choice but go get and compile the [ffmpeg sources](https://ffmpeg.org/download.html).

Or give up and convert your mp3 into wav to use dependency-free aubio executable .
 
 ## Mac environment
Not tested.

# Main depedency
 - [Plotters](https://github.com/plotters-rs/plotters) Drawing data library

# Installation / Launch
see [parent README](https://github.com/jgarnierGit/mp3-player/blob/develop/README.md)
