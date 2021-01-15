def main():
    letters = list('abcdefghijklmnop')

    ops = []
    with open('input', 'r') as f:
        for line in f.readlines():
            ops = line.strip().split(',')

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
    print(''.join(letters))

main()