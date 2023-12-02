
def getdigit(codestr):
    first = 0
    last = len(codestr) - 1

    foundf = False
    foundl = False

    dig1 = 0
    dig2 = 0

    while not foundf or not foundl:
        if not foundf:
            if codestr[first].isdigit():
                dig1 = int(codestr[first])*10
                foundf = True
            else:
                first += 1
        if not foundl:
            if codestr[last].isdigit():
                dig2 = int(codestr[last])
                foundl = True
            else:
                last -= 1

    return dig1 + dig2

def sumcode(liststr):
    sum = 0
    
    for codestr in liststr:
        sum += getdigit(codestr)

    return sum

if __name__ == "__main__":

    file = open(input(), "r")

    liststr = file.read().splitlines()
    print(sumcode(liststr))