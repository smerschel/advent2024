import sys
from dataclasses import dataclass

@dataclass
class File:
    id: int
    size: int
    start: int
    end: int

@dataclass
class Space:
    size: int
    start: int
    end: int

def read_file(name):
    with open(name, "r") as file:
        data = file.read()
    return data

def explode_blocks(data):
    answer = []
    isFile = True
    curIdx = 0
    curFile = 0
    # problem, there are going to be more than 10 files, so using a single number to represent a file in the string like in the example won't work
    for c in data:
        if isFile:
            answer[curIdx:int(c)] = [curFile] * int(c)
            isFile = False
            curIdx += int(c)
            curFile += 1
        else:
            answer[curIdx:int(c)] = [-1] * int(c)
            isFile = True
            curIdx += int(c)

    #print(answer)
    return answer

def explode_blocks2(data):
    isFile = True
    curIdx = 0
    curFile = 0
    files = []
    spaces = []
    # I want my data to be a little smarter to make the logic easier this time
    # For each file, I want an ID, size, and start (and end, I guess)
    # Then, I want a list of empty spaces, they don't need an ID, but the should have a size and start (and end?)
    for c in data:
        if isFile:
            files.append(File(id=curFile, size=int(c), start=curIdx, end=curIdx+int(c)-1))
            isFile = False
            curIdx += int(c)
            curFile += 1
        else:
            spaces.append(Space(size=int(c), start=curIdx, end=curIdx+int(c)-1))
            isFile = True
            curIdx += int(c)

    #print(files)
    #print(spaces)
    return files,spaces

def compact_blocks(data):
    # put a pointer at the end of the array and a pointer at the beginning,
    # start walking from the start and when you encounter a -1, move the value at the end to that location and start moving from the 
    # end until you hit the next non-negative value.  You are done when the start and end pointer meet
    start = 0
    end = len(data)-1
    while start < end:
        if data[start] >= 0:
            start += 1
        else:
            while data[end] < 0:
                end -= 1
            data[start] = data[end]
            data[end] = -1
            end -= 1
    #print(data)
    return data

def fits(file, spaces):
    for space in spaces:
        if space.size >= file.size and space.start < file.start:            
            return space
    return None

def update(file, space):
    space.size -= file.size
    file.start = space.start
    file.end = file.start + file.size - 1
    space.start = file.end + 1

def print_files(files):
    end = 0
    for file in files:
       if file.end > end:
           end = file.end
    output = list('.' * end)
    for file in files:
        print (file)
        output[file.start:file.end+1] = [str(file.id)] * file.size
        print("".join(output))

def compact_full_blocks(files, spaces):
    # my reading of the problem is you start at the end of files and only go through it once moving full
    # files to the earliest space they will completely fit, if any
    for file in reversed(files):
        space = fits(file, spaces)
        if space is not None:
            update(file, space)
    #print(files)
    #print(spaces)
    #print_files(files)
    return files

def compute_answer(data):
    answer = 0
    for i,v in enumerate(data):
        if v < 0:
            break
        answer += i*v
    return answer

def compute_answer2(files):
    answer = 0
    for file in files:
        for i in range(file.size):
            answer += file.id * (file.start+i)
    return answer

def solve_part1(data):
    exploded_blox = explode_blocks(data)
    compressed_blox = compact_blocks(exploded_blox)
    return compute_answer(compressed_blox)

def solve_part2(data):
    files,spaces = explode_blocks2(data)
    files = compact_full_blocks(files,spaces)
    return compute_answer2(files)
      
if __name__ == "__main__":
    part2 = sys.argv[2] == "part2" if len(sys.argv) > 2 else False
    dev = sys.argv[3] == "dev" if len(sys.argv) > 3 else False

    part1answer = 0

    data = read_file(sys.argv[1])

    part1answer = solve_part1(data)

    print(f"Part 1: {part1answer}")

    part2answer = solve_part2(data)

    print(f"Part 2: {part2answer}")
