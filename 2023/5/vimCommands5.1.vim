gg 0
V
:'<,'>s/ \(\d*\)/\r\1/g
gg 0 j
V } k
:sort n
/seed-to-soil
j
V } k
:'<,'>s/\(\d*\) \(\d*\)/\2 \1/
/seed-to-soil 
j V } kA
:sort n
" delete the lines starting with seed
gg 0
V ) ) ) k
:sort n
dd
"where a stores the starting value and b stores the desitnation
qq
cw^R=^R"-^Ra+^Rb^M^[0j
q
"run the macro for each block, setting the a and b registers each time (could
"make a macro for this)
"delete all the lines that arent outputs
"if you know whats going on repeat this for all the boxes or make a macro
