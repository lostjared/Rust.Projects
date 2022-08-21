Another project to practice

Write down in a text file the movements you want the pixel to take
the program will interpret the file and move the pixel in the direction
specified in the text file. A few example scripts are in the scripts/ directory.

L stands for move Left
R stands for move Right
D stands for move Down
U stands for move Up
S stands for Set
C stands for color

	the format for Directions Direction:Steps
	teh format for setting the position is S:X,Y


so

	C:0
	R:1
	D:1
	L:1
	U:1

moves the pixel in a square with color red

or

	S:10,10
	C:1
	L:1
	L:1
	R:1
	R:1

sets the position to 10,10
sets color green
moves left by 1, left by 1, right by 1, right by 1

When the program reachs the end of the script it goes back up to the top and starts over
in a loop.

	Colors:
	0 - Red
	1 - Green
	2 - Blue



