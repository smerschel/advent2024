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

def mark_and_move(x, y, direction, data, mark):
    if mark:
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

def check_infinite(x, y, direction, data, history):
    # if I have been here before going this direction, I'm in an infinite loop
    if direction in history[y][x]:
        return True
    return False

def infinite_move(x, y, direction, data, history):
    # print history for xy
    #print(f"PreMove History: {history[y][x]}")
    # check for wall or O in front of me
    if direction == UP:
        if data[y-1][x] == "#" or data[y-1][x] == "O":
            # turn right
            direction = RIGHT
            inf = check_infinite(x, y, direction, data, history)
            history[y][x].append(direction)
            return x, y, direction, inf
    elif direction == DOWN:
        if data[y+1][x] == "#" or data[y+1][x] == "O":
            # turn right
            direction = LEFT
            inf = check_infinite(x, y, direction, data, history)
            history[y][x].append(direction)
            return x, y, direction, inf
    elif direction == LEFT:
        if data[y][x-1] == "#" or data[y][x-1] == "O":
            # turn right
            direction = UP
            inf = check_infinite(x, y, direction, data, history)
            history[y][x].append(direction)
            return x, y, direction, inf
    elif direction == RIGHT:
        if data[y][x+1] == "#" or data[y][x+1] == "O":
            # turn right
            direction = DOWN
            inf = check_infinite(x, y, direction, data, history)
            history[y][x].append(direction)
            return x, y, direction, inf
        
    # move forward, record history, and check for infinite loop
    if direction == UP:
        y -= 1
    elif direction == DOWN:
        y += 1
    elif direction == LEFT:
        x -= 1
    elif direction == RIGHT:
        x += 1
    #print(f"PostMove History: {history[y][x]}")
    inf = check_infinite(x, y, direction, data, history)
    history[y][x].append(direction)
    return x, y, direction, inf

def run_infinite(x, y, direction, data, history):
    # check if I am at the exit
    if next_to_exit(x, y, direction, len(data[0]), len(data)):
        return False

    # make a deep copy of data and history so we don't modify the original
    data_copy = copy.deepcopy(data)
    history_copy = copy.deepcopy(history)
    # put an "O" in front of current location
    if direction == UP:
        data_copy[y-1][x] = "O"
    elif direction == DOWN:
        data_copy[y+1][x] = "O"
    elif direction == LEFT:
        data_copy[y][x-1] = "O"
    elif direction == RIGHT:
        data_copy[y][x+1] = "O"

    loopcount = 0
    # while not at exit
    while not next_to_exit(x, y, direction, len(data_copy[0]), len(data_copy)):
        # print location and direction
        #print(f"Location: {x}, {y}, Direction: {direction}")
        x, y, direction, repeat = infinite_move(x, y, direction, data_copy, history_copy)
        loopcount += 1
        if loopcount > 1000000:
            print(f"Too many loops: {loopcount}")
            #print("Data:")
            return True
        if repeat:
            # print out data_copy
            print("Infinite loop detected")
            #print("Data:")
            #for row in data_copy:
            #    print("".join(row))
            return True
    return False


def move_and_check(x, y, startx, starty, direction, data, history):

    # check if I am at the exit
    if next_to_exit(x, y, direction, len(data[0]), len(data)):
        # move to exit and return position
        if direction == UP:
            y -= 1
        elif direction == DOWN:
            y += 1
        elif direction == LEFT:
            x -= 1
        elif direction == RIGHT:
            x += 1
        return x, y, direction
    
    # check if there is a wall in front of me, if so, turn and return
    if direction == UP:
        if data[y-1][x] == "#":
            # turn right
            direction = RIGHT
            history[y][x].append(direction)
            return x, y, direction
    elif direction == DOWN:
        if data[y+1][x] == "#":
            # turn right
            direction = LEFT
            history[y][x].append(direction)
            return x, y, direction
    elif direction == LEFT:
        if data[y][x-1] == "#":
            # turn right
            direction = UP
            history[y][x].append(direction)
            return x, y, direction
    elif direction == RIGHT:
        if data[y][x+1] == "#":
            # turn right
            direction = DOWN
            history[y][x].append(direction)
            return x, y, direction
        
    # if we got here, there is not a wall in front of us

    # check if the original position of the guard is in front of us, and if so, move forward and return
    # because we can't put an obstacle there
    if direction == UP:
        if y-1 == starty and x == startx:
            y -= 1
            history[y][x].append(direction)
            return x, y, direction
    elif direction == DOWN:
        if y+1 == starty and x == startx:
            y += 1
            history[y][x].append(direction)
            return x, y, direction
    elif direction == LEFT:
        if y == starty and x-1 == startx:
            x -= 1
            history[y][x].append(direction)
            return x, y, direction
    elif direction == RIGHT:
        if y == starty and x+1 == startx:
            x += 1
            history[y][x].append(direction)
            return x, y, direction

    # now move forward 1
    if direction == UP:
        y -= 1
    elif direction == DOWN:
        y += 1
    elif direction == LEFT:
        x -= 1
    elif direction == RIGHT:
        x += 1
    history[y][x].append(direction)

    return x, y, direction
    
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

    # this works for the test cases, but it must be missing infinite loops because my number is too low.  Could I brute force
    # this and just for each step, turn right and move and if I get back to the same spot or get to the exit is how I check?  
    # It's crude, but it would work.  

    # I found a corner case, where I think you get in an infinite loop, but never return to the spot you started at.  Which makes 
    # sense.  So, do I have to do my checking better?  So, if you ever get to the same spot and same direction you've been
    # previously, then you're in an infinite loop.  Which requires you to do a huge history store and check for every move.
    # Damn, that's brute force.  An even shittier solution would be to just run some threshold of steps and if you never get to 
    # the exit in say 1,000,000 steps, you are more than likely in an infinite loop.  But that is a hack more than a solution.

    if part2:
        # ok, I've been through the ringer with this piece of shit.

        # walk the guard through the map, a step, or a turn is considered a move
        # at each move, you must check:
        # 1 - is there a barrier in front of me, if so, turn right (you can't add an obstacle where one exists)
        # 2 - is the edge in front of me, if so, move forward (and exit)
        # 3 - if I turned right here (and followed the original map) would I get stuck in an infinite loop?
        # How do you determine an infinite loop?  I think for each location you visit, you must mark that you have 
        # been there, and what direction you were going when you were there.  If, by following the map, you get back 
        # to a location that you have visited before, and in the same direction, you are stuck in an infinite loop
        # If you get to the exit, you are obviously not.  
        # 
        # There is also a silly exception that the obstruction can't be put at the original location of the guard, 
        # so add a check for that.  

        # reset the data and the carrot and all variables
        data = read_file_to_2d_array(sys.argv[1])
        columns = len(data[0])
        rows = len(data)
        curcol, currow, direction = find_carrot(data)
        startcol = curcol
        startrow = currow
        startdir = direction
        # create a history, this will be a 2d array of lists.  This list for each location will be the directions that have 
        # visited this location.  So, the list will originally be empty for every location.  As you visit a location, you will add
        # the direction.  
        history = []
        for row in range(rows):
            history.append([])
            for col in range(columns):
                history[row].append([])
        # add the starting position to the history
        history[currow][curcol].append(direction)

        infinitecount = 0
        while not at_exit(curcol, currow, columns, rows):
            if run_infinite(curcol, currow, direction, data, history):
                infinitecount += 1
            curcol, currow, direction = move_and_check(curcol, currow, startcol, startrow, direction, data, history)

        print(f"Part 2: {infinitecount}")





