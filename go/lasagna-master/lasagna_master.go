package lasagna

// define the 'PreparationTime()' function
func PreparationTime(layers []string, time int) int {
	if time == 0 {
		return len(layers) * 2
	}
	return len(layers) * time
}

// define the 'Quantities()' function
func Quantities(layers []string) (int, float64) {
	var noodles int
	var sauce float64
	for _, layer := range layers {
		if layer == "noodles" {
			noodles += 50
		}
		if layer == "sauce" {
			sauce += 0.2
		}
	}
	return noodles, sauce
}

// define the 'AddSecretIngredient()' function
func AddSecretIngredient(friendsList, myList []string) {
	myList[len(myList)-1] = friendsList[len(friendsList)-1]
}

// define the 'ScaleRecipe()' function
func ScaleRecipe(quantities []float64, portions int) []float64 {
	scaled := make([]float64, len(quantities))
	for i, quantity := range quantities {
		scaled[i] = quantity / 2.0 * float64(portions)
	}
	return scaled
}
