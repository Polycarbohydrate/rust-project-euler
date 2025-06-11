# https://projecteuler.net/problem=48
import time
start_time = time.time()
sum = 0
for i in range(1, 1001):
    number = i**i
    sum += number
print(sum)
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)