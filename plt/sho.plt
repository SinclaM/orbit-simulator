set term png size 1792, 1120
set output 'img/sho.png'

p 'data/sho.dat' u 2:4
