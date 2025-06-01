# https://projecteuler.net/problem=26
periods = {}
for d in range(1, 1000):
    if d % 2 == 0 or d % 5 == 0:
        continue
    n = 1
    remainder = 10 % d
    seen_remainders = set()
    while remainder not in seen_remainders:
        seen_remainders.add(remainder)
        if remainder == 1:
            periods[d] = n
            break
        remainder = (remainder * 10) % d
        n += 1
    else:
        periods[d] = 0
sorted_periods = dict(sorted(periods.items(), key=lambda item: item[1], reverse=True))
print(sorted_periods)
