def is_safe(row):
    up = row[0] < row[1]
    # check for distance >0 and <4
    windows = zip(row, row[1:])
    for nums in windows:
        if abs(nums[0]-nums[1]) < 1 or abs(nums[0]-nums[1]) > 3:
            return False
        if up:
            if nums[1] < nums[0]:
                return False
        else:
            if nums[1] > nums[0]:
                return False
    return True



def read_file(filename: str):
    data = []
    with open(filename, 'r') as file:
        for line in file:
            numbers = list(map(int, line.split()))  # Convert space-separated numbers to int
            data.append(numbers)
    return data

# Example usage
filename = "input.txt"
numbers_list = read_file(filename)

safe_count = 0

# Print to verify
for line in numbers_list:
    print(line)
    if is_safe(line):
        safe_count += 1
    else:
        for index, num in enumerate(line):
            newline = line.copy()
            newline.pop(index)
            if is_safe(newline):
                safe_count += 1
                break

print(safe_count)

