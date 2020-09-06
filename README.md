# mp3chomp

`mp3chomp` is a command line tool to (want to) chomp trailing silent part of mp3 data.

Currently, it only shows possible trailing seconds where rest part is silent.
It keeps 5 seconds of silent trailing part for afterglow.

```console
$ mp3chomp --verbose foo.mp3
foo.mp3: 328.48678
```

Then, use `ffmpeg4` to chomp it.

```console
$ ffmpeg4 -to 328.48678 -i foo.mp3 bar.mp3
```
