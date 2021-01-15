def dance(letters, ops):
    for op in ops:
        if op[0] == 'p':
            index_a, index_b = op[1:].split('/')
            index_a = letters.index(index_a)
            index_b = letters.index(index_b)
            letters[index_a], letters[index_b] = letters[index_b], letters[index_a]
        elif op[0] == 'x':
            index_a, index_b = op[1:].split('/')
            index_a = int(index_a)
            index_b = int(index_b)
            letters[index_a], letters[index_b] = letters[index_b], letters[index_a]
        else:
            split = len(letters) - int(op[1:])
            letters = letters[split:] + letters[:split]
    return letters

def main():
    ops = []
    with open('input', 'r') as f:
        for line in f.readlines():
            ops = line.strip().split(',')

    original_letters = list('abcdefghijklmnop')
    letters = original_letters[:]

    limit = 1000000000
    repeats = 0
    for i in range(0, limit):
        letters = dance(letters, ops)
        if letters == original_letters:
            repeats = i
            break

    n = limit % (repeats + 1)
    for _ in range(0, n):
        letters = dance(letters, ops)
    print(''.join(letters))

main()