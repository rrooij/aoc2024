with open("input.txt", "r") as f:
    lines = f.read().splitlines()

# structure: { page: [befores] }
ordering = {}

def check_rule(pages):
    for idx, page in enumerate(pages):
        if page not in ordering:
            continue
        afters = pages[idx + 1:]
        for after in afters:
            if after not in ordering[page]:
                continue
            if after in ordering[page]:
                tmp = page
                pages[pages.index(after)] = tmp
                pages[idx] = after
                (success, new_pages) = check_rule(pages)
                return (False, pages)
    return (True, pages)

valid_rules = 0
middles = 0

for line in lines:
    if line == "":
        continue
    if "|" not in line:
        pages = [int(x) for x in line.split(",")]
        (valid, pages) = check_rule(pages)
        valid_rules = valid_rules + 1
        if not valid:
            middle = pages[int((len(pages) - 1) / 2)]
            middles = middles + middle
        continue
    splitted = line.split("|")
    before = int(splitted[0])
    after = int(splitted[1])
    if after not in ordering:
        ordering[after] = [before]
    else:
        ordering[after].append(before)
    
print(middles)
