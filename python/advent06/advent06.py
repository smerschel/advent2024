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

def mark_and_move(x, y, direction, data):
    data[y][x] = "X"  # Mark the current position
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
    # now check for wall
    if data[y][x] == '#':
        # move back to previous position
        if direction == UP:
            y += 1
        elif direction == DOWN:
            y -= 1
        elif direction == LEFT:
            x += 1
        elif direction == RIGHT:
            x -= 1
        # increment direction (and wrap around) and move
        if direction == UP:
            direction = RIGHT
        elif direction == RIGHT:
            direction = DOWN
        elif direction == DOWN:
            direction = LEFT
        elif direction == LEFT:
            direction = UP
    return x, y, direction

def count_exes(data):
    count = 0
    for row in data:
        for cell in row:
            if cell == "X":
                count += 1
    return count

def check_right(x, y, direction, data, moddata):
    if direction == UP:
        # look to the "right" of my current location, if there is a right or a upright or a downright before the end or #
        # return true
        for i in range(x+1, len(data[0])):
            if data[y][i] == "#":
                return False
            elif data[y][i] == "r" or data[y][i] == "R" or data[y][i] == "+":
                moddata[y-1][x] = "O"
                return True
    elif direction == RIGHT:
        # look to the "right" of my current location, if there is a down or a downleft or a downright before the end or #
        # return true
        for i in range(y+1, len(data)):
            if data[i][x] == "#":
                return False
            elif data[i][x] == "d" or data[i][x] == "=" or data[i][x] == "+":
                moddata[y][x+1] = "O"
                return True
    elif direction == DOWN:
        # look to the "right" of my current location, if there is a left or a downleft or a upleft before the end or #
        # return true
        for i in range(x-1, -1, -1):
            if data[y][i] == "#":
                return False
            elif data[y][i] == "l" or data[y][i] == "L" or data[y][i] == "=":
                moddata[y+1][x] = "O"
                return True
    elif direction == LEFT:
        # look to the "right" of my current location, if there is a up or a upright or a upleft before the end or #
        # return true
        for i in range(y-1, -1, -1):
            if data[i][x] == "#":
                return False
            elif data[i][x] == "u" or data[i][x] == "R" or data[i][x] == "L":
                moddata[y][x-1] = "O"
                return True
    return False

def advanced_move(x, y, direction, data):
    if direction == UP:
        if data[y][x] == "l":
            data[y][x] = "L"
        elif data[y][x] == "r":
            data[y][x] = "R"
        else:
            data[y][x] = "u"
        y -= 1
    elif direction == RIGHT:
        if data[y][x] == "u":
            data[y][x] = "R"
        elif data[y][x] == "d":
            data[y][x] = "+"
        else:
            data[y][x] = "r"
        x += 1
    elif direction == DOWN:
        if data[y][x] == "l":
            data[y][x] = "="
        elif data[y][x] == "r":
            data[y][x] = "+"
        else:
            data[y][x] = "d"
        y += 1
    elif direction == LEFT:
        if data[y][x] == "u":
            data[y][x] = "L"
        elif data[y][x] == "d":
            data[y][x] = "="
        else:
            data[y][x] = "l"
        x -= 1
    
    # check for wall and make sure x and y are in bounds
    if at_exit(x, y, len(data[0]), len(data)):
        return x, y, 0
    # now check for wall
    if data[y][x] == '#': 
        # move back to previous position
        if direction == UP:
            y += 1
        elif direction == DOWN:
            y -= 1
        elif direction == LEFT:
            x += 1
        elif direction == RIGHT:
            x -= 1
        # increment direction (and wrap around) and move
        if direction == UP:
            direction = RIGHT
            data[y][x] = "R"
            x += 1
        elif direction == RIGHT:
            direction = DOWN
            data[y][x] = "+"
            y += 1
        elif direction == DOWN:
            direction = LEFT
            data[y][x] = "="
            x -= 1
        elif direction == LEFT:
            direction = UP
            data[y][x] = "L"
            y -= 1
    return x, y, direction
    # now check for wall   
    
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
        curcol, currow, direction = mark_and_move(curcol, currow, direction, data)
    part1pos = count_exes(data)
    print(f"Part 1: {part1pos}")

    # I think part2 works if there is a block to your "right" you can add a block in front of you
    # and it will make a loop.  Not sure if that works 100%, but let's gite it a try

    # wrong, that doesn't work.  Maybe it has to be a block to your right that has an X in front of it?

    # I also think it needs to be at least one square over, because I don't think you're allowed to make a 
    # double turn

    # this is all wrong.  Way too many answers.  I think you have to track the direction better like in the 
    # question example.  

    # wow, I can't figure out this algorithm,  next attempt:
    # walk through reset map, and check right.  If there is a previous path to the right, in the same direction,
    # then add a "O" in front of you, to force yourself right.  The previous path to the right can't have a block "#"
    # before the path.  Question, do I have to differentiate between left/right and up/down?  Also, the puzzle made it
    # explicit that I can't put an "O" at the carrot, so maybe mark that square as special

    # dang, I was close, but you do need to differentiate between left/right and up/down.  
    # maybe we use a dictionary:
    # u = up
    # d = down
    # l = left
    # r = right
    # R = up right
    # L = up left
    # + = down right
    # = = down left

    if part2:
        # reset the data
        data = read_file_to_2d_array(sys.argv[1])
        curcol, currow, direction = find_carrot(data)
        
        if dev:
            while not at_exit(curcol, currow, columns, rows):
                curcol, currow, direction = mark_and_move(curcol, currow, direction, data)

            for row in data:
                print("".join(row))
            data = read_file_to_2d_array(sys.argv[1])
            curcol, currow, direction = find_carrot(data)

        moddata = copy.deepcopy(data)
        while not at_exit(curcol, currow, columns, rows):
            if check_right(curcol, currow, direction, data, moddata):
                part2count += 1
                #for row in moddata:
                #    print("".join(row))
                print(f"Part 2: {part2count}")
            curcol, currow, direction = advanced_move(curcol, currow, direction, data)
            print(f"curcol: {curcol}, currow: {currow}, direction: {direction}")

        print(f"Part 2: {part2count}")
        for row in moddata:
            print("".join(row))

