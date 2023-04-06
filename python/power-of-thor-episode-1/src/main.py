import sys
import math

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.
# ---
# Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.

# light_x: the X position of the light of power
# light_y: the Y position of the light of power
# initial_tx: Thor's starting X position
# initial_ty: Thor's starting Y position
light_x, light_y, initial_tx, initial_ty = [int(i) for i in input().split()]

# game loop
current_position = {
    "x": initial_tx,
    "y": initial_ty
}
while True:
    remaining_turns = int(input())  # The remaining amount of turns Thor can move. Do not remove this line.

    # Write an action using print
    # To debug: print("Debug messages...", file=sys.stderr, flush=True)


    # A single line providing the move to be made: N NE E SE S SW W or NW
    direction = ""
    if current_position["y"] != light_y:
        if light_y > initial_ty:
            current_position["y"] += 1
            direction += "S"
        elif light_y < initial_ty:
            current_position["y"] -= 1
            direction += "N"

    if current_position["x"] != light_x:
        if light_x > initial_tx:
            current_position["x"] += 1
            direction += "E"
        elif light_x < initial_tx:
            current_position["x"] -= 1
            direction += "W"

    print(f'Direction: {direction}', file=sys.stderr)
    print(f'Current_position: {current_position}', file=sys.stderr)
    assert(direction != "")
    print(direction)
