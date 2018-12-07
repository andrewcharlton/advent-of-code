
def read_input(filename):

    pairs = []
    with open(filename) as f:
        for line in f:
            a = line.split("Step ")[1].split(" ")[0]
            b = line.split("step ")[1].split(" ")[0]
            pairs.append((a, b))

    return pairs


def solve_1(pairs):

    steps = set([])
    for (a, b) in pairs:
        steps.add(a)
        steps.add(b)

    order = []

    while pairs:
        can_start = set(steps)
        for (_, b) in pairs:
            if b in can_start:
                can_start.remove(b)

        candidate = min([s for s in steps if s in can_start])
        order.append(candidate)
        steps.remove(candidate)

        pairs = [(a, b) for (a, b) in pairs if a != candidate]

    return order + sorted([s for s in steps])


if __name__ == '__main__':
    pairs = read_input("input")
    print(solve_1(pairs))
