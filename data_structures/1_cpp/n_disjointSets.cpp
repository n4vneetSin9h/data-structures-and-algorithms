#include <unordered_map>
#include <unordered_set>
#include <vector>
#include <iostream>

template <typename T>
class DisjointSets {
private:
    std::unordered_map<T, T> parent;
    std::unordered_map<T, int> rank;

public:
    void makeSet(const T& element) {
        if (parent.find(element) == parent.end()) {
            parent[element] = element;
            rank[element] = 0;
        }
    }

    T* find(const T& element) {
        if (parent[element] == element)
            return &parent[element];

        parent[element] = find(parent[element]);
        return &parent[element];
    }

    void unionSets(const T& element1, const T& element2) {
        T* parent1 = find(element1);
        T* parent2 = find(element2);

        if (*parent1 == *parent2)
            return;

        if (rank[*parent1] > rank[*parent2])
            parent[*parent2] = *parent1;
        else if (rank[*parent1] < rank[*parent2])
            parent[*parent1] = *parent2;
        else {
            parent[*parent2] = *parent1;
            rank[*parent1]++;
        }
    }

    bool contains(const T& element) const {
        return parent.find(element) != parent.end();
    }

    int setSize(const T& element) const {
        const T* representative = find(element);
        int count = 0;
        for (const auto& entry : parent) {
            if (*(find(entry.first)) == *representative)
                count++;
        }
        return count;
    }

    std::vector<T> elementsInSameSet(const T& element) {
        std::vector<T> elements;
        const T* representative = find(element);
        for (const auto& entry : parent) {
            if (*(find(entry.first)) == *representative)
                elements.push_back(entry.first);
        }
        return elements;
    }

    std::vector<std::vector<T>> allSets() {
        std::unordered_set<T> setRepresentatives;
        std::vector<std::vector<T>> sets;

        for (const auto& entry : parent) {
            T representative = *find(entry.first);
            if (setRepresentatives.find(representative) == setRepresentatives.end()) {
                std::vector<T> set;
                for (const auto& innerEntry : parent) {
                    if (*(find(innerEntry.first)) == representative)
                        set.push_back(innerEntry.first);
                }
                sets.push_back(set);
                setRepresentatives.insert(representative);
            }
        }

        return sets;
    }

    void reset() {
        parent.clear();
        rank.clear();
    }

    bool isSameSet(const T& element1, const T& element2) {
        T* parent1 = find(element1);
        T* parent2 = find(element2);

        if (parent1 == nullptr || parent2 == nullptr)
            return false;

        return *parent1 == *parent2;
    }

    void removeSet(const T& element) {
        T* representative = find(element);
        if (representative == nullptr)
            return;

        for (auto it = parent.begin(); it != parent.end(); /* no increment */) {
            if (*(find(it->first)) == *representative) {
                it = parent.erase(it);
                rank.erase(it->first);
            } else {
                ++it;
            }
        }
    }

    std::vector<T> pathToRoot(const T& element) {
        std::vector<T> path;
        const T* currentNode = &element;
        const T* representative = find(element);

        while (*currentNode != *representative) {
            path.push_back(*currentNode);
            const T* parent = find(*currentNode);
            if (parent == nullptr)
                return {};
            currentNode = parent;
        }
        path.push_back(*representative);
        return path;
    }

    std::vector<T> getRepresentatives() {
        std::unordered_set<T> representativesSet;
        std::vector<T> representatives;
        for (const auto& entry : parent) {
            T representative = *find(entry.first);
            if (representativesSet.find(representative) == representativesSet.end()) {
                representatives.push_back(representative);
                representativesSet.insert(representative);
            }
        }
        return representatives;
    }

    T* findAndCompress(const T& element) {
        T* representative = find(element);
        if (representative == nullptr)
            return nullptr;

        compressPath(element, *representative);
        return representative;
    }

    void compressPath(const T& element, const T& representative) {
        T currentNode = element;
        while (currentNode != representative) {
            auto parentIt = parent.find(currentNode);
            if (parentIt == parent.end())
                return;
            parent[currentNode] = representative;
            currentNode = parent[currentNode];
        }
    }

    void printSets() {
        std::unordered_map<T, std::vector<T>> sets;

        for (const auto& entry : parent) {
            T representative = *find(entry.first);
            sets[representative].push_back(entry.first);
        }

        for (const auto& entry : sets) {
            std::cout << "Set with representative " << entry.first << ": [";
            const std::vector<T>& elements = entry.second;
            for (size_t i = 0; i < elements.size(); ++i) {
                std::cout << elements[i];
                if (i < elements.size() - 1)
                    std::cout << ", ";
            }
            std::cout << "]\n";
        }
    }
};
