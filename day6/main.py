import cmath

a = -1

b = 123 # time
c = 123 # distance

# Discriminant
d = (b**2) - (4 * a * c)

solution1 = (-b - cmath.sqrt(d)) / (2*a)
solution2 = (-b + cmath.sqrt(d)) / (2*a)

print(int(solution1.real - solution2.real))
