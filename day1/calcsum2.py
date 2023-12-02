
def getdigit(code):

    # if first character is a digit, dig = 10*first
    # if first character is not a digit, take the substring from first character to the point where a digit is found

    if code[0].isdigit():
        dig = int(code[0])*10
    
    else:
        substr = "".join([char for char in code if not char.isdigit()])

def sumcode(liststr):
    total = 0
    
    for code in liststr:
        total += getdigit(code)

    return total


if __name__ == "__main__":

    file = open(input(), "r")

    liststr = file.read().splitlines()
    print(sumcode(liststr))