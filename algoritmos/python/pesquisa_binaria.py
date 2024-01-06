from random import randint

def binary_search(array, item):
	# Definimos as posições extrema do array
	smaller = 0
	bigger = len(array) - 1

	while smaller <= bigger:
		# Buscamos o elemento do meio do array
		middle = (smaller + bigger) // 2
		element = array[middle]
		# Verificamos se o element é o valor que buscamos. 
		if element > item:
			bigger = middle - 1
		if element < item:
			smaller = middle + 1
		if element == item:
			return middle
	return None


array = [randint(1, 100) for _ in range(20)]
array.sort()
item = 15

print(f'Minha lista: {array}\n Número que estou buscando: {item}\n O valor se encontra na posição: {binary_search(array, item)}')
