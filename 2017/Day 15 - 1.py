# generic generator
def generator(value, factor, divisor, multiple=1):
    """Return 16 LSB of previous value * factor % divisor
    """
    while True:
        value = value * factor % divisor
        if value % multiple == 0:
            yield value & 0xFFFF

with open("Day 15 - input") as f:
    data = f.readlines()

gen_a_init = int(data[0].split()[-1])
gen_b_init = int(data[1].split()[-1])
a_factor = 16807
b_factor = 48271
divisor = 2147483647

gen_a = generator(gen_a_init, a_factor, divisor, multiple=1)
gen_b = generator(gen_b_init, b_factor, divisor, multiple=1)

matches = 0
for i in range(40_000_000):
    if next(gen_a) == next(gen_b):
        matches += 1

print("Judge's final count:", matches)
