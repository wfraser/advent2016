import re
import sys

pattern = re.compile('^(.*)-([0-9]+)\[([^\]]+)\]$')

sum = 0
while True:
    line = sys.stdin.readline().rstrip()
    if not line: break
    m = pattern.match(line)
    (room, sector, checksum) = m.group(1, 2, 3)
    freq = {}
    for c in room:
        if c == "-": continue
        if c in freq:
            freq[c] += 1
        else:
            freq[c] = 1
    
    s = sorted(freq.items(), key=lambda item: item[0])
    s = sorted(s, key=lambda item: item[1], reverse=True)
    expected_cksum = ""
    for pair in s:
        expected_cksum += pair[0]
        if len(expected_cksum) == 5:
            break

    if expected_cksum == checksum:
        sector_num = int(sector)
        sum += sector_num

print(sum)
