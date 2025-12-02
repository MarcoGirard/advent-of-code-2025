import pprint


def get_input():
    with open("input.txt", "r") as f:
        return f.read().strip().split("\n")


def main():
    """Calculate the passwor"""
    moves = get_input()
    zero_count = 0
    curr_pos = 50

    for m in moves:
        direction, steps = m[0], int(m[1:])
        if direction == "L":
            curr_pos = (curr_pos - steps) % 100
        elif direction == "R":
            curr_pos = (curr_pos + steps) % 100
        if curr_pos == 0:
            zero_count += 1

    print("Pwd: ", zero_count)


if __name__ == "__main__":
    main()
