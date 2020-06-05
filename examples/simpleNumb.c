#include <stdio.h>

int main() {
	long int low, high, i, flag, temp;

	printf("Введите a и b: ");
	scanf("%ld %ld", &low, &high);

	if (low > high) {
		temp = low;
		low = high;
		high = temp;
	}

	while (low < high) {
	
		flag = 0;

		for(i=2; i<=low/2; ++i){
			
			if(low % i == 0){
				flag = 1;
				break;
			}
		}

		if(flag==0)
			printf("%ld ", low);
		
		++low;
	}

	printf("\n");
	return 0;
}