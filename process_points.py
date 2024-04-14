with open("points.txt", "r") as f:
    lines = f.readlines()
points = set()

for line in lines:
    # print(line)
    if line == "\n":
        break
    x, y, z = list(map(float, line.split(" ")))
    points.add(
        (
            x,
            y,
            z,
        )
    )

print(len(points))
print(len(lines))
