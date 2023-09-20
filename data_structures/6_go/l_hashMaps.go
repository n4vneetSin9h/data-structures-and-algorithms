package main

import (
	"fmt"
	"hash/fnv"
)

type KeyValue struct {
	key   interface{}
	value interface{}
}

type HashMap struct {
	buckets  [][]KeyValue
	capacity int
}

func NewHashMap(capacity int) *HashMap {
	return &HashMap{
		buckets:  make([][]KeyValue, max(1, capacity)),
		capacity: max(1, capacity),
	}
}

// Insertion
func (h *HashMap) SetValue(value interface{}, key interface{}) {
	index := h.bucketIndex(key)
	for i, kv := range h.buckets[index] {
		if kv.key == key {
			h.buckets[index][i].value = value
			return
		}
	}
	h.buckets[index] = append(h.buckets[index], KeyValue{key, value})
}

// Retrieval
func (h *HashMap) GetValue(key interface{}) interface{} {
	index := h.bucketIndex(key)
	for _, kv := range h.buckets[index] {
		if kv.key == key {
			return kv.value
		}
	}
	return nil
}

// Removal
func (h *HashMap) RemoveValue(key interface{}) {
	index := h.bucketIndex(key)
	var updated []KeyValue
	for _, kv := range h.buckets[index] {
		if kv.key != key {
			updated = append(updated, kv)
		}
	}
	h.buckets[index] = updated
}

// Count
func (h *HashMap) Count() int {
	totalCount := 0
	for _, bucket := range h.buckets {
		totalCount += len(bucket)
	}
	return totalCount
}

// Keys and Values
func (h *HashMap) AllKeys() []interface{} {
	var keys []interface{}
	for _, bucket := range h.buckets {
		for _, kv := range bucket {
			keys = append(keys, kv.key)
		}
	}
	return keys
}

func (h *HashMap) AllValues() []interface{} {
	var values []interface{}
	for _, bucket := range h.buckets {
		for _, kv := range bucket {
			values = append(values, kv.value)
		}
	}
	return values
}

// Helper Functions
func (h *HashMap) IsEmpty() bool {
	for _, bucket := range h.buckets {
		if len(bucket) > 0 {
			return false
		}
	}
	return true
}

func (h *HashMap) RemoveAll() {
	h.buckets = make([][]KeyValue, h.capacity)
}

func (h *HashMap) Contains(key interface{}) bool {
	index := h.bucketIndex(key)
	for _, kv := range h.buckets[index] {
		if kv.key == key {
			return true
		}
	}
	return false
}

func (h *HashMap) LoadFactor() float64 {
	return float64(h.Count()) / float64(h.capacity)
}

func (h *HashMap) KeyValuePairs() map[interface{}]interface{} {
	pairs := make(map[interface{}]interface{})
	for _, bucket := range h.buckets {
		for _, kv := range bucket {
			pairs[kv.key] = kv.value
		}
	}
	return pairs
}

// Update Values
func (h *HashMap) UpdateValue(value interface{}, key interface{}) {
	index := h.bucketIndex(key)
	for i, kv := range h.buckets[index] {
		if kv.key == key {
			h.buckets[index][i].value = value
			return
		}
	}
	h.buckets[index] = append(h.buckets[index], KeyValue{key, value})
}

func (h *HashMap) UpdateValues(dictionary map[interface{}]interface{}) {
	for key, value := range dictionary {
		h.UpdateValue(value, key)
	}
}

// ... Add more hashmap operations as needed ...

func (h *HashMap) bucketIndex(key interface{}) int {
	hash := fnv.New32a()
	fmt.Fprintf(hash, "%v", key)
	return int(hash.Sum32()) % h.capacity
}
