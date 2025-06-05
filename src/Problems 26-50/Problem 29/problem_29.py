# https://projecteuler.net/problem=29
import time
start_time = time.time()
storage = []
for a in range(2,101):
    for b in range(2,101):
        answer = a**b
        storage.append(answer)
sorted_storage = list(set(storage))
length = len(sorted_storage)
print(length)
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)