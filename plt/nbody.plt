set term qt font "Arial" size 1000, 1000
set xrange [-10:10]
set yrange [-10:10]
plot "data/bodies.dat" using 4:6
pause 0.01
reread
