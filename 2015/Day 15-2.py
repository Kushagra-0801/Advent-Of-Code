from collections import namedtuple

with open('Day 15 - input', 'r') as f:
    Cookie = namedtuple(
        'Cookie', 'name capacity durability flavor texture calories')
    ingredients = []
    for line in f:
        line = line.strip().split()
        ingredients.append(Cookie(name=line[0][:-1], capacity=int(line[2][:-1]), durability=int(
            line[4][:-1]), flavor=int(line[6][:-1]), texture=int(line[8][:-1]), calories=int(line[10])))
    max_score = 0
    for i in range(101):
        for j in range(101-i):
            for k in range(101-i-j):
                l = 100 - i - j - k
                total_calories = ingredients[0].calories * i + ingredients[1].calories * j + ingredients[2].calories * k + ingredients[3].calories * l
                if total_calories == 500:
                    total_capacity = max(0, ingredients[0].capacity * i + ingredients[1].capacity * j + ingredients[2].capacity * k + ingredients[3].capacity * l)
                    total_durability = max(0, ingredients[0].durability * i + ingredients[1].durability * j + ingredients[2].durability * k + ingredients[3].durability * l)
                    total_flavor = max(0, ingredients[0].flavor * i + ingredients[1].flavor * j + ingredients[2].flavor * k + ingredients[3].flavor * l)
                    total_texture = max(0, ingredients[0].texture * i + ingredients[1].texture * j + ingredients[2].texture * k + ingredients[3].texture * l)
                    max_score = max(max_score, total_capacity*total_durability*total_flavor*total_texture)
    print(max_score)
