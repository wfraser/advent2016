import sys

screen = [[False for x in range(6)] for y in range(50)]
count = 0

while True:
    line = sys.stdin.readline().rstrip()
    if not line: break

    parts = line.split()
    if parts[0] == "rect":
        (x, y) = parts[1].split("x")
        x = int(x)
        y = int(y)

        for i in range(x):
            for j in range(y):
                screen[i][j] = True

    elif parts[0] == "rotate":
        assert parts[3] == "by"
        n = int(parts[4])

        if parts[1] == "row":
            (_, y) = parts[2].split("=")
            y = int(y)

            row = [False] * 50
            for i in range(50):
                dest = i + n
                if dest >= 50:
                    dest -= 50
                row[dest] = screen[i][y]
            for i in range(50):
                screen[i][y] = row[i]

        elif parts[1] == "column":
            (_, x) = parts[2].split("=")
            x = int(x)

            col = [False] * 6
            for i in range(6):
                dest = i + n
                if dest >= 6:
                    dest -= 6
                col[dest] = screen[x][i]
            for i in range(6):
                screen[x] = col

        else:
            raise Exception("invalid rotation {}".format(parts[1]))

    else:
        raise Exception("invalid command {}", parts[0])

for y in range(6):
    for x in range(50):
        if screen[x][y]:
            print("#", end="")
            count += 1
        else:
            print(".", end="")
    print()
print(count)
