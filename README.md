This program is a further optimzed version of my [Mandelbrot set viewer for the C64](https://github.com/rmsk2/c64_mandelbrot) that
makes use of commander X16 features like:

- More RAM which offers the possibility to precalculate more stuff
- More free zero page addresses which allow a broader use of more efficient addressing modes
- More colours to use in Hires mode (256 for the X16, 2 for the C64)
- New instructions for the 65C02 (well only stz and bra)

**This is work in progress**. Currently the default section of the Mandelbrot set using the full resolution 
of 320x240 at a depth of 24 steps is calculated in roughly 14 minutes. The C64 version takes two and a
halve hours to calculate the same visualisation in a resolution of 320x200. Of course the biggest
part of the speedup stems from the fact that the 65C02 in the X16 runs at 8MHz where the 6510 in a
C64 is clocked at 1MHz. But even when that is taken into account the X16 version is currently about
25% faster.

You need the ACME macro assembler to assemble the program. Use the `LOAD "FILENAME"` command followed
by `RUN` to start the program.

Limitations at the moment:

- No user interface to change the section of the Mandelbrot set that is visualized
- As I do not have access to real hardware and therefore have to use the X16 emulator it is not 100% sure that the program
performs in the same way on a real machine
- I have not yet tested whether my fixed point math routines are actually faster than the floating point routines
in the X16s math library and if they are really faster by how much.
