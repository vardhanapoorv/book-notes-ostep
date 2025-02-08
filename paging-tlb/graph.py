import pandas as pd
import matplotlib.pyplot as plt

# Load results
data = pd.read_csv("tlb_results-on-same-cpu-without-initialization.txt")

# Plot
plt.figure(figsize=(10, 5))
plt.plot(data["Pages Accessed"], data[" Avg Time per Access (ns)"], marker="o", linestyle="-")
plt.xscale("log", base=2)  # Log scale for clarity
plt.xlabel("Pages Accessed (log2 scale)")
plt.ylabel("Avg Time per Access (ns)")
plt.title("TLB Performance Measurement")
plt.grid()
plt.show()

