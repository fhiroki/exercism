package meetup

import "time"

type WeekSchedule int

const (
	First  WeekSchedule = 1
	Second WeekSchedule = 8
	Third  WeekSchedule = 15
	Fourth WeekSchedule = 22
	Teenth WeekSchedule = 13
	Last   WeekSchedule = -6
)

func Day(week WeekSchedule, weekday time.Weekday, month time.Month, year int) int {
	if week == Last {
		month++
	}
	t := time.Date(year, month, int(week), 12, 0, 0, 0, time.UTC)
	return t.Day() + (int(weekday-t.Weekday())+7)%7
}
