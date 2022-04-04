package elon

import "fmt"

// define the 'Drive()' method
func (car *Car) Drive() {
	if car.batteryDrain > car.battery {
		return
	}
	car.battery -= car.batteryDrain
	car.distance += car.speed
}

// define the 'DisplayDistance() string' method
func (car *Car) DisplayDistance() string {
	return fmt.Sprintf("Driven %d meters", car.distance)
}

// define the 'DisplayBattery() string' method
func (car *Car) DisplayBattery() string {
	return fmt.Sprintf("Battery at %d%%", car.battery)
}

// define the 'CanFinish(trackDistance int) bool' method
func (car *Car) CanFinish(trackDistance int) bool {
	for car.batteryDrain <= car.battery {
		car.Drive()
	}
	return trackDistance <= car.distance
}
