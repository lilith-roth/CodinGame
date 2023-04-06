import sys

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.

# w: width of the building.
# h: height of the building.
w, h = [int(i) for i in input().split()]
n = int(input())  # maximum number of turns before game over.
x0, y0 = [int(i) for i in input().split()]
current_position = (x0, y0)
# game loop
while True:
    bomb_dir = input()  # the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

    # Write an action using print
    # To debug: print("Debug messages...", file=sys.stderr, flush=True)

    print(f'x0 {x0} - y0 {y0}', file=sys.stderr)
    print(f'w {w} - h {h}', file=sys.stderr)
    print(f'bomb_dir: {bomb_dir}', file=sys.stderr)

    if "U" in bomb_dir:
        bomb_area_y = range(0, current_position[1])
    elif "D" in bomb_dir:
        bomb_area_y = range(current_position[1], h)
    if "L" in bomb_dir:
        bomb_area_x = range(0, current_position[0])
    elif "R" in bomb_dir:
        bomb_area_x = range(current_position[0], w)


    # the location of the next window Batman should jump to.
    if bomb_area_x:
        print(f'bomb_area_x: {list(bomb_area_x)}', file=sys.stderr)
        jump_position_x = bomb_area_x[round(len(bomb_area_x) / 2)]
    if bomb_area_y:
        print(f'bomb_area_y: {list(bomb_area_y)}', file=sys.stderr)
        jump_position_y = bomb_area_y[round(len(bomb_area_y) / 2)]

    current_position = (
        jump_position_x if jump_position_x else "",
        jump_position_y if jump_position_y else ""
    )
    print(f'{jump_position_x} {jump_position_y}')
    bomb_area_x, bomb_area_y = None, None
