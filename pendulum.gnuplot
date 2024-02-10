set xlabel "Time" font "Arial,12"
set ylabel "Angle" font "Arial,12"
set title "Pendulum Angle vs Time" font "Arial,12"

plot 'pendulum.dat' using 1:2 with points pointtype 5 pointsize 0.5 linecolor 1

pause -1 "Press Enter key to quit"
set terminal postscript eps enhanced color solid "Arial,12"
