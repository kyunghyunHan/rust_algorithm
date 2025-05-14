# Stack



- Push() : adds key to collection
- top() : returns most recently - added key
- pop() : removes and returns most recently-added key
- empty() : removes and returns most recently-added key

- 스택과 큐는 Delete연산에 의해 집합에서 삭제되는 원소가 미리 정해져있는 동적집합
- 스택에서는 가장 최근에 삽입된 원소가 먼저 삭제
- 후입선출(last-in-fist-out)

- push:insert
- POP:Delete

- S[1:S.top]
- S[1은 스택에맨 밑에 원소를 나타내고] S[S.top]은 맨 위에 있는 원소를 나타낸다

- S.top=0일떄 스택은 원소를 포함하지 않고 비었디(empty)라고한다
- 빈 스택에서 원소를 추출하려는 경우 스택부족(underflow)라고한다
- s.top이 원소의개수 S.size를 초과하면 overflow라고한다

## Study

- [3minute](https://www.youtube.com/watch?v=KcT3aVgrrpU)