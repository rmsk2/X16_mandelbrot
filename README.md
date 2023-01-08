# General Information

This program is a further optimzed version of my [Mandelbrot set viewer for the C64](https://github.com/rmsk2/c64_mandelbrot) that
is intended for [Commander X16](https://www.commanderx16.com) (see also here on 
[GitHub](https://github.com/commanderx16)) and makes use of commander X16 features like:

- More RAM which offers the possibility to precompute more stuff
- More free zero page addresses which allow a broader use of more efficient addressing modes
- More colours to use in Hires mode (256 for the X16, 2 for the C64)
- New instructions for the 65C02 (well only `stz` and `bra`)

Currently the default section of the Mandelbrot set (see screenshot below) using the X16s full resolution of 320x240 
at a depth of 24 iterations (in total 717022 iterations) is calculated in about 12 minutes at close to 1000 iterations/sec. 
The  C64 version takes two and a half hours to calculate the same visualisation in a resolution of 320x200. Of course 
the biggest part of the speedup stems from the fact that the 65C02 in the X16 runs at 8MHz where the 6510 in a C64 is
clocked at 1MHz. But even if that is taken into account the X16 version is currently faster.

![](/result.png?raw=true "Example picture in hires mode")

You have to start the Commander X16 emulator with at least the following options: `-sdcard sdcard.img -rtc` for
all features to work. `LOAD` the program from SD card and type `RUN` to start it. Alternatively you can utilize 
the `-prg` option to load the program even if it is not stored on the SD-card  image.

# Building the software

You need the ACME macro assembler to assemble the program. A makefile is provided to simplify building the software.
Under MacOS you have to set the variable `MAC` (use `make MAC=1`) when calling the makefile and you have to adapt 
the variables `ACME` and `WORKDIR` to reflect the situation on your machine. Under Linux the makefile should run 
without changes as long as ACME is in your `PATH`.

# About my motivation for writing this program

Why does someone write a program (in machine language) for an 8-bit microprocessor that is nowadys only 
used in small embedded systems, that offers functionality which is on several levels (performance, 
visual appeal, ...) orders of magnitude worse than software written for the same purpose for modern 
systems? And on top of that the computer that the program is designed to run on does only exist to
a certain extent as only a few prototypes have been built yet.

The only reasons I can give are: I had fun doing it and it was nostalgic as it transported me back to
the days when another 6502 system was the center of at least my (home) computing universe. The commander
X16 (or to be more precise at the moment its emulator) strikes the right balance between new possibilities 
and a nostalgic familiarity with the Commodore family of home computers that allows all the people
that grew up with these computers to now write the programs that they did not or could not write in
the 80ies or 90ies.

# Using the program

When you start the program you can select whether you want 

1. to load a picture and its corresponding valuesfrom SD card 
2. to start a new calculation using the current values but with a different iteration depth
3. to reset to the default values and start a new calculation
4. start a new calculation using the current values
5. to load calculation paramaters and start a new calculation
6. to save the current calculation parameters
7. to exit again. 

An option is selected by pressing the key corresponding to the number.

![](/main_menu.png?raw=true "Main menu")

When you select option 1. the picture data and its associated parameters are loaded into RAM and then shown
on the screen. You can zoom into the picture by pressing `F5` (see *Zooming into the Mandelbrot set*) below or 
you can look at the parameters by pressing any other key.

When you select option 2. ,3., 4. or .5 a new picture is calculated. The calculation can be interrupted at any time
by pressing a key. If that key is `F5` you can zoom into the Mandelbrot set (see corresponding section below).
When any other key is pressed the parameters used for calculation are presented. If you then press `RETURN` the
calculation is stopped and the program returns to the main menu. Any other key resumes the calculation.

If all calculations for a picture have been performed the program waits for a key press while showing the
picture. You can either press `F5` to zoom further into the set (see below) or press `F7` to save the picture 
on SD card. If you press any other key the values used for the calculation are printed to the screen.

# Zooming into the Mandelbrot set

If `F5` is pressed you can select a new section of the Mandelbrot set. Use the following key commands to 
select the new section:

| Key | Function |
|-|-|
|Cursor Keys | Move upper left corner of new section |
| `F1` | Zoom in |
| `F3` | Zoom out |
| `F2`| Abort selection | 
| `RETURN` | Start calculation of new section |

The cursor keys can be used to move a rectangular frame of reversed pixels over the visualization. The frame
represents the currently selected new section. `F1` and  `F3` can be used to change the size of that frame. 
Pressing return starts the calculation of the selected subsection.

![](/zoom_frame.png?raw=true "Zooming in action")

Zooming in one level simply halves the stepping width in the complex plane in both directions. As this software uses
fixed point arithmentic with 24 bits after the comma this has the consequence that the maximum zoom level is
limited to 16. The theoretical maximum is 17. On zoom level 18 the stepping width in X and Y direction 
would become zero as the last nonzero bit would have been shifted out of the 24 bit "mantissa".

# Limitations at the moment

- Loading and saving pictures only works with a mounted SD-card image. I am at the moment not sure if there is anything
I can do about that.
- As I do not have access to real hardware and therefore have to use the X16 emulator it is not 100% sure that the program
performs in the same way on a real machine
- I have not yet tested whether my fixed point math routines are actually faster than the floating point routines
in the X16s math library and if they are really faster I don't know by how much.
- Currently the 32 bit multiplication uses the text book algorithm taught at school for manual multiplication. I could test
whether Karatsuba's algorithm speeds things up at least a little even in this (close to a corner) case.
