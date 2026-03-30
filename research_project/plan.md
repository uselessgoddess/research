# Research Plan: Distributed CS2 Case Farming System

## Main Topic
Design and architecture of a distributed system for farming CS2 cases using KVM/QEMU virtual machines, with GPU forwarding, hardware spoofing, memory optimization, and remote control. All components built in Rust.

## Research Subtopics / Questions

### Стартовое исследование (Phase 1)
- [x] 1. **VM OS Choice**: Windows 10 LTSC vs Linux + OpenBox → **Linux выбран**
- [x] 2. **KVM/QEMU Setup**: Venus GPU, kernel 6.13+, QEMU 9.2+
- [x] 3. **Hardware Spoofing**: MAC, серийники, SMBIOS, CPUID hiding
- [x] 4. **Memory Optimization**: zswap + KSM → **zswap приоритет**
- [x] 5. **CS2 Low-Res Mode**: 384×288, параметры запуска, система дропов
- [x] 6. **Input Injection & Display Capture**: VNC + QMP из Rust
- [x] 7. **Worker Architecture**: Install script → **без привязки к сборке ОС**
- [x] 8. **Parallel VM Management**: qcow2 cloning, ресурсные лимиты
- [x] 9. **Server-Worker Communication**: REST/WebSocket → **расширение Axum-сервера**
- [x] 10. **Security & Evasion**: Спуфинг достаточен для VAC

### Анализ Rust RPC и новые темы (Phase 2)
- [x] 11. **Steam Login Automation**: TOTP, refresh tokens, steamguard crate
- [x] 12. **Rust VM Management**: virt crate, XML-шаблоны, сравнение подходов
- [x] 13. **zswap Configuration**: Детальная настройка zswap + KSM для KVM-хоста
- [x] 14. **Implementation Order**: Bottom-Up подход с дорожной картой из 4 фаз

## Output Files
### Стартовое исследование
- `notes/topic_1_os_choice.md`
- `notes/topic_2_kvm_setup.md`
- `notes/topic_3_hardware_spoofing.md`
- `notes/topic_4_memory_optimization.md`
- `notes/topic_5_cs2_lowres.md`
- `notes/topic_6_input_display.md`
- `notes/topic_7_worker_arch.md`
- `notes/topic_8_parallel_vms.md`
- `notes/topic_9_server_worker_comm.md`
- `notes/topic_10_security.md`

### Анализ Rust RPC (обновлённое исследование)
- `notes/topic_11_steam_login.md`
- `notes/topic_12_rust_vm_management.md`
- `notes/topic_13_zswap_config.md`
- `notes/topic_14_implementation_order.md`

### Финальный отчёт
- `FINAL_REPORT.md` (обновлён: Rust-стек, без bash/python, новые разделы)
