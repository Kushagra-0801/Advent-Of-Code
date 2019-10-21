"""
Advent Of Code 2017 - adventofcode.com
Day 8"""

with open("day8in") as f:
    LINES = [r.strip() for r in f.readlines()]


def prepare_input(lines):
    """
    Change the input into a python readable format
    :return:
    """
    new_lines = []
    for line in lines:
        line = line.replace("inc", "+=")
        line = line.replace("dec", "-=")
        line += " else 0"

        new_lines.append(line)

    return new_lines


def main():
    pythonic_lines = prepare_input(LINES)
    reg_names = [element.split(" ")[0] for element in pythonic_lines]
    print(pythonic_lines)
    print(reg_names)

    reg_dict = {name: 0 for name in reg_names}

    highest_val = 0

    for line in pythonic_lines:
        exec(line, {}, reg_dict)

        if reg_dict[line.split(" ")[0]] > highest_val:
            highest_val = reg_dict[line.split(" ")[0]]

    print("Part 1 answer (final highest value):", max(reg_dict.values()))
    print("Part 2 answer (highest value over all iterations): ", highest_val)


if __name__ == '__main__':
    main()
