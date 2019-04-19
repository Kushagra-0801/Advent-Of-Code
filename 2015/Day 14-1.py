TIME = 2503
with open('Day 14 - input', 'r') as f:
    reindeers = {}
    for line in f:
        line = line.strip().split()
        reindeers[line[0]] = (int(line[3]), int(line[6]), int(line[13]))
    max_dis = 0
    # for deer, stats in reindeers.items():
    for stats in reindeers.values():
        full_cycles = TIME // (stats[1]+stats[2])
        dis = 0
        dis += full_cycles * stats[0] * stats[1]
        rem_time = TIME % (stats[1]+stats[2])
        dis += stats[0] * min(rem_time, stats[1])
        # print(deer, dis)
        if dis > max_dis:
            max_dis = dis
    print(max_dis)
