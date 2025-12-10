import re
import sys

from z3 import Int, Optimize, sat


def main():
    part2 = 0

    for line in sys.stdin.read().strip().split("\n"):
        # Extract parenthesis groups and curly brace group
        paren_groups = re.findall(r"\((.*?)\)", line)
        curly_group = re.findall(r"\{(.*?)\}", line)[0]

        # Convert input group to list of integers
        paren_groups = [list(map(int, group.split(","))) for group in paren_groups]
        results = list(map(int, curly_group.split(",")))

        # Each parenthesis group corresponds to a variable
        variables = []
        for i in range(len(paren_groups)):
            variables.append(Int(f"x{i + 1}"))

        # Build equations
        solver = Optimize()
        for eq_index, result in enumerate(results):
            var_indices = [
                i for i, group in enumerate(paren_groups) if eq_index in group
            ]

            # Add in each variable whose paren group contains this result index
            expr = sum(variables[idx] for idx in var_indices)
            solver.add(expr == result)

        for var in variables:
            solver.add(var >= 0)

        # Minimize sum of all variables
        total_sum = sum(variables)
        solver.minimize(total_sum)

        if solver.check() == sat:
            model = solver.model()
            part2 += sum(model[v].as_long() for v in variables)
        else:
            print("No solution found.")

    print("Part 2: ", part2)


if __name__ == "__main__":
    main()
