package matrix

import (
	"errors"
	"strconv"
	"strings"
)

// Matrix is int matrix.
type Matrix [][]int

// New returns new matrix.
func New(s string) (*Matrix, error) {
	var m Matrix

	for _, line := range strings.Split(s, "\n") {
		fields := strings.Fields(line)
		if len(m) > 0 && len(fields) != len(m[len(m)-1]) {
			return nil, errors.New("colmun size must be same")
		}
		row := make([]int, len(fields))
		for i, field := range fields {
			val, err := strconv.Atoi(field)
			if err != nil {
				return nil, errors.New("value type must be int64")
			}
			row[i] = val
		}
		m = append(m, row)
	}

	return &m, nil
}

// Set sets value to matrix.
func (m *Matrix) Set(r, c, val int) bool {
	if r < 0 || c < 0 || r >= len(*m) || c >= len((*m)[0]) {
		return false
	}
	(*m)[r][c] = val
	return true
}

// Rows returns matrix rows.
func (m *Matrix) Rows() [][]int {
	rows := make([][]int, len(*m))
	for i, row := range *m {
		rows[i] = make([]int, len(row))
		for j, val := range row {
			rows[i][j] = val
		}
	}
	return rows
}

// Cols returns matrix columns.
func (m *Matrix) Cols() [][]int {
	cols := make([][]int, len((*m)[0]))
	for _, row := range *m {
		for i, val := range row {
			cols[i] = append(cols[i], val)
		}
	}
	return cols
}
