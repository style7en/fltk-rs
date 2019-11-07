# fltk-rs

Rust bindings for the FLTK GUI library.
Still very barebones, undocumented, untested and not at all production ready! Segfaults on release builds!

The FLTK gui library is a crossplatform lightweight C++ library which can be linked to statically (LGPL) to produce small and fast binaries. 

To build, just run:
```
$ cargo build
```


## Dependencies

On Windows and Mac OS X, normally no external dependencies are needed. For Linux, X11 development headers need to be installed. For Debian-based distrobution, that means running:
```
$ sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev
```


## Examples

To run the examples: 
```
$ cargo run --example editor
$ cargo run --example calculator
$ cargo run --example gallery
$ cargo run --example button
$ cargo run --example hello
```

![alt_test](screenshots/gallery.jpg)
![alt_test](screenshots/calc.jpg)
![alt_test](screenshots/editor.jpg)


## Currently implemented widgets

Most common widgets are implemented: 
- Button
- RadioButton
- ToggleButton
- RoundButton
- CheckButton
- LightButton
- RepeatButton
- Native FileDialog
- Frame (Fl_Box)
- Window
- DoubleWindow
- Group
- Pack
- Tabs
- Scroll
- Tile
- TextDisplay (needs more work)
- TextEditor (needs more work)
- Input, IntInput, FloatInput, MultilineInput
- Output, MultilineOutput
- MenuBar
- MenuItem
- Choice (dropdown list)
- Slider, ValueSlider
- Dial
- Counter
- Scrollbar
- Roller

The implementation isn't complete no less. Customized event handling is not implemented.

## Todo

- Support customized event handling
- Complete widget set
- Better documentation
- Better testing
- Add image support

## Contributions

Contributions are very welcome!
