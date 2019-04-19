import json

def return_sum(obj):
    if isinstance(obj, dict):
        final_sum = 0
        for k, v in obj.items():
            if v == 'red': #or k == 'red':
                return 0
            final_sum += return_sum(k) + return_sum(v)
        return final_sum
    elif isinstance(obj, int):
        return obj
    elif isinstance(obj, str):
        return 0
    elif isinstance(obj, list):
        final_sum = 0
        return sum((return_sum(i) for i in obj))


with open('Day 12 - input', 'r') as f:
    decoded = json.load(f)
    print(return_sum(decoded))
