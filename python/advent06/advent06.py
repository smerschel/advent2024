import sys
import re
import copy

UP = 1
RIGHT = 2
DOWN = 3
LEFT = 4

def read_file_to_2d_array(filename):
    with open(filename, "r", encoding="utf-8") as f:
        return [list(line.rstrip("\n")) for line in f]  # Convert each line to a list of characters
       
def find_carrot(data):
    for row in range(len(data)):
        for col in range(len(data[0])):
            if data[row][col] == "^":
                return col, row, UP  # Return column, row, and direction (0 = right)
            elif data[row][col] == "v":
                return col, row, DOWN
            elif data[row][col] == "<":
                return col, row, LEFT
            elif data[row][col] == ">":
                return col, row, RIGHT
    return None

def at_exit(x, y, columns, rows):
    return x < 0 or y < 0 or x >= columns or y >= rows

def next_to_exit(x, y, direction, columns, rows):
    if direction == UP:
        if y == 0:
            return True
    elif direction == DOWN:
        if y == rows - 1:
            return True
    elif direction == LEFT:
        if x == 0:
            return True
    elif direction == RIGHT:
        if x == columns - 1:
            return True
    return False

# you either turn right, or step forward
def mark_and_move(x, y, direction, data, mark):
    if mark:
        data[y][x] = "X"  # Mark the current position

    if not next_to_exit(x, y, direction, len(data[0]), len(data)):
        # check for wall
        if direction == UP:
            if data[y-1][x] == "#" or data[y-1][x] == "O":
                # turn right
                direction = RIGHT
                return x, y, direction
        elif direction == RIGHT:
            if data[y][x+1] == "#" or data[y][x+1] == "O":
                # turn down
                direction = DOWN
                return x, y, direction
        elif direction == DOWN:
            if data[y+1][x] == "#" or data[y+1][x] == "O":
                # turn left
                direction = LEFT
                return x, y, direction
        elif direction == LEFT:
            if data[y][x-1] == "#" or data[y][x-1] == "O":
                # turn up
                direction = UP
                return x, y, direction
    # move in the direction
    if direction == UP:
        y -= 1
    elif direction == DOWN:
        y += 1
    elif direction == LEFT:
        x -= 1
    elif direction == RIGHT:
        x += 1
    if at_exit(x, y, len(data[0]), len(data)):
        return x, y, 0
    return x, y, direction

def count_exes(data):
    count = 0
    for row in data:
        for cell in row:
            if cell == "X":
                count += 1
    return count

def add_obstruction(data, x, y, direction):
    locy = y
    locx = x
    if direction == UP:
        data[y-1][x] = "O"
        locy = y - 1
        locx = x
    elif direction == DOWN:
        data[y+1][x] = "O"
        locy = y + 1
        locx = x
    elif direction == LEFT:
        data[y][x-1] = "O"
        locy = y
        locx = x - 1
    elif direction == RIGHT:
        data[y][x+1] = "O"
        locy = y
        locx = x + 1
    return data, (locx, locy)

def run_sim(data, startrow, startcol, direction):
    curcol = startcol
    currow = startrow
    loopcount = 0
    while not at_exit(curcol, currow, len(data[0]), len(data)):
        curcol, currow, direction = mark_and_move(curcol, currow, direction, data, False)
        loopcount += 1
        if loopcount > (len(data) * len(data[0])):  
            return True
    return False
    
if __name__ == "__main__":
    part2 = sys.argv[2] == "part2" if len(sys.argv) > 2 else False
    dev = sys.argv[3] == "dev" if len(sys.argv) > 3 else False

    part1pos = 0
    part2count = 0

    data = read_file_to_2d_array(sys.argv[1])
    columns = len(data[0])
    rows = len(data)
    curcol, currow, direction = find_carrot(data)
    startcol = curcol
    startrow = currow
    startdir = direction
    print(f"Starting position: {curcol}, {currow}, {direction}")
    while not at_exit(curcol, currow, columns, rows):
        curcol, currow, direction = mark_and_move(curcol, currow, direction, data, True)
    part1pos = count_exes(data)
    print(f"Part 1: {part1pos}")

    if part2:
        # reset data
        data = read_file_to_2d_array(sys.argv[1])
        curcol, currow, direction = startcol, startrow, startdir
        # need to track obstructions to eliminate duplicates
        obstructions = []
        # we only have to try adding obstructions along the path
        # so for every step along the path, add an abstruction and run a sim to see if it gets stuck
        while not at_exit(curcol, currow, columns, rows):
            newdata = copy.deepcopy(data)
            if not next_to_exit(curcol, currow, direction, len(data[0]), len(data)):
                newdata, obs = add_obstruction(newdata, curcol, currow, direction)
                stuck = run_sim(newdata, startrow, startcol, startdir)
                if stuck:
                    # print newdata
                    print(f"Stuck at: {curcol}, {currow}, {direction}")
                    #for row in newdata:
                    #    print("".join(row))
                    if obs not in obstructions:
                        obstructions.append(obs)
                        part2count += 1
            curcol, currow, direction = mark_and_move(curcol, currow, direction, data, False)

        print(f"Part 2: {part2count}")





