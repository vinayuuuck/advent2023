
def getdigit(code):
    # TODO: Implement this function

    return 0

def sumcode(liststr):
    total = 0
    
    for code in liststr:
        total += getdigit(code)

    return total


if __name__ == "__main__":

    file = open(input(), "r")

    liststr = file.read().splitlines()
    print(sumcode(liststr))