// Package weather provide current weather condition and location.
package weather

// CurrentCondition represent current condition.
var CurrentCondition string

// CurrentLocation represent current location.
var CurrentLocation string

// Forecast retrun current weather and location.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
