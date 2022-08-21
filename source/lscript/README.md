Another project to practice

Write down in a text file the movements you want the pixel to take
the program will interpret the file and move the pixel in the direction
specified in the text file. A few example scripts are in the scripts/ directory.

L stands for move Left
R stands for move Right
D stands for move Down
U stands for move Up
S stands for Set

	the format for Directions Direction:Steps
	teh format for setting the position is S:X,Y


so

	R:1
	D:1
	L:1
	U:1

moves the pixel in a square

or

	S:10,10
	L:1
	L:1
	R:1
	R:1

sets the position to 10,10
moves left by 1, left by 1, right by 1, right by 1

When the program reachs the end of the script it goes back up to the top and starts over
in a loop.
