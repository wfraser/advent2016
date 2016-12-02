import sys

up =    [1,2,3,
         1,2,3,
         4,5,6]
down =  [4,5,6,
         7,8,9,
         7,8,9]
left =  [1,1,2,
         4,4,5,
         7,7,8]
right = [2,3,3,
         5,6,6,
         8,9,9]

while True:
    line = sys.stdin.readline().rstrip()
    if not line: break

    number = 5
    for c in line:
        if c == 'U':
            number = up[number - 1]
        elif c == 'D':
            number = down[number - 1]
        elif c == 'L':
            number = left[number - 1]
        elif c == 'R':
            number = right[number - 1]
    sys.stdout.write("{}".format(number))
print("")
