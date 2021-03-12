package tournament

import (
	"bufio"
	"fmt"
	"io"
	"sort"
	"strings"
)

type team struct {
	name        string
	matchPlayed int
	win         int
	draw        int
	loss        int
	points      int
}

// Tally returns football competition results.
func Tally(reader io.Reader, writer io.Writer) error {
	var teams = map[string]team{}
	scanner := bufio.NewScanner(reader)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}

		fields := strings.Split(line, ";")
		if len(fields) != 3 {
			return fmt.Errorf("invalid %v", line)
		}

		t1Name := fields[0]
		t2Name := fields[1]

		t1 := teams[t1Name]
		t2 := teams[t2Name]
		t1.name = t1Name
		t2.name = t2Name
		t1.matchPlayed++
		t2.matchPlayed++

		switch fields[2] {
		case "win":
			t1.win++
			t1.points += 3
			t2.loss++
		case "loss":
			t1.loss++
			t2.win++
			t2.points += 3
		case "draw":
			t1.draw++
			t1.points++
			t2.draw++
			t2.points++
		default:
			return fmt.Errorf("invalid %v", line)
		}

		teams[t1Name] = t1
		teams[t2Name] = t2
	}

	league := make([]team, 0, len(teams))
	for _, val := range teams {
		league = append(league, val)
	}

	sort.Slice(league, func(i, j int) bool {
		if league[i].points == league[j].points {
			return league[i].name < league[j].name
		}
		return league[i].points > league[j].points
	})

	const formatString = "%-31v|%3v |%3v |%3v |%3v |%3v\n"
	fmt.Fprintf(writer, formatString, "Team", "MP", "W", "D", "L", "P")
	for _, t := range league {
		fmt.Fprintf(writer, formatString, t.name, t.matchPlayed, t.win, t.draw, t.loss, t.points)
	}

	return nil
}
