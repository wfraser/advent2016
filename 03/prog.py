import sys

def triangle(a, b, c):
    return a + b > c and b + c > a and a + c > b

lines = []

possible1 = 0
possible2 = 0
while True:
    line = sys.stdin.readline().rstrip()
    if not line: break
    (a, b, c) = line.split()
    a = int(a)
    b = int(b)
    c = int(c)
    if triangle(a, b, c):
        possible1 += 1
    lines += [[a, b, c]]

for i in range(0, len(lines), 3):
    for j in range(3):
        a = lines[i][j]
        b = lines[i+1][j]
        c = lines[i+2][j]
        if triangle(a, b, c):
            possible2 += 1

print(possible1)
print(possible2)
