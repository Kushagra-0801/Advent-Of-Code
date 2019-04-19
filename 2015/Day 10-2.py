num_operations = 50

with open('Day 10 - input', 'r') as f:
    num = f.readline().strip()
    for _ in range(num_operations):
        current_digit = num[0]
        num_digits = 0
        new_num = ''
        for i in num:
            if i == current_digit:
                num_digits += 1
            else:
                new_num += str(num_digits) + str(current_digit)
                current_digit = i
                num_digits = 1
        new_num += str(num_digits) + str(current_digit)
        num = new_num
    print(len(num))
    # print(num)
