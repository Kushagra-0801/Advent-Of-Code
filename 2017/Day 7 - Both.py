"""
Advent Of Code 2017 - adventofcode.com
Day 7"""

with open("day7in") as f:
    LINES = [r.strip() for r in f.readlines()]


def calculate_weight(prog_data, prog_name):
    """
    calculates program weight recursively
    :param prog_data: A dictionary containing all of the data as defined by order_data
    :param prog_name: the prog to calculate the weight for
    :return: the weight of the program (how many programs are on top of it)
    """
    weight = prog_data[prog_name][0]
    for prog_on_top in prog_data[prog_name][1]:
        if prog_on_top != "":
            weight += calculate_weight(prog_data, prog_on_top)

    return weight


def get_weights_for_on_top(prog_data, prog_name):
    """returns a dictionary containing the calculated (NOT displayed) weights for each of the programs on top of
     the supplied one"""
    return {name: calculate_weight(prog_data, name) for name in prog_data[prog_name][1]}


def get_unbalanced_program(prog_data, prog_name):
    """Recursively iterates on all programs until it finds the single unbalanced program.
    :returns last tree that is unbalanced"""

    weight_dict = get_weights_for_on_top(prog_data, prog_name)
    print(weight_dict)
    weights = list(weight_dict.values())
    weight_set = list(set(weights))


    # this means that we achieved our goal, as there is only one weight. The parent prog is unbalanced.
    if len(weight_set) == 1:
        return None

    # only get the weight that appears once
    unique_weight = weight_set[0] if weights.count(weight_set[0]) == 1 else weight_set[1]

    unbalanced_prog = None

    for k, v in weight_dict.items():
        if unique_weight == v:
            unbalanced_prog = k  # find the program that contains the unbalanced weight
        else:
            balanced_prog = k  # to use later in calculation

    if not unbalanced_prog:
        raise Exception("unknown error")

    if prog_data[unbalanced_prog][1] == [""]:
        # get the weight difference we need to balance the stack from the rogue program
        weight_diff = weight_dict[unbalanced_prog] - weight_dict[balanced_prog]
        return prog_data[unbalanced_prog][0] - weight_diff
    else:
        # this means that we are currently at the last unbalanced tree.
        if get_unbalanced_program(prog_data, unbalanced_prog) == None:
            # get the weight difference we need to balance the stack from the rogue program
            weight_diff = weight_dict[unbalanced_prog] - weight_dict[balanced_prog]
            return prog_data[unbalanced_prog][0] - weight_diff


def order_data(lines):
    """Returns dictionary of progs ordered in {program_name: [weights, [progs,on,top]]}"""
    lines_split = [line.split(" ") for line in lines]
    program_names = [r[0] for r in lines_split]
    program_weights = [r[1][1:-1] for r in lines_split]
    program_weights = list(map(int, program_weights))

    split_arrows = [line.split(" -> ") for line in lines]  # split to get progs on top
    split_arrows = [r[1] if len(r) == 2 else "" for r in split_arrows]  # also add data to avoid KeyErrors
    programs_on_top = [arrows.split(", ") for arrows in split_arrows]

    prog_data = {}
    for num, name in enumerate(program_names):
        prog_data[name] = [program_weights[num], programs_on_top[num]]

    return prog_data


def main():
    prog_data = order_data(LINES)
    print(prog_data)
    bottom_prog = [r for r in prog_data.keys() if len([line for line in LINES if r in line]) == 1][0]
    print("Part 1 (only program that is shown once): ", bottom_prog)
    # bottom_prog = "efrqc"
    print(calculate_weight(prog_data, "ghzsq"))
    unbalanced_prog = get_unbalanced_program(prog_data, bottom_prog)
    print("Part 2, weight needed for rogue program to balance the tree: {}".format(unbalanced_prog))


if __name__ == '__main__':
    main()
