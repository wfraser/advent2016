import sys

def turn(turn, direction):
    if turn == 'R':
        if direction == 3:
            return 0
        else:
            return direction + 1
    elif turn == 'L':
        if direction == 0:
            return 3
        else:
            return direction - 1
    else:
        raise Exception("invalid turn direction {}".format(turn))

def walk(pos, direction):
    if direction == 0:
        return (pos[0], pos[1]+1)
    elif direction == 1:
        return (pos[0]+1, pos[1])
    elif direction == 2:
        return (pos[0], pos[1]-1)
    elif direction == 3:
        return (pos[0]-1, pos[1])

locations = set()
pos = (0, 0)
direction = 0
visited_twice = False

input = sys.stdin.readline().rstrip()
for instruction in input.split(", "):
    direction = turn(instruction[0:1], direction)
    distance = int(instruction[1:])
    for _ in range(distance):
        pos = walk(pos, direction)
        if not visited_twice and pos in locations:
            visited_twice = True
            print("found it: {} ({} away)".format(pos, abs(pos[0]) + abs(pos[1])))
        locations.add(pos)
print("final location: {} ({} away)".format(pos, abs(pos[0]) + abs(pos[1])))
