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



if __name__ == "__main__":
    data = read_file("input.txt")
    #find_all(data, "mul(")
    #data = "mul(5,111)"
    dos = find_all_regex(data, r"do\(\)")
    print(dos)
    donts = find_all_regex(data, r"don't\(\)")
    print(donts)
    newdata = strip_donts(data)
    find_all_regex(newdata, r"mul\((\d{1,3}),(\d{1,3})\)", True)
    #print(data)