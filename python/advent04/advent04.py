import re

def read_file(name):
    with open(name, "r") as file:
        data = file.read()
    return data

def find_all(text, substring):
    index = text.find(substring)
    while index != -1:
        print(f"Found at index {index}")
        index = text.find(substring, index + 1)  # Continue searching after the last found position

def find_all_regex(text, pattern, mult=False):
    total = 0
    matches = []
    for match in re.finditer(pattern, text):
        #print(f"Found at index {match.start()}")
        matches.append(match.start())
        if mult:
            num1 = int(match.group(1))
            num2 = int(match.group(2))
            #print(f"Extracted numbers: {num1}, {num2}")  
            total += num1*num2
    print(total)      
    return matches

def strip_donts(text):
    result = ""
    index = text.find("don't()")
    result = text[:index]
    print(f"Found don't() at index {index}")
    while index != -1:
        # find next do
        start = text.find("do()", index + 1)
        print(f"Found do() at index {start}")
        index = text.find("don't()", start + 1)
        print(f"Found don't() at index {index}")
        result += text[start:index]
    return result

def count_regex(data, pattern):
    count = 0
    for match in re.finditer(pattern, data):
        count += 1
    return count

def read_file_to_2d_array(filename):
    with open(filename, "r", encoding="utf-8") as f:
        return [list(line.rstrip("\n")) for line in f]  # Convert each line to a list of characters

def transpose_2d_array(array):
    return list(map(list, zip(*array)))  # Transpose using zip and convert tuples to lists

def diag_transpose_2d_array(array):
    newdata = []
    rows = len(array)
    cols = len(array[0])

    # you start at row 0 and go down diagonally making lines
    for i in range(rows):
        line = array[i][0]
        column = 1
        for j in range(i+1,rows):
            line += array[j][column]
            column += 1
        newdata.append(list(line))
    # then you start at column 1 and go left to right down diagonally making lines
    for i in range(1,cols):
        line = array[0][i]
        row = 1
        for j in range(i+1,cols):
            line += array[row][j]
            row += 1 
        newdata.append(list(line))

    #print(newdata)
    return newdata

def check_cross(data,row, col):
    found_one = False
    if data[i-1][j-1] == 'M':
        if data[i+1][j+1] == 'S':
            found_one = True
    elif data[i-1][j-1] == 'S':
        if data[i+1][j+1] == 'M':
            found_one = True

    if found_one:
        if data[i-1][j+1] == 'M':
            if data[i+1][j-1] == 'S':
                return True
        elif data[i-1][j+1] == 'S':
            if data[i+1][j-1] == 'M':
                return True
    return False

        

if __name__ == "__main__":
    part1 = False
    count = 0
    if part1:
        data = read_file("input.txt")
        #testdata = """abc
        #              def
        #              ghi"""
        #data = testdata
        count = 0
        # find horizontal
        count = count_regex(data, r"XMAS")
        print(f"Count: {count}")
        count += count_regex(data, r"SAMX")
        print(f"Count: {count}")
        # transpose data
        data = read_file_to_2d_array("input.txt")
        #data = [['a','b','c'],['d','e','f'],['g','h','i']]
        tdata = transpose_2d_array(data)
        for row in tdata:
            count += count_regex("".join(row), r"XMAS")
            count += count_regex("".join(row), r"SAMX")
        print(f"Count: {count}")
        # now find the diagonals
        # we can do this again by re-arranging the input.  
        ddata = diag_transpose_2d_array(data)
        for row in ddata:
            print(row)
            count += count_regex("".join(row), r"XMAS")
            count += count_regex("".join(row), r"SAMX")
        print(f"Count: {count}")
        # need to do the other direction diagonal
        for row in data:
            row.reverse()
        tddata = diag_transpose_2d_array(data)
        #print(tddata)
        for row in tddata:
            count += count_regex("".join(row), r"XMAS")
            count += count_regex("".join(row), r"SAMX")
    else:
        # this actually seems easier.  I will go through the array and look for A's and check the corners to see if it's a X-MAS
        data = read_file_to_2d_array("input.txt")
        rows = len(data)
        cols = len(data[0])
        # A can't be on an edge
        for i in range(1,rows-1):
            for j in range(1,cols-1):
                if data[i][j] == 'A':
                    if check_cross(data,i,j):
                        count += 1
    print(f"Count: {count}")



                

