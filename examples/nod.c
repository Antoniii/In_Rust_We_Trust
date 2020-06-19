#include <stdio.h>

int hcf(int n1, int n2);

int main() {

	int n1, n2;
	printf("Ведите 2 натуральных числа: ");
	scanf("%d %d", &n1, &n2);

	printf("НОД = %d\n", hcf(n1, n2));
	printf("НОK = %d\n", (n1*n2)/hcf(n1, n2));

	return 0;
}

int hcf(int n1, int n2) {

	if (n2 != 0)
		return hcf(n2, n1 % n2);
	else
		return n1;
}