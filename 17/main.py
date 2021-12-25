from itertools import product 


def a():
    limits = [-123, -86]

    def h(v):
        return (v**2 + v) / 2

    def check(v):
        h_ = h(v)
        v = -1
        #print(h_, v)
        while h_ > limits[0]:
            h_ += v
            v -= 1
            #print(h_, v)
            if limits[0] <= h_ <= limits[1]:
                return True
        return False

    for i in range(250):
        if check(i):
            print(f"{i} - 'jo' - {h(i)}\n")
        else:
            print(f"{i} - ne\n")


def b():
    xlim = [124, 174]
    ylim = [-123, -86]

    def check2(vx, vy, debug=False):
        x, y = 0, 0
        while x < xlim[1] and y > ylim[0]:
            x += vx
            y += vy
            vy -= 1
            if vx > 0:
                vx -= 1
            if  debug:
                print(x, y)
            if xlim[0]<=x<=xlim[1] and ylim[0]<=y<=ylim[1]:
                return True
        return False

    res = []
    count = 0
    for vx, vy in product(range(1, 175), range(-123, 123)):
        if check2(vx, vy):
            res.append((vx, vy))
        count += 1

    print(len(res))

if __name__ == '__main__':
    a()
    b()
