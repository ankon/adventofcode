package utils

type Stack[T any] struct {
	data []T
}

func (s *Stack[T]) Top() T {
	return s.data[len(s.data)-1]
}

func (s *Stack[T]) Pop() T {
	end := len(s.data)-1
	result := s.data[end]
	s.data = s.data[:end]
	return result
}

func (s *Stack[T]) Push(x T) {
	s.data = append(s.data, x)
}

func (s *Stack[T]) Clone() Stack[T] {
	result := Stack[T]{
		data: append([]T{}, s.data...),
	}
	return result
}
