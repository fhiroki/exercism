package tree

import (
	"fmt"
	"sort"
)

// Record is input record.
type Record struct {
	ID, Parent int
}

// Node is tree node.
type Node struct {
	ID       int
	Children []*Node
}

// Build returns reconstructed tree.
func Build(records []Record) (*Node, error) {
	sort.Slice(records, func(i, j int) bool { return records[i].ID < records[j].ID })

	nodes := make(map[int]*Node)
	for i, r := range records {
		if i != r.ID || r.Parent > i || (i != 0 && r.Parent == i) {
			return nil, fmt.Errorf("invalid record: %+v", r)
		}
		node := &Node{ID: r.ID}
		if r.ID > 0 {
			nodes[r.Parent].Children = append(nodes[r.Parent].Children, node)
		}
		nodes[i] = node
	}
	return nodes[0], nil
}
