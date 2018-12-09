
def read_input(filename):

    pairs = []
    with open(filename) as f:
        for line in f:
            a = line.split("Step ")[1].split(" ")[0]
            b = line.split("step ")[1].split(" ")[0]
            pairs.append((a, b))

    return pairs

def all_steps(pairs):

    steps = set([])
    for (a, b) in pairs:
        steps.add(a)
        steps.add(b)

    return steps

def solve_1(pairs):

    steps = all_steps(pairs)
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

def solve_2(pairs, workers, time_offset):

    remaining_steps = all_steps(pairs)
    ongoing = []
    t = 0

    while t == 0 or ongoing:

        # Advance to any finishing job(s)
        ongoing.sort(key = lambda x: x[1])
        if ongoing:
            t = ongoing[0][1]

            # Split out finishing jobs from ongoing
            finishing = [job[0] for job in ongoing if job[1] == t]
            ongoing = [job for job in ongoing if job[1] != t]
            for f in finishing:
                remaining_steps.remove(f)

            # Remove any dependencies for finishing jobs
            pairs = [(a, b) for (a, b) in pairs if a not in finishing]

        # Find jobs that have no dependencies on finished jobs
        can_start = set(remaining_steps)
        # Remove jobs that have a dependency
        for (_, b) in pairs:
            if b in can_start:
                can_start.remove(b)

        # Remove jobs that are already ongoing
        for (a, _) in ongoing:
            if a in can_start:
                can_start.remove(a)

        # Produce sorted list
        can_start = sorted(list(can_start))

        # If there are workers available, add new jobs
        for job in can_start[:workers-len(ongoing)]:
            ongoing.append((job, t + ord(job) - 64 + time_offset))

        print(ongoing, t)


    return t





if __name__ == '__main__':
    pairs = read_input("input")
    print(solve_1(pairs))
    print(solve_2(pairs, 5, 60))
