Execute shell command for each item in a list
Example

use like this


	$ ls | grep .txt | proc_list "cat %f"

or


	$ ls | grep .txt > list.txt  && proc_list list.txt "cat %f"


%f stands for each line of the list. 
