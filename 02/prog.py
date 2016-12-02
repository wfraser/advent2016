import sys

keypad1 = [['1','2','3'],
           ['4','5','6'],
           ['7','8','9']]
keypad2 = [[None,None,'1', None,None],
           [None,'2', '3', '4', None],
           ['5', '6', '7', '8', '9' ],
           [None,'A', 'B', 'C', None],
           [None,None,'D', None,None]]

directions = { 'U': (-1,0), 'D': (1,0), 'R': (0,1), 'L': (0,-1) }

def move(keypad, pos, direction):
    move = directions[direction]
    next_pos = (pos[0]+move[0], pos[1]+move[1])
    if next_pos[0] < 0 or next_pos[1] < 0 \
            or next_pos[0] >= len(keypad) or next_pos[1] >= len(keypad) \
            or not keypad[next_pos[0]][next_pos[1]]:
        return pos
    else:
        return next_pos

code1 = ""
code2 = ""
while True:
    line = sys.stdin.readline().rstrip()
    if not line: break

    pos1 = (1,1)
    pos2 = (2,0)
    for c in line:
        pos1 = move(keypad1, pos1, c)
        pos2 = move(keypad2, pos2, c)

    code1 += keypad1[pos1[0]][pos1[1]]
    code2 += keypad2[pos2[0]][pos2[1]]

print(code1)
print(code2)
