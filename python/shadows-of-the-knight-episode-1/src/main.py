import sys

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.

# w: width of the building.
# h: height of the building.
w, h = [int(i) for i in input().split()]
n = int(input())  # maximum number of turns before game over.
x0, y0 = [int(i) for i in input().split()]

current_position = (x0, y0)
limit_top, limit_bottom, limit_left, limit_right = 0, h, 0, w
bomb_area_x, bomb_area_y = None, None

# game loop
while True:
    bomb_dir = input()  # the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

    # Write an action using print
    # To debug: print("Debug messages...", file=sys.stderr, flush=True)

    print(f'x0 {x0} - y0 {y0}', file=sys.stderr)
    print(f'w {w} - h {h}', file=sys.stderr)
    print(f'bomb_dir: {bomb_dir}', file=sys.stderr)
    print(f'Limits t {limit_top} b {limit_bottom} - l {limit_left} r {limit_right}',
          file=sys.stderr)

    if "U" in bomb_dir:
        limit_bottom = current_position[1]
        bomb_area_y = (limit_top, limit_bottom)
    elif "D" in bomb_dir:
        limit_top = current_position[1]
        bomb_area_y = (limit_top, limit_bottom)
    if "L" in bomb_dir:
        limit_right = current_position[0]
        bomb_area_x = (limit_left, limit_right)
    elif "R" in bomb_dir:
        limit_left = current_position[0]
        bomb_area_x = (limit_left, limit_right)

    # the location of the next window Batman should jump to.
    if bomb_area_x:
        print(f'bomb_area_x: {list(bomb_area_x)}', file=sys.stderr)
        jump_position_x = (round((bomb_area_x[0] + bomb_area_x[1]) * .5))
    else:
        jump_position_x = current_position[0] if current_position else x0
    if bomb_area_y:
        print(f'bomb_area_y: {list(bomb_area_y)}', file=sys.stderr)
        jump_position_y = (round((bomb_area_y[0] + bomb_area_y[1]) * .5))
    else:
        jump_position_y = current_position[1] if current_position else y0

    # print(f'Target x {jump_position_x} - y {jump_position_y}', file=sys.stderr)
    current_position = (
        jump_position_x,
        jump_position_y
    )
    print(f'{jump_position_x} {jump_position_y}')
    bomb_area_x, bomb_area_y = None, None
