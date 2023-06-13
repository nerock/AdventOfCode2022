package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const START = "AA"
const MINUTES = 30

type Valve struct {
	Flow    int
	Tunnels []string
}

func main() {
	_, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	fmt.Println(lengthOfLongestSubstring("abcabcbb"))

	//fmt.Println(partOne(string(input)))
}

func lengthOfLongestSubstring(s string) int {
	var max int
	for i := 0; i < len(s)-1; i++ {
		var j int
		for j = i + 1; j < len(s); j++ {
			a := string(s[i])
			b := string(s[j])
			if a == b {
				break
			}
		}
		if max < j-i {
			max = j - i
		}
		i = j - 1
	}

	return max
}

func partOne(input string) int {
	valves := getValves(input)

	return bestFlow(valves)
}

func getValves(input string) map[string]Valve {
	valves := make(map[string]Valve)
	for _, valve := range strings.Split(input, "\n") {
		split := strings.Split(valve, ";")
		id := strings.Split(split[0], " ")[1]
		flow, _ := strconv.Atoi(strings.Split(split[0], "=")[1])
		word := "valve "
		if strings.Contains(split[1], "valves") {
			word = "valves "
		}
		tunnels := strings.Split(strings.Split(split[1], word)[1], ", ")

		valves[id] = Valve{
			Flow:    flow,
			Tunnels: tunnels,
		}
	}

	return valves
}

func bestFlow(valves map[string]Valve) int {
	visited := make(map[string]int, len(valves))
	opened := make([]string, len(valves))

	var getFlow func(id string, v map[string]Valve, minutes int, opened []string, visited map[string]int) int
	getFlow = func(id string, v map[string]Valve, minutes int, opened []string, visited map[string]int) int {
		key := fmt.Sprintf("%s%d%q", id, minutes, opened)
		if v, ok := visited[key]; ok {
			return v
		}

		if minutes <= 0 {
			visited[id] = 0
			return 0
		}

		var best int
		if !find(id, opened) {
			current := (minutes - 1) * v[id].Flow
			for _, tunnel := range v[id].Tunnels {
				if current > 0 {
					best = max(best, current+getFlow(tunnel, v, minutes-2, append(opened, id), visited))
				}

				best = max(best, getFlow(tunnel, v, minutes-1, opened, visited))
			}
		}

		visited[key] = best
		return best
	}

	return getFlow(START, valves, MINUTES, opened, visited)
}

func bestFlowElephant(valves map[string]Valve) int {
	visited := make(map[string]int, len(valves))
	opened := make([]string, len(valves))

	var getFlow func(id string, v map[string]Valve, minutes int, opened []string, visited map[string]int) int
	getFlow = func(id string, v map[string]Valve, minutes int, opened []string, visited map[string]int) int {
		key := fmt.Sprintf("%s%d%q", id, minutes, opened)
		if v, ok := visited[key]; ok {
			return v
		}

		if minutes <= 0 {
			visited[id] = 0
			return 0
		}

		var best int
		if !find(id, opened) {
			current := (minutes - 1) * v[id].Flow
			for _, tunnel := range v[id].Tunnels {
				if current > 0 {
					best = max(best, current+getFlow(tunnel, v, minutes-2, append(opened, id), visited))
				}

				best = max(best, getFlow(tunnel, v, minutes-1, opened, visited))
			}
		}

		visited[key] = best
		return best
	}

	return getFlow(START, valves, MINUTES, opened, visited)
}

func find(v string, s []string) bool {
	for _, i := range s {
		if i == v {
			return true
		}
	}

	return false
}

func copyMap(m map[string]struct{}) map[string]struct{} {
	c := make(map[string]struct{})
	for k, v := range m {
		c[k] = v
	}

	return c
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
