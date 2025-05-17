import sys

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

def compute_answer(data):
    answer = 0
    for i,v in enumerate(data):
        if v < 0:
            break
        answer += i*v
    return answer

def compute_answer2(data):
    answer = 0
    for i,v in enumerate(data):
        if v > 0:
            answer += i*v
    return answer

def solve_part1(data):
    exploded_blox = explode_blocks(data)
    compressed_blox = compact_blocks(exploded_blox)
    return compute_answer(compressed_blox)

def solve_part2(data):
    exploded_blox = explode_blocks(data)
    return compute_answer2(compressed_blox)
      
if __name__ == "__main__":
    part2 = sys.argv[2] == "part2" if len(sys.argv) > 2 else False
    dev = sys.argv[3] == "dev" if len(sys.argv) > 3 else False

    part1answer = 0

    data = read_file(sys.argv[1])

    part1answer = solve_part1(data)

    print(f"Part 1: {part1answer}")

    part2answer = solve_part2(data)

    print(f"Part 2: {part2answer}")
