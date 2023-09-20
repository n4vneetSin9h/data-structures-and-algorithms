package main

import (
	"fmt"
	"hash/fnv"
)

type HashTable struct {
	capacity int
	buckets  [][]pair
}

type pair struct {
	key   interface{}
	value interface{}
}

func NewHashTable(capacity int) *HashTable {
	if capacity <= 0 {
		panic("Capacity should be greater than 0")
	}

	buckets := make([][]pair, capacity)
	for i := range buckets {
		buckets[i] = []pair{}
	}

	return &HashTable{
		capacity: capacity,
		buckets:  buckets,
	}
}

func (ht *HashTable) bucketIndex(key interface{}) int {
	h := fnv.New32a()
	fmt.Fprintf(h, "%v", key)
	hash := h.Sum32()
	return int(hash) % ht.capacity
}

func (ht *HashTable) GetValue(key interface{}) interface{} {
	index := ht.bucketIndex(key)
	for _, p := range ht.buckets[index] {
		if p.key == key {
			return p.value
		}
	}
	return nil
}

func (ht *HashTable) SetValue(key interface{}, value interface{}) {
	index := ht.bucketIndex(key)

	// Check if the key already exists, and update the value
	for i, p := range ht.buckets[index] {
		if p.key == key {
			ht.buckets[index][i] = pair{key, value}
			return
		}
	}

	// Key doesn't exist, add a new entry
	ht.buckets[index] = append(ht.buckets[index], pair{key, value})
}

func (ht *HashTable) RemoveValue(key interface{}) {
	index := ht.bucketIndex(key)

	// Remove the key if it exists
	var newBucket []pair
	for _, p := range ht.buckets[index] {
		if p.key != key {
			newBucket = append(newBucket, p)
		}
	}
	ht.buckets[index] = newBucket
}

func (ht *HashTable) RemoveAll() {
	for i := range ht.buckets {
		ht.buckets[i] = []pair{}
	}
}

func (ht *HashTable) Count() int {
	count := 0
	for _, bucket := range ht.buckets {
		count += len(bucket)
	}
	return count
}

func (ht *HashTable) Contains(key interface{}) bool {
	index := ht.bucketIndex(key)
	for _, p := range ht.buckets[index] {
		if p.key == key {
			return true
		}
	}
	return false
}

func (ht *HashTable) AllKeys() []interface{} {
	var keys []interface{}
	for _, bucket := range ht.buckets {
		for _, p := range bucket {
			keys = append(keys, p.key)
		}
	}
	return keys
}

func (ht *HashTable) AllValues() []interface{} {
	var values []interface{}
	for _, bucket := range ht.buckets {
		for _, p := range bucket {
			values = append(values, p.value)
		}
	}
	return values
}

func (ht *HashTable) Merge(otherTable *HashTable) {
	for _, bucket := range otherTable.buckets {
		for _, p := range bucket {
			ht.SetValue(p.key, p.value)
		}
	}
}

func (ht *HashTable) Resize(newCapacity int) {
	if newCapacity <= 0 {
		panic("New capacity should be greater than 0")
	}

	newBuckets := make([][]pair, newCapacity)
	for i := range newBuckets {
		newBuckets[i] = []pair{}
	}

	for _, bucket := range ht.buckets {
		for _, p := range bucket {
			newIndex := ht.bucketIndex(p.key)
			newBuckets[newIndex] = append(newBuckets[newIndex], p)
		}
	}

	ht.buckets = newBuckets
}

func (ht *HashTable) PrintHashTable() {
	for i, bucket := range ht.buckets {
		fmt.Printf("Bucket %d: ", i)
		for _, p := range bucket {
			fmt.Printf("(%v, %v) ", p.key, p.value)
		}
		fmt.Println()
	}
}
