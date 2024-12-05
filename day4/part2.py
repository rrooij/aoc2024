import sys


def main():
    with open("input.txt", "r") as f:
        lines = f.read().splitlines()
    row_max_idx = len(lines[0]) - 1
    max_lines_idx = len(lines) - 1

    matches = 0
    match_word = "MAS"
    match_word_reverse = "SAM"
    for idx, line in enumerate(lines):
        for idx_char, character in enumerate(line):
            if (
                character == "A"
                and idx > 0
                and idx < max_lines_idx
                and idx_char > 0
                and idx_char < row_max_idx
            ):
                diagleft_chars = "".join(
                    [lines[(idx - 1) + x][(idx_char - 1) + x] for x in range(0, 3)]
                )
                diagright_chars = "".join(
                    [lines[(idx - 1) + x][(idx_char + 1) - x] for x in range(0, 3)]
                )
                if (
                    diagright_chars == match_word
                    or diagright_chars == match_word_reverse
                ) and (
                    diagleft_chars == match_word or diagleft_chars == match_word_reverse
                ):
                    matches = matches + 1
    print(matches)


if __name__ == "__main__":
    main()
