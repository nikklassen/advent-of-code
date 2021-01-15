#include <iostream>
#include <unordered_map>
#include <set>
#include <vector>
#include <fstream>
#include <string>
#include <sstream>
#include <algorithm>

typedef std::pair<int, int> Comp;
typedef std::unordered_map<int, std::vector<Comp>> PortMap;

PortMap makeMap(std::vector<Comp> components) {
    PortMap g;
    for (auto& c : components) {
        if (g.count(c.first) > 0) {
            g[c.first].push_back(c);
        } else {
            g[c.first] = {c};
        }
        if (g.count(c.second) > 0) {
            g[c.second].push_back(c);
        } else {
            g[c.second] = {c};
        }
    }

    return g;
}

std::pair<int, int> findMaxStrength(int ports, PortMap& portMap, int strength, std::set<Comp>& visited, int depth = 0) {

    strength += ports;
    int newStrength = strength;
    int maxDepth = depth;
    for (auto& neighbour : portMap[ports]) {
        if (visited.count(neighbour) > 0) {
            continue;
        }

        visited.insert(neighbour);
        int next = neighbour.first == ports ? neighbour.second : neighbour.first;
        int tmpStrength, newDepth;
        std::tie(tmpStrength, newDepth) = findMaxStrength(next, portMap, strength + ports, visited, depth + 1);
        visited.erase(neighbour);
        if (newDepth >= maxDepth) {
            newStrength = std::max(newStrength, tmpStrength);
            maxDepth = newDepth;
        }
    }

    return std::make_pair(newStrength, maxDepth);
}

int main() {
    std::ifstream fin("input");
    std::string line;
    std::vector<Comp> components;
    while (fin >> line) {
        std::istringstream iss(line);
        int portA, portB;
        char _c;
        iss >> portA >> _c >> portB;
        components.push_back(std::make_pair(portA, portB));
    }

    PortMap g = makeMap(components);
    std::set<Comp> visited;
    int maxStrength;
    std::tie(maxStrength, std::ignore) = findMaxStrength(0, g, 0, visited);
    std::cout << "Max strength: " << maxStrength << std::endl;

    return 0;
}