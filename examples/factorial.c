#include <stdio.h>
long int fact(int n);

int main() {
	int n;
	printf("Ведите n: ");
	scanf("%d", &n);
	printf("%d! = %ld\n", n, fact(n));
}

long int fact(int n) {
	if (n >= 1)
		return n*fact(n-1);
	else
		return 1;
}

// max: 20! = 2432902008176640000
// 15! = 1307674368000
// 13! = 6227020800