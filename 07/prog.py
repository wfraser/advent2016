import re
import sys

def has_abba(s):
    for i in range(len(s) - 3):
        if s[i] == s[i+3] and s[i+1] == s[i+2] and s[i] != s[i+1]:
            return True
    return False

def has_aba_bab(outsides, insides):
    for text in outsides:
        for i in range(len(text) - 2):
            if text[i] == text[i+2] and text[i] != text[i+1]:
                if has_bab(insides, text[i], text[i+1]):
                    return True
    return False

def has_bab(insides, a, b):
    for text in insides:
        for i in range(len(text) - 2):
            if text[i] == b and text[i+1] == a and text[i+2] == b:
                return True
    return False

r = re.compile('[^\[\]]+')
count1 = 0
count2 = 0

while True:
    line = sys.stdin.readline().rstrip()
    if not line: break

    parts = r.findall(line)
    inside = True # inside brackets or not?
    valid = False
    for part in parts:
        inside = not inside
        if has_abba(part):
            if inside:
                valid = False
                break
            else:
                valid = True

    if valid:
        count1 += 1

    outsides = [parts[i] for i in range(0,len(parts),2)]
    insides  = [parts[i] for i in range(1,len(parts),2)]
    if has_aba_bab(outsides, insides):
        count2 += 1

print(count1)
print(count2)
