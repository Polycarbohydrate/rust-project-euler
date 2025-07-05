# https://projecteuler.net/problem=65
import time
start_time = time.time()
# Found that the OEIS sequence A003417 has the continued fraction for e.
# https://oeis.org/A003417/
# Specifically, https://oeis.org/A003417/b003417.txt
main_numbers = []
with open("b003417.txt", "r") as file:
    filelines = file.readlines()
    for line_index in range(0, 98):
        line_contents = filelines[line_index]
        nums = line_contents.split()
        main_numbers.append(nums[1])
main_numbers.reverse()
main_numerator = 1
main_denominator = 1
for num in main_numbers:
    temp_numerator = (int(num) * main_denominator) + main_numerator
    main_numerator = main_denominator
    main_denominator = temp_numerator
final_numerator = (2 * main_denominator) + main_numerator
print("Final Numerator: ", final_numerator)
print("Final Denominator: ", main_denominator)
print("Ratio: ", final_numerator / main_denominator)
digits = list(map(int, str(final_numerator)))
sum = 0
for num in digits:
    sum += num
print("Sum of digits:", sum)
print("============================")
end_time = time.time()
elapsed_time = end_time - start_time
print(elapsed_time)