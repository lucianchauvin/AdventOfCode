:%s/one/one1one/g <ENTER>
:%s/two/two2two/g <ENTER>
:%s/three/three3three/g <ENTER>
:%s/four/four4four/g <ENTER>
:%s/five/five5five/g <ENTER>
:%s/six/six6six/g <ENTER>
:%s/seven/seven7seven/g <ENTER>
:%s/eight/eight8eight/g <ENTER>
:%s/nine/nine9nine/g <ENTER>
gg 0
:%s/[a-z]*//g <ENTER>
:%s/\n\([1-9]\)\n/\r\1x\1\r/g <ENTER>
:%s/\n\([1-9]\)\n/\r\1x\1\r/g <ENTER>
:%s/\n\(\([1-9]\)\([1-9]\)\)\n/\r\2x\3\r/g <ENTER>
:%s/\n\(\([1-9]\)\([1-9]\)\)\n/\r\2x\3\r/g <ENTER>
gg 0
qq lvehdj0 q
VG
:normal @q <ENTER>
:%s/\n/+/g <ENTER>
$
x
V
c
<CTRL-r> =
<CTRL-V>
<ENTER>
<ESC>
:wq <ENTER>
