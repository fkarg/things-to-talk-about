set terminal png transparent size 640,240
set size 1.0,1.0

set terminal png transparent size 640,480
set output 'commits_by_author.png'
set key left top
set yrange [0:]
set xdata time
set timefmt "%s"
set format x "%Y-%m-%d"
set grid y
set ylabel "Commits"
set xtics rotate
set bmargin 6
plot 'commits_by_author.dat' using 1:2 title "Felix Karg" w lines, 'commits_by_author.dat' using 1:3 title "Jenny Muenk" w lines, 'commits_by_author.dat' using 1:4 title "Anna Ostrovskaya" w lines, 'commits_by_author.dat' using 1:5 title "Jan Erik Sundermann" w lines, 'commits_by_author.dat' using 1:6 title "uuweq" w lines, 'commits_by_author.dat' using 1:7 title "uwjbq" w lines, 'commits_by_author.dat' using 1:8 title "jan.sundermann" w lines, 'commits_by_author.dat' using 1:9 title "jenny" w lines
