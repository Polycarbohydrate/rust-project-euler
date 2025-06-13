# https://projecteuler.net/problem=99
import time
import sys
start_time = time.time()
sys.set_int_max_str_digits(1_000_000)
file = open("0099_base_exp.txt", "r")
counter = 0
biggest = 0
for line_number, line in enumerate(file, start=1):
    if line_number % 10 == 0:
        print(line_number)
    numbers = line.split(",")
    exponent = int(numbers[0])**int(numbers[1])
    if biggest < exponent:
        biggest = exponent
        counter = line_number
print(counter)
file_2 = open("answer.txt", "w")
file_2.write(str(counter))
file_2.close()
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)
