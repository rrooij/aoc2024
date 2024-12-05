with open("input.txt", "r") as f:
    lines = f.read().splitlines()

def check_rule(rule, ordering):
    pages = [int(x) for x in rule.split(",")]
    for idx, page in enumerate(pages):
        if page not in ordering:
            continue
        afters = pages[idx + 1:]
        for after in afters:
            if after not in ordering[page]:
                continue
            if after in ordering[page]:
                return (False, pages)
    return (True, pages)

# structure: { page: [befores] }
orders = {}
valid_rules = 0
middles = 0
for line in lines:
    if line == "":
        continue
    if "|" not in line:
        (valid, pages) = check_rule(line,orders)
        if valid:
            valid_rules = valid_rules + 1
            middle = pages[int((len(pages) - 1) / 2)]
            middles = middles + middle
        continue
    splitted = line.split("|")
    before = int(splitted[0])
    after = int(splitted[1])
    if after not in orders:
        orders[after] = [before]
    else:
        orders[after].append(before)

print(middles)
