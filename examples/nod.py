def nod(x,y):
	if y != 0:
		return nod(y, x % y)
	else:
		return x

n1 = int(input("Введите n1: "))
n2 = int(input("Введите n2: "))

print("НОД = ", nod(n1,n2))
print("НОK = ", int((n1*n2)/nod(n1,n2)))