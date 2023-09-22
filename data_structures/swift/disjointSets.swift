class DisjointSets<T: Hashable> {
    private var parent: [T: T] = [:]
    private var rank: [T: Int] = [:]
    
    func makeSet(_ element: T) {
        if parent[element] == nil {
            parent[element] = element
            rank[element] = 0
        }
    }
    
    func find(_ element: T) -> T? {
        guard let currentParent = parent[element], currentParent != element else {
            return parent[element]
        }
        parent[element] = find(currentParent)
        return parent[element]
    }
    
    func union(_ element1: T, _ element2: T) {
        guard let parent1 = find(element1), let parent2 = find(element2), parent1 != parent2 else {
            return
        }
        
        if rank[parent1]! > rank[parent2]! {
            parent[parent2] = parent1
        } else if rank[parent1]! < rank[parent2]! {
            parent[parent1] = parent2
        } else {
            parent[parent2] = parent1
            rank[parent1]! += 1
        }
    }

    func contains(_ element: T) -> Bool {
        return parent[element] != nil
    }

    func setSize(_ element: T) -> Int {
        guard let representative = find(element) else { return 0 }
        return parent.values.filter { find($0) == representative }.count
    }

    func elementsInSameSet(_ element: T) -> [T] {
        guard let representative = find(element) else { return [] }
        return parent.keys.filter { find($0) == representative }
    }

    func allSets() -> [[T]] {
        var sets: [[T]] = []
        var setRepresentatives: Set<T> = Set()
        
        for element in parent.keys {
            let representative = find(element)!
            if !setRepresentatives.contains(representative) {
                let set = parent.keys.filter { find($0) == representative }
                sets.append(set)
                setRepresentatives.insert(representative)
            }
        }
        
        return sets
    }

    func reset() {
        parent.removeAll()
        rank.removeAll()
    }

    func isSameSet(_ element1: T, _ element2: T) -> Bool {
        guard let parent1 = find(element1), let parent2 = find(element2) else {
            return false
        }
        return parent1 == parent2
    }

    func removeSet(_ element: T) {
        guard let representative = find(element) else { return }
        for key in parent.keys {
            if find(key) == representative {
                parent[key] = nil
                rank[key] = nil
            }
        }
    }

    func pathToRoot(_ element: T) -> [T]? {
        guard let representative = find(element) else { return nil }
        var path: [T] = []
        var currentNode = element
        while currentNode != representative {
            path.append(currentNode)
            guard let parent = parent[currentNode] else { return nil }
            currentNode = parent
        }
        path.append(representative)
        return path
    }

    func getRepresentatives() -> [T] {
        return Array(Set(parent.values.compactMap { find($0) }))
    }

    func findAndCompress(_ element: T) -> T? {
        guard let representative = find(element) else { return nil }
        compressPath(element, representative)
        return representative
    }

    private func compressPath(_ element: T, _ representative: T) {
        var currentNode = element
        while currentNode != representative {
            guard let parent = parent[currentNode] else { return }
            parent[currentNode] = representative
            currentNode = parent
        }
    }

    func printSets() {
        var sets: [T: [T]] = [:]
        for element in parent.keys {
            guard let representative = find(element) else { continue }
            if sets[representative] == nil {
                sets[representative] = []
            }
            sets[representative]?.append(element)
        }

        for (representative, elements) in sets {
            print("Set with representative \(representative): \(elements)")
        }
    }
}
