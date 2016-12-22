import sys;

def pair_is_viable(a, b):
    return a[0] != 0 and a[0] <= b[1]

nodes = []
i = 0
while True:
    i += 1
    line = sys.stdin.readline().rstrip()
    if i <= 2: continue
    if not line: break
    words = line.split()
    used = int(words[2][:-1])
    avail = int(words[3][:-1])
    nodes += [(used, avail)]
print(nodes)

n = 0
for i in range(len(nodes)):
    for j in range(len(nodes)):
        if i != j and pair_is_viable(nodes[i], nodes[j]):
            n += 1
print(n)
