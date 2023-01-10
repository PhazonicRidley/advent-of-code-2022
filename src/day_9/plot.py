import matplotlib.pyplot as plt

correct_data_part1 = [[3, 3], [2, 2], [2, 0], [3, 2], [4, 3], [3, 0], [1, 0], [3, 4], [1, 2], [2, 4], [4, 2], [4, 1], [0, 0]]
data = [[0, 0], [2, 4], [1, 0], [1, 2], [4, 2], [3, 2], [4, 1], [2, 2], [3, 0], [3, 4], [3, 3], [4, 3], [2, 0]]
print(sorted(data) == sorted(correct_data_part1))
print(f"Length of data: {len(data)}")
data_x = [d[0] for d in data]
data_y = [d[1] for d in data]
plt.scatter(data_x, data_y)
plt.xticks(range(10))
plt.yticks(range(10))
plt.grid(True)
plt.show()
exit(0)