import sys
from itertools import combinations
import math

def read_file_to_2d_array(filename):
    with open(filename, "r", encoding="utf-8") as f:
        return [list(line.rstrip("\n")) for line in f]  # Convert each line to a list of characters
       
def read_file(name):
    with open(name, "r") as file:
        data = file.read()
    return data

def check_limits(anti, rows, cols):
    return anti[0] >=0 and anti[0] < cols and anti[1] >= 0 and anti[1] < rows

def generate_antinodes(nodes, rows, cols):
    antis = []
    for key in nodes:
        for a, b in combinations(nodes[key], 2):
            #print(a,b)
            dx,dy = a[0]-b[0], a[1]-b[1]
            #print(dx,dy)
            anti = (a[0]+dx, a[1]+dy)
            if check_limits(anti, rows, cols):
                antis.append(anti)
            anti = (b[0]-dx, b[1]-dy)
            if check_limits(anti, rows, cols):
                antis.append(anti)
    # this will remove duplicates
    return len(list(set(antis)))

# now I can start at 0,yint and walk each column and see if y is an integer (or close to it)
def get_nodes_on_line(slope, yint, rows, cols):
    antis = []
    x = 0
    y = yint
    while x < cols:
        # check that y is on an integer value (nodes must be integers)
        if math.isclose(y, round(y), abs_tol=1e-10):
            y = round(y)
            # check that y is in the range of 0..rows
            if y >= 0 and y < rows:
                anti = (x, y)
                antis.append(anti)
        x += 1
        y += slope 
    return antis
    

def generate_anti_with_resonance(nodes, rows, cols):
    antis = []
    for key in nodes:
        for a, b in combinations(nodes[key], 2):
            #print(a, b)
            slope = (a[1]-b[1]) / (a[0]-b[0])
            yint = a[1] - slope*a[0]
            #print(f"slope: {slope} yint: {yint}")
            antis.extend(get_nodes_on_line(slope, yint, rows, cols))
    # this will remove duplicates
    return len(list(set(antis)))
       
if __name__ == "__main__":
    part2 = sys.argv[2] == "part2" if len(sys.argv) > 2 else False
    dev = sys.argv[3] == "dev" if len(sys.argv) > 3 else False

    part1answer = 0

    data = read_file_to_2d_array(sys.argv[1])

    # start with an empty hash map of characters that map to coordinates
    nodes = {}

    #print(f"data:  {data}")

    # first thing is to go through the data row by row and column by column and if I find a
    # alphanumeric character there, if it's new, add it as a key, if the key already exists in the 
    # hashmap, append the current coordinates to the list of coordinates for that key
    for i, row in enumerate(data):
        for j, val in enumerate(row):
            if val.isalnum():
                if val not in nodes:
                    nodes[val] = [(j,i)]
                else:
                    nodes[val].append((j,i))

    #print(f"nodes: {nodes}")
    part1answer = generate_antinodes(nodes, len(data), len(data[0]))

    print(f"Part 1: {part1answer}")

    part2answer = generate_anti_with_resonance(nodes, len(data), len(data[0]))

    print(f"Part 2: {part2answer}")
