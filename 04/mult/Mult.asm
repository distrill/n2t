// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// n = RAM[0]
@R0
D=M
@n
M=D

// m = RAM[1]
@R1
D=M
@m
M=D

// sum = 0
@sum
M=0

(LOOP)
    // if m == 0, jump to end
    @m
    D=M
    @END
    D; JEQ

    // sum += n
    @n
    D=M
    @sum
    M=D+M

    // m -= 1
    @m
    M=M-1

    // jump to loop start
    @LOOP
    0; JMP
(END)
    // RAM[2] = sum
    @sum
    D=M
    @R2
    M=D
    @END
    0; JMP
