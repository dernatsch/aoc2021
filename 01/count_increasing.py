from collections import deque

def a():
    with open("input.txt", "r") as inputfile:
        lastnum = None
        increase_counter = 0
        for line in inputfile.readlines():
            num = int(line)
            if lastnum is not None and num > lastnum:
                increase_counter += 1

            lastnum = num

    print(f"a) {increase_counter}")

def b():
    increasing = 0

    with open("input.txt", "r") as inputfile:
        buffer = deque(maxlen=3)
        leaving_num = None
        sum_of_three = 0
        old_sum_three = 0
        
        for line in inputfile.readlines():
            num = int(line)

            if len(buffer) == 3:
                leaving_num = buffer.popleft()
                
                old_sum_three = sum_of_three
                sum_of_three -= leaving_num

            buffer.append(num)

            sum_of_three += num 
            
            if leaving_num is not None and sum_of_three > old_sum_three:
                    increasing += 1

    print(f"b) {increasing}")            

if __name__=='__main__':
    a()
    b()
