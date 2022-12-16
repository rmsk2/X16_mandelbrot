# On MacOS use make MAC=1
# On Linux simply use make

all: mandelbr.prg

ifdef MAC
ACME=../acme/acme
WORKDIR=/Users/martin/data/X16_mandelbrot
else
ACME=acme
WORKDIR=.
endif

clean:
	rm $(WORKDIR)/mandelbr.prg
	rm $(WORKDIR)/mandelbr.txt

mandelbr.prg: main.a arith16.a string.a vera.a mandelhelp.a zeropage.a
	$(ACME) -l $(WORKDIR)/mandelbr.txt $(WORKDIR)/main.a
