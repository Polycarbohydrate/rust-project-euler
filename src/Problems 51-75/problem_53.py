# https://projecteuler.net/problem=53
import time
import sys
start_time = time.time()
def factorial(num):
    product = 1
    for i in range(0,num + 1):
        if i == 0:
            product *= 1
        else:
            product *= i
    return product
counter = 0
for n in range (0, 101):
    for r in range(0, 101):
        number = (factorial(n)) / ((factorial(r)) * (factorial(n - r)))
        if number > 1000000:
            counter += 1
print(counter)
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)
