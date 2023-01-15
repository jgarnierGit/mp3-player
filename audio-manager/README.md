# Audio-manager crate

File system visitor & tag aggregator,
will be the main force for playlist manual edition

# Installation / Launch
see [parent README](https://github.com/jgarnierGit/mp3-player/blob/develop/README.md)

# How to use

```bash
# Count audio files recursively
audio-manager -i /path/to/lib -c

# Aggregate and count audio files by given tag i.e <AGGREGATE_TAG> in ["frameRate","channels","genre"]
audio-manager -i /path/to/lib -a <AGGREGATE_TAG>

# Filters audio files by metadata / tags i.e <FILTER_TAG>=genre <FILTER_VALUE>=rock
# is case sensitive and may take several minutes to process music library
audio-manager -i /path/to/file -f <FILTER_TAG> --filter-value <FILTER_VALUE>

```