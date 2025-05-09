import sys


def read_file(name):
    with open(name, "r") as file:
        data = file.read()
    return data
           
def next_step(possibles, var):
    newpossibles = []
    for i in possibles:
        newpossibles.append(i + var)
        newpossibles.append(i * var)
        # uncomment this line for part 2
        newpossibles.append(int(str(i)+str(var)))
    return newpossibles

def computes(line):
    result, varstr = line.split(":")
    vars = tuple(int(x) for x in varstr.split())
    print (f"result: {result}, vars: {vars}")
    # brute force? - there is probably a way to divide the result by each number and look for results with no remainder
    # but we're going to start with brute force.  There are (V-1)^2 combinations of V variables, but we might
    # be able to build up the results recursively?  
    possibles = [vars[0]]
    for i in range(len(vars)-1):
        possibles = next_step(possibles, vars[i+1])
    # if result is in possibles, return True
    if int(result) in possibles:
        return True
    else:
        return False
       
if __name__ == "__main__":
    part2 = sys.argv[2] == "part2" if len(sys.argv) > 2 else False
    dev = sys.argv[3] == "dev" if len(sys.argv) > 3 else False
    part1answer = 0
    part2answer = 0

    data = read_file(sys.argv[1])

    for line in data.splitlines():
        if computes(line):
            part1answer += int(line.split(":")[0])

    print(f"Part 1: {part1answer}")




