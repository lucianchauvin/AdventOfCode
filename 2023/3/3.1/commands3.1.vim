"special chars that can exist inclue: @*#-/=%$+ 
"START THIS MACRO AT THE FIRST DIGIT OF A NUMBER
qq
ma
e
mb
`a
k
h
mc
`b
k
l
v
`c
"ay
`a
j
h
mc
`b
j
l
v
`c
"by
`a
h
"cyl
`b
l
"dyl
`a
:let @e='"'..@a..@b..@c..@d..'"'
:execute (!
<CTRL-r> =
(@e=~"&")||(@e=~"@")||(@e=~"*")||(@e=~"#")||(@e=~"-")||(@e=~"/")||(@e=~"=")||(@e=~"%")||(stridx(@e,"$") >= 0)||(@e=~"+")
<Enter>
) ? "normal ver." : 0
`a
q
"this worsk to remove a number if it doesnt have a symbol around it but i just
"dont know how to turn run it on the start of every number
"also doesnt work on single digit or number at the border, will work on more
"tmmorrow"
"manually fixed sides cause thats where macro dies, digits that need to be
"adde below
:/\</
qr
:try | exe "norm! @q" | endtry | exe "norm! n"
<Enter>
`an
q
0
n
"remove all the numbers around the outside that the exe fails for
"remove all the single digits that should be removed that dont get removed
"from the exe (I could have made another exe for one digits but too lazy
1000r
"add the good single digits back in
:%s/\W/\r/g
:g/^$/d
:%s/\n/+
$
x
0
c$
<CTRL-R> =
<CTRL-R> "
<Enter>
