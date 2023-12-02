:%s/Game //g
:%s/: /\r
:%s/\n\(\d*\)\n/\r\r\1\r
:%s/red/r/g
:%s/blue/b/g
:%s/green/g/g
:%s/; /\r/g
:%s/\(\n\d* g, \d* r\)\n/\1, 1 b\r/g
:%s/\(\n\d* r, \d* g\)\n/\1, 1 b\r/g
:%s/\(\n\d* r, \d* b\)\n/\1, 1 g\r/g
:%s/\(\n\d* b, \d* r\)\n/\1, 1 g\r/g
:%s/\(\n\d* b, \d* g\)\n/\1, 1 r\r/g
:%s/\(\n\d* g, \d* b\)\n/\1, 1 r\r/g
:%s/\(\n\d* r\)\n/\1, 1 g, 1 b\r/g
:%s/\(\n\d* g\)\n/\1, 1 r, 1 b\r/g
:%s/\(\n\d* b\)\n/\1, 1 r, 1 g\r/g
:%s/, /\r/g
:%s/\(\d*\) \([a-z]\)/\2 \1
gg 0
qq
j
V
)
k
:sort
)
j
q
10000@q
:%s/\([a-z] \d*\)\n/\1 
:%s/\(b \d*\) g/\1\rg
:%s/\n\(\d*\)\n/\r\r\1\r
:%s/\([a-z] \d*\) /\1\r/g
:%s/\(b \d*\)\ng/\1 \r\rg
:%s/\(g \d*\)\nr/\1 \r\rr
:%s/\([a-z]\) \(\d*\)/\2 \1
qq
jV)k:sort! n^M)jV)k:sort! n^M)jV)k:sort! n^M))
q
10000@q
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
q
10000@q
gg 0
qq
c$^R=^R"^M^[0jj
q
:%s/\(\d*\)\n/\1+
:%s/++/+/g
$
x
0
c$
<ctrl-r>=<ctrl-r>"<cr><esc>
