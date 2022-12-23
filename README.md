# General Information

This program is a further optimzed version of my [Mandelbrot set viewer for the C64](https://github.com/rmsk2/c64_mandelbrot) that
is intended for [Commander X16](https://www.commanderx16.com) and makes use of commander X16 features like:

- More RAM which offers the possibility to precalculate more stuff
- More free zero page addresses which allow a broader use of more efficient addressing modes
- More colours to use in Hires mode (256 for the X16, 2 for the C64)
- New instructions for the 65C02 (well only `stz` and `bra`)

**This is work in progress**. Currently the default section of the Mandelbrot set (see screenshot below) 
using the X16s full resolution of 320x240 at a depth of 24 iterations is calculated in roughly 14 minutes. 
The  C64 version takes two and a half hours to calculate the same visualisation in a resolution of 320x200. 
Of course the biggest part of the speedup stems from the fact that the 65C02 in the X16 runs at 8MHz where 
the 6510 in a C64 is clocked at 1MHz. But even if that is taken into account the X16 version is currently 
about 25% faster.

You need the ACME macro assembler to assemble the program. Use the `LOAD "FILENAME"` command followed
by `RUN` to start the program. Under MacOS you have to set the variable `MAC` (use `make MAC=1`) when 
calling the makefile and you have to adapt the variables `ACME` and `WORKDIR` to reflect the situation on 
your machine. Under Linux the makefile should run without changes as long as ACME is in your `PATH`.

![](/result.png?raw=true "Example picture in hires mode")

# Zooming into the Mandelbrot set

The calculation can be interrupted at any time by pressing a key. If `F5` is pressed you can select a new section of 
the Mandelbrot set. Use the following key commands to select the new section:

| Key | Function |
|-|-|
|Cursor Keys | Move upper left corner of new section |
| `F1` | Zoom in |
| `F3` | Zoom out |
| `RETURN` | Start calculation of new section |

The cursor keys can be used to move a rectangular frame of reversed pixels over the visualization. The frame
represents the currently selected new section. `F1` and  `F3` can be used to change the size of that frame. 
Pressing return starts the calculation of the selected subsection. If the  caclulation has finished you can 
also select a new subsection  after pressing `F5`. Pressing any other key ends the program.

Zooming in essence halves the stepping width in X and Y direction. As this software uses fixed point arithmetic 
this results in a loss of accuracy which at a certain point can lead to visual artifacts in the form of spuriously
colored pixels. In experiments these artifacts started to appear at zoom level 7 but did not affect the result
overly negative. At zoom level 8 the quality suffers visibly.


# Limitations at the moment

- No possibility to save the caclulated picture. As a primitive workaround all the parameters needed to recreate
the last picture are printed to the screen when the program shuts down.
- No possibilty to view a saved picture
- As I do not have access to real hardware and therefore have to use the X16 emulator it is not 100% sure that the program
performs in the same way on a real machine
- I have not yet tested whether my fixed point math routines are actually faster than the floating point routines
in the X16s math library and if they are really faster I don't know by how much.
