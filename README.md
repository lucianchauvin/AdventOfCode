# AdventOfCode

## 2024
Functionally Rust.

## 2023
Attempting to do advent of code purely in neovim. No classical programming languages :). So basically doing the entire thing with just substitution(replacment), key macros, and the expression register for math. If you see python files I'm going to be solving the problem first in there so I can check my work while creating vim solution. What the *code* looks like:
```txt
:%s/\([a-z] \d*\)\n/\1 
:%s/\(b \d*\) g/\1\rg
:%s/\(g \d*\) r/\1\rr
:%s/\n\(\d*\)\n/\r\r\1\r
:%s/\([a-z] \d*\) /\1\r/g
:%s/\(b \d*\)\ng/\1 \r\rg
:%s/\(g \d*\)\nr/\1 \r\rr
:%s/\([a-z]\) \(\d*\)/\2 \1
qq
jV)k:sort! n^M)jV)k:sort! n^M)jV)k:sort! n^M))
q
10000@q
:%s/\(\d* [a-z]\)\n\n/\1\r\1\r\r
qq
jjV)djV)djV)dj
q
10000@q
:%s/ [a-z]//g
qq
dd)j
q
10000@q
:%s/ //g
qq
j0i<80>kb8^[j0i<80>kb*^[0jj
```

## Some things Ive learned
Not only doing it solely with vim is hard but the mental acrobatics you have to do to solve the problem in vim is extremly hard. In some senses it is also easier though. I came up with a way to do for loops and doing it in vim makes more sense than in a classical language. First you write a macro for what the for loop is going to do at each iteration. Then you figure out how many times you need to iterate and store that in a register. Then you go to where you want to start and do :normal "a@q where a is the register and q is the macro. It so far has been a really cool experience and having to come up with mind boggling solutions is really fun. 
