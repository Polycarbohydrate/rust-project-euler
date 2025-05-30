# https://projecteuler.net/problem=25
# I did not use Rust for this one due to integer type limits enforced by Rust.
# If in the future necessitates, I will create my own BigInt library for Rust.
def fibonacci(n):
    if n <= 1:
        return n
    curr = 0
    prev1 = 1
    prev2 = 0
    for i in range(2, n+1):
        curr = prev1 + prev2
        prev2 = prev1
        prev1 = curr
    return curr

# Change this variably to get 4782
n = 4782
num = fibonacci(n)
num_len = len(str(num))
print("Number:")
print(num)
print("Position:")
print(n)
print("Length:")
print(num_len)
