#! /usr/bin/env python
import subprocess
import numpy as np
import matplotlib.pyplot as plt

def run_simulation(seed, bounds):
    result = subprocess.run(
        ["python3", "relocation.py", "-s", str(seed), "-n", "10", "-l", str(bounds), "-c"],
        capture_output=True,
        text=True
    )
    output = result.stdout
    print(f"Output for seed {seed} and bounds {bounds}:\n{output}")  # Print the output for debugging

    lines = output.splitlines()
    valid_count = 0
    total_count = 0

    for line in lines:
        if "VA" in line:
            total_count += 1
            if "VALID" in line:
                valid_count += 1

    print(f"Valid Count: {valid_count}, Total Count: {total_count}")  # Print the counts for debugging
    if total_count == 0:
        return 0  # Avoid division by zero

    fraction_valid = valid_count / total_count
    return fraction_valid

# Bounds register values to test
bounds_values = range(0, 2048, 100)  # From 0 to 3000 in steps of 300
seeds = [0, 1, 2, 3, 4]  # Different random seeds
data = []

for bounds in bounds_values:
    fractions = []
    for seed in seeds:
        fraction_valid = run_simulation(seed, bounds)
        fractions.append(fraction_valid)
    average_fraction = np.mean(fractions)
    data.append((bounds, average_fraction))
    print(f"Bounds: {bounds}, Average Fraction Valid: {average_fraction:.2f}")

# Extract bounds and fractions for plotting
bounds, fractions = zip(*data)

# Plotting the graph
plt.plot(bounds, fractions, marker='o')
plt.xlabel('Bounds Register Value')
plt.ylabel('Fraction of Valid Addresses')
plt.title('Fraction of Valid Addresses vs. Bounds Register Value')
plt.grid(True)
plt.show()
