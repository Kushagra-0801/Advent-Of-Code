from recordclass import recordclass

TIME = 2503
with open('Day 14 - input', 'r') as f:
    reindeer = recordclass('reindeer', 'speed run_time rest_time distance points')
    reindeers = {}
    for line in f:
        line = line.strip().split()
        reindeers[line[0]] = reindeer(speed=int(line[3]), run_time=int(line[6]), rest_time=int(line[13]), distance=0, points=0)

    for t in range(1, TIME+1):
        max_dis = 0
        furthest_deer = []
        for deer, stats in reindeers.items():
            full_cycles = t // (stats.run_time + stats.rest_time)
            dis = 0
            dis += full_cycles * stats.speed * stats.run_time
            rem_time = t % (stats.run_time+stats.rest_time)
            dis += stats.speed * min(rem_time, stats.run_time)
            stats.distance = dis
            if dis > max_dis:
                max_dis = dis
                furthest_deer = [deer]
            elif dis == max_dis:
                furthest_deer.append(deer)
        for deer in furthest_deer:
            reindeers[deer].points += 1
    print(*reindeers.items(), sep='\n')
    print(max(reindeers.values(), key=lambda x: x.points))
