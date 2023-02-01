
# code2text - concurrent  version
 generate random words from text files or source code

use:

    -i filename.txt
    -u only find underscore words
    -n how many words to find
    -l how long each word must be
    -m maximum length of each word (optional)
    -o output to filename
    -s sort output list of words
    -w list all collected words
    -r replacement with collected words .txt

how to use --replace -r
create a text file with some text and use %1 to replace with first word %2 replace with second word etc.
Example:

The world is a %1 and Rust is great %2.
Here is the %3.
