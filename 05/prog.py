import hashlib
import sys

def hash(key, n):
    h = hashlib.md5()
    h.update(key.encode())
    h.update("{}".format(n).encode())
    return h.hexdigest()

prefix = sys.stdin.readline().rstrip()

answer1 = ""
answer2 = {}
foundFive = False
n = 0
while True:
    out = hash(prefix, n)
    if out.startswith("00000"):
        if len(answer1) < 8:
            answer1 += out[5]
        if out[5].isdigit():
            pos = int(out[5])
            if pos < 8 and not pos in answer2:
                answer2[pos] = out[6]
        print(out)
        if len(answer2) == 8:
            break
    if n % 100000 == 0:
        sys.stdout.write(".")
        sys.stdout.flush()
    n += 1

print(answer1)

answer2_str = ""
for i in range(8):
    answer2_str += answer2[i]
print(answer2_str)
