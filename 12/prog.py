import sys

registers = {'a': 0, 'b': 0, 'c': 0, 'd': 0}

def value(r, registers):
    try:
        v = int(r)
    except ValueError:
        v = registers[r]
    return v

def ex(instr, registers):
    if instr[0] == "cpy":
        v = value(instr[1], registers)
        registers[instr[2]] = v
        return 1
    elif instr[0] == "inc":
        registers[instr[1]] += 1
        return 1
    elif instr[0] == "dec":
        registers[instr[1]] -= 1
        return 1
    elif instr[0] == "jnz":
        v = value(instr[1], registers)
        if v != 0:
            return int(instr[2])
        else:
            return 1

instructions = []

while True:
    line = sys.stdin.readline().rstrip()
    if not line: break
    instructions += [line.split()]

def run(instructions, registers):
    i = 0
    while True:
        offset = ex(instructions[i], registers)
        i += offset
        if i >= len(instructions):
            break

run(instructions, registers)
print(registers['a'])

registers = {'a': 0, 'b': 0, 'c': 1, 'd': 0}
run(instructions, registers)
print(registers['a'])
