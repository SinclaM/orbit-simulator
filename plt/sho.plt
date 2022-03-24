set term png size 1792, 1120
set output 'img/sho.png'

p 'data/euler.dat' u 2:4, 'data/rk2.dat' u 2:4, 'data/rk4.dat' u 2:4, 1000 * cos(sqrt(9.8) * x)
