from math import sin

sinLUTFile = open('sin.txt', "wt")

for i in range(0, 1571):
    sinLUTFile.write(str(sin(i/1000)) + ', ')