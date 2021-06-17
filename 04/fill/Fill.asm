// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// @SCREEN
// @8192
(LOOP)
	
    // if keyboard register is equal to 0, nothing is being pressed,
    // jump to fill everything white
    @KBD
    D=M
    @SET_WHITE
    D; JEQ
    
    // set color to black, jump past set white to FILL
    @color
    M=-1
    @FILL
    0; JMP
	
    (SET_WHITE)
    @color 
    M=0

    (FILL)
    // i = 8191, last element in screen we will need to fill
    @8191
    D=A
    @i
    M=D

    // we will work backwards, filling in screen + i and 
    // walking i backwards until < 0
    (FILL_LOOP)
    	// continue if i == 0
	@i
	D=M
	@CONT
	D; JLT

	// print = screen + i
	@SCREEN
	D=A
	@i
	D=D+M
	@print
	M=D

	// RAM[print] = color
	@color
	D=M
	@print
	A=M
	M=D

	// i = i-1
	@i
	D=M
	M=D-1
	@FILL_LOOP
	0; JMP

    (CONT)
    @LOOP
    0; JMP

(END)
