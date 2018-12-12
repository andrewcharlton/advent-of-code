from collections import defaultdict


def read_input():

    points = []
    with open("test_input") as f:
        for line in f:
            x, y = [int(x) for x in line.split(", ")]
            points.append((x, y))

    # Normalise so mininum value of x and y is 1
    min_x = min([p[0] for p in points])
    min_y = min([p[1] for p in points])

    return {(p[0] - min_x + 1, p[1] - min_y + 1): i+1 for (i, p) in enumerate(points)}


def make_grid(points):

    max_x = max([p[0] for p in points.keys()])
    max_y = max([p[1] for p in points.keys()])
    print("Grid size:", max_x, "x", max_y)

    grid = [[(0, False) for i in range(max_y + 2)] for y in range(max_x + 2)]

    while points:
        current = {}

        # Set grid for previous iterations's points
        for ((x, y), i) in points.items():
            grid[x][y] = (i, True)

        for ((x, y), i) in points.items():
            if i == 0:
                continue

            for (m, n) in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)]:

                # If on an edge continue
                if m < 0 or m > max_x + 1 or n < 0 or n > max_y + 1:
                    continue

                # If already assigned, skip
                try:
                    if grid[m][n][1]:
                        continue
                except:
                    print(m, n)

                if (m, n) in current and current[(m, n)] != i:
                    current[(m, n)] = 0
                else:
                    current[(m, n)] = i

        points = current

    return grid


def sum_areas(grid):
    areas = defaultdict(int)

    for row in grid:
        for (p, _) in row:
            areas[p] += 1

    return areas


def edge_pieces(grid):

    edges = set([p[0] for p in grid[0]] + [p[0] for p in grid[-1]])
    for row in grid:
        edges.add(row[0][0])
        edges.add(row[-1][0])

    return edges


def solve_1():

    points = read_input()
    grid = make_grid(points)
    print(grid)

    areas = sum_areas(grid)
    print(areas)

    edges = edge_pieces(grid)
    print(edges)

    answer = max([v for (k, v) in areas.items() if k not in edges])
    print(answer)


def solve_2():

    # points = {
    #     (1, 1): 'a',
    #     (1, 6): 'b',
    #     (8, 3): 'c',
    #     (3, 4): 'd',
    #     (5, 5): 'e',
    #     (8, 9): 'f',
    # }
    points = read_input()

    count = 0

    max_x = max([p[0] for p in points.keys()])
    max_y = max([p[1] for p in points.keys()])

    for x in range(max_x):
        for y in range(max_y):

            total = 0
            for (m, n) in points.keys():
                total += abs(x - m) + abs(y - n)

            if total < 10000:
                count += 1

    print(count)


if __name__ == "__main__":
    solve_1()
    solve_2()
