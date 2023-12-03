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
:(@e=~"@")||(@e=~"*")||(@e=~"#")||(@e=~"-")||(@e=~"/")||(@e=~"=")||(@e=~"%")||(stridx(@e,"$") >= 0)||(@e=~"+")
<Enter>
) ? "normal ver." : 0
`a
q
"this worsk to remove a number if it doesnt have a symbol around it but i just
"dont know how to turn run it on the start of every number
"also doesnt work on single digit or number at the border, will work on more
"tmorrow"
