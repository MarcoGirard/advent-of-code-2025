#!/usr/bin/env python3


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
            inter_pos = curr_pos - steps
        elif direction == "R":
            inter_pos = curr_pos + steps

        spin = abs(inter_pos // 100)
        curr_pos = inter_pos % 100
        zero_count += spin
        # if curr_pos == 0:
        #     zero_count += 1

    print("Pwd: ", zero_count)


if __name__ == "__main__":
    main()
