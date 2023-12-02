:%s/one/one1one/g 
:%s/two/two2two/g 
:%s/three/three3three/g 
:%s/four/four4four/g 
:%s/five/five5five/g 
:%s/six/six6six/g 
:%s/seven/seven7seven/g 
:%s/eight/eight8eight/g 
:%s/nine/nine9nine/g 
:%s/[a-z]*//g 
:%s/\n\([1-9]\)\n/\r\1x\1\r/g 
:%s/\n\([1-9]\)\n/\r\1x\1\r/g 
:%s/\n\(\([1-9]\)\([1-9]\)\)\n/\r\2x\3\r/g 
:%s/\n\(\([1-9]\)\([1-9]\)\)\n/\r\2x\3\r/g 
:normal gg
:normal qq 
:normal 0
:normal lvehdj0 
:normal q
:normal VG
:normal @q 
:%s/\n/+/g 
:normal $ 
:normal x
:normal 0
:normal c$
<ctrl-r>=<ctrl-r>"<cr><esc>
