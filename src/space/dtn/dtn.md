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

