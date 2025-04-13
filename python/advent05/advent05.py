import sys
import re
from collections import defaultdict

def read_file(name):
    with open(name, "r") as file:
        data = file.read()
    return data
       
def check_good(update):
    for num in update:
        # get index of num in update
        index = update.index(num)
        # make sublist of numbers before num in update
        sublist = update[:index]
        # see if any number in sublist is also in rules for this num
        if num in rules:
            for val in rules[num]:
                if val in sublist:
                    # if rule is in sublist, return False
                    print(f"Bad update: {update} - {num} in rules")
                    return False
    print (f"Good update: {update}")
    return True

def get_mid_value(update):
    # get mid value of update
    mid = len(update) // 2
    return int(update[mid])

def fix_update(update):
    for num in update:
        # get index of num in update
        index = update.index(num)
        # make sublist of numbers before num in update
        sublist = update[:index]
        # see if any number in sublist is also in rules for this num
        if num in rules:
            for val in rules[num]:
                if val in sublist:
                    # if val is in sublist, swap val and num in update
                    print(f"Fixing update: {update} - {num} in rules")
                    # get index of val in update
                    val_index = update.index(val)
                    # swap val and num in update
                    update[index], update[val_index] = update[val_index], update[index]
    return update


if __name__ == "__main__":
    part2 = sys.argv[2] == "part2" if len(sys.argv) > 2 else False
    dev = sys.argv[3] == "dev" if len(sys.argv) > 3 else False

    part1sum = 0
    part2sum = 0

    data = read_file(sys.argv[1])
    rules = defaultdict(list)
    updates = []
    bad_updates = []

    for line in data.splitlines():
        # if line has a | in it, parse rule
        if "|" in line:
            # split line into rule and pattern
            rule = line.split("|")
            rules[int(rule[0])].append(int(rule[1]))
        elif ',' in line:
            # split line into numbers
            numbers = line.split(",")
            updates.append(list(map(int, numbers)))
    
    print(f"Rules: {rules}")
    print(f"Updates: {updates}")
    
    for update in updates:
        if check_good(update):
            part1sum += get_mid_value(update)
        else:
            bad_updates.append(update)

    print(f"Part 1: {part1sum}")

    if part2:
        for bad_update in bad_updates:
            while not check_good(bad_update):
                fix_update(bad_update)
            part2sum += get_mid_value(bad_update)
        print(f"Part 2: {part2sum}")

