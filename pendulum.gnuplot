plot 'pendulum_exact.dat' using 1:2 with points pointtype 5 pointsize 0.5 linecolor 1
#replot 'pendulum_approx.dat' using 1:2 with points pointtype 5 pointsize 0.5 linecolor 2

pause -1 "Enter to quit"
set terminal postscript eps enhanced color solid "Times-Roman,12"
