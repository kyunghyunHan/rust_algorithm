# 🚀 DTN Bundle Protocol 구조 정리

DTN(Delay/Disruption Tolerant Networking)은 연결이 끊기고 지연이 큰 환경에서 데이터를 안전하게 전달하기 위한 네트워크 방식이다.  
일반 인터넷이 **패킷(Packet)** 기반이라면, DTN은 **번들(Bundle)** 기반으로 동작한다.

---

## 📌 Bundle Protocol이란?

> **인터넷에는 패킷이 있고, DTN에는 번들이 있다.**

Bundle Protocol(BP)은 DTN에서 데이터를 묶어 전달하는 핵심 구조다.

---

## 🧱 Bundle의 구성 요소

Bundle은 크게 세 가지 영역으로 구성된다.


```
+------------------------------------------------+
| Primary Block (필수 헤더) |
+------------------------------------------------+
| Canonical Blocks (선택 확장) |
| - Payload Block (실제 데이터) |
| - Hop Count Block (노드 이동 횟수) |
| - Security Block (암호/서명) |
+------------------------------------------------+
```


---

## ✔ 1. Primary Block (필수 헤더)

**필수 정보 포함**
- 버전
- Source EID (보낸 주소)
- Destination EID (받는 주소)
- Creation time (생성 시각)
- Lifetime / TTL (유효기간)
- 처리 플래그 (전송 옵션)

---

## ✔ 2. Canonical Blocks (확장 기능)

필요에 따라 여러 개가 포함될 수 있음

| 블록 이름 | 역할 |
|----------|------|
| Payload Block | 실제 데이터(예: 이미지, 메시지 등) |
| Hop Count Block | 번들이 몇 노드를 거쳤는지 |
| Previous Node Block | 마지막으로 경유한 노드 |
| Metadata Block | 우선순위, 라우팅 힌트 등 |
| Security Block | 암호화, 인증 서명 |

**확장형 구조 → 필요한 기능만 선택해 사용 가능**

---

## 🛰️ Bundle 전달 예시 흐름

상황: **화성 로버 → 궤도 위성 → 지구**

1. 로버가 Bundle을 생성
