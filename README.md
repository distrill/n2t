# n2t
I am working through the [Nand to Tetris textbook](https://www.amazon.com/Elements-Computing-Systems-Building-Principles/dp/0262640686/ref=sr_1_2)         


End of chapter exercises are broken up into respective folders.

### can we go deeper?
* We were given a number of primitive units beyond the nand gate: Register, Screen, Keyboard, ROM32 to name a few. How much of this can we actually roll on our own?
* We opted for a Harvard Architecture, rather than a Von Neumann one. The consequence of this was a simpler CPU, at the cost of having to either burn programs into the ROM, or use an emulator to load them for us. Can we implement a proper Von Neumann architecture and load programs on the fly?
* We build an assembler on another working computer. This is an obvious choice, but it would be really fun to try and bootstrap our initial assembler with machine code.
