def check_match(chars, matches):
    if chars == "XMAS":
        return matches + 1
    return matches


def main():
    with open("input.txt", "r") as f:
        lines = f.read().splitlines()
    row_max_idx = len(lines[0]) - 1
    max_lines_idx = len(lines) - 1

    matches = 0
    match_word = "XMAS"
    for idx, line in enumerate(lines):
        for idx_char, character in enumerate(line):
            if character == "X":
                left_chars = line[idx_char - 3 : idx_char + 1] if idx_char > 2 else ""
                right_chars = line[idx_char : idx_char + 4] if idx_char < row_max_idx - 2 else ""
                bottom_chars = "".join(
                    [lines[idx + x][idx_char] for x in range(0, 4)]
                ) if idx < max_lines_idx - 2 else ""
                top_chars = "".join([lines[idx - x][idx_char] for x in range(0, 4)]) if idx > 2 else ""
                diagleft_top_chars = "".join(
                        [lines[idx - x][idx_char - x] for x in range(0, 4)]
                ) if idx_char > 2 and idx > 2 else ""
                diagright_top_chars = "".join(
                        [lines[idx - x][idx_char + x] for x in range(0, 4)]
                ) if idx_char < row_max_idx - 2 and idx > 2 else ""
                diagright_down = "".join(
                        [lines[idx + x][idx_char + x] for x in range(0, 4)]
                ) if idx_char < row_max_idx - 2 and idx < max_lines_idx - 2 else ""
                diagleft_down = "".join(
                        [lines[idx + x][idx_char - x] for x in range(0, 4)]
                ) if idx_char > 2 and idx < max_lines_idx - 2 else ""
                if left_chars == "SAMX":
                    matches = matches + 1
                other_matches = [right_chars, bottom_chars, top_chars, diagleft_top_chars, diagright_top_chars, diagright_down, diagleft_down]
                for x in other_matches:
                    matches = check_match(x, matches)
    print(matches)


if __name__ == "__main__":
    main()
