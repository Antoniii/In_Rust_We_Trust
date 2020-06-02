#include <stdio.h>
int main() {
	int A, B;

	printf("Введите А: ");
	scanf("%d", &A);

	printf("Введите B: ");
	scanf("%d", &B);

	A = A - B;
	B = A + B;
	A = B - A;

	printf("\nПосле замены, А = %d\n", A);
	printf("B = %d\n", B);

	return 0;
}

// gcc change2num.c -o change2num