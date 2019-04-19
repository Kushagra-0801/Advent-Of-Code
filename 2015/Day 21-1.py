from collections import namedtuple
from itertools import combinations, product

player = {}
boss = {}
Equipment = namedtuple('Equipment', 'cost damage armor')
weapons = {}
armors = {'blank': Equipment(0, 0, 0)}
rings = {'blank': Equipment(0, 0, 0)}

def player_wins(weapon, armor='blank', ring1='blank', ring2='blank'):
    damage = weapons[weapon].damage + rings[ring1].damage + rings[ring2].damage
    armor = armors[armor].armor + rings[ring1].armor + rings[ring2].armor
    dmg_give = max(damage - boss['Armor'], 1)
    dmg_take = max(boss['Damage'] - armor, 1)
    moves_for_player = (boss['Hit Points'] + dmg_give - 1) // dmg_give
    moves_for_boss = (player['Hit Points'] + dmg_take - 1) // dmg_take
    if moves_for_player <= moves_for_boss:
        return True
    else:
        return False

with open('Day 21 - input', 'r') as f:
    f.readline()
    for _, line in zip(range(3), f):
        line = line.strip().split(': ')
        player[line[0]] = int(line[-1])
    f.readline()
    f.readline()
    for _, line in zip(range(3), f):
        line = line.strip().split(': ')
        boss[line[0]] = int(line[-1])
    f.readline()
    f.readline()
    for _, line in zip(range(5), f):
        line = line.strip().split()
        weapons[line[0]] = (Equipment(cost=int(line[1]), damage=int(line[2]), armor=int(line[3])))
    f.readline()
    f.readline()
    for _, line in zip(range(5), f):
        line = line.strip().split()
        armors[line[0]] = (Equipment(cost=int(line[1]), damage=int(line[2]), armor=int(line[3])))
    f.readline()
    f.readline()
    for _, line in zip(range(6), f):
        line = line.strip().split()
        rings[line[0] + line[1]] = (Equipment(cost=int(line[2]), damage=int(line[3]), armor=int(line[4])))

    lowest_cost = 74 + 102 + 100 + 80
    for weapon, armor in product(weapons.keys(), armors.keys()):
        for ring in rings.keys():
            cost = weapons[weapon].cost + armors[armor].cost + rings[ring].cost
            if player_wins(weapon, armor, ring):
                lowest_cost = min(lowest_cost, cost)
        for ring1, ring2 in combinations(rings.keys(), 2):
            cost = weapons[weapon].cost + armors[armor].cost + rings[ring1].cost + rings[ring2].cost
            if player_wins(weapon, armor, ring1, ring2):
                lowest_cost = min(lowest_cost, cost)
    print(lowest_cost)
