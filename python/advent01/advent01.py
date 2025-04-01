with open("input.txt", "r") as file:
    list1 = []
    list2 = []
    for line in file:
        values = line.split()        
        list1.append(int(values[0]))
        list2.append(int(values[1]))

    slist1 = sorted(list1)
    slist2 = sorted(list2)
    distance = 0
    for a,b in zip(slist1, slist2):
        distance += abs(a-b)
    print(distance)

    # now do similarity
    similarity = 0
    for a in list1:
        count  = 0
        for b in list2:
            if a==b:
                count += 1
        similarity += a*count

    print(similarity)