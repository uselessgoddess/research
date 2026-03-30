# Финальный отчёт: Распределённая система фарма кейсов CS2

## Оглавление

1. [Введение и архитектура системы](#1-введение-и-архитектура-системы)
2. [Анализ кастомного Rust-сервера (license repo)](#2-анализ-кастомного-rust-сервера-license-repo)
3. [Выбор ОС для VM: Linux + OpenBox](#3-выбор-ос-для-vm-linux--openbox)
4. [KVM/QEMU: VirtIO-GPU Venus — наш фаворит](#4-kvmqemu-virtio-gpu-venus--наш-фаворит)
5. [Спуфинг железа: MAC, серийники, SMBIOS](#5-спуфинг-железа-mac-серийники-smbios)
6. [Оптимизация памяти: zswap + KSM (приоритет)](#6-оптимизация-памяти-zswap--ksm-приоритет)
7. [CS2 в минимальном режиме (384×288)](#7-cs2-в-минимальном-режиме-384288)
8. [Управление VM из Rust: virt crate + VNC](#8-управление-vm-из-rust-virt-crate--vnc)
9. [Steam Login: автоматизация многоаккаунтного входа](#9-steam-login-автоматизация-многоаккаунтного-входа)
10. [Управление XML-конфигами libvirt из Rust](#10-управление-xml-конфигами-libvirt-из-rust)
11. [Архитектура воркера: Install Script](#11-архитектура-воркера-install-script)
12. [Параллельное управление VM и масштабирование](#12-параллельное-управление-vm-и-масштабирование)
13. [Коммуникация сервер-воркер: расширение Rust-сервера](#13-коммуникация-сервер-воркер-расширение-rust-сервера)
14. [Обнаружение VM и изоляция отпечатков](#14-обнаружение-vm-и-изоляция-отпечатков)
15. [Порядок реализации: Bottom-Up](#15-порядок-реализации-bottom-up)
16. [Итоговые рекомендации и архитектурная схема](#16-итоговые-рекомендации-и-архитектурная-схема)
17. [Список литературы](#17-список-литературы)

---

## 1. Введение и архитектура системы

Цель — построить распределённую систему, где:
- **Центральный сервер** (существующий Rust/Axum сервис `license`) управляет очередью аккаунтов и лицензиями
- **Воркер** (новый Rust-сервис) — физический хост с KVM/QEMU, создающий несколько VM с CS2
- **VM** — изолированная Linux-среда с Venus GPU, запускающая Steam + CS2
- **Управление VM** — воркер читает буфер дисплея через VNC, отправляет команды мышь/клавиатура снаружи

### Высокоуровневая схема

```
┌─────────────────────────────────────────────────────┐
│           Центральный сервер (Rust/Axum)             │
│  - REST API для управления (существующий)             │
│  - SQLite: лицензии, аккаунты, статусы               │
│  - Telegram bot (Teloxide) для администрирования     │
│  - Новое: API для воркеров                           │
└─────────────────┬───────────────────────────────────┘
                  │ REST / WebSocket
      ┌───────────┴─────────────┐
      ▼                         ▼
┌─────────────┐           ┌─────────────┐
│  Воркер 1   │           │  Воркер N   │
│  (Rust)     │           │  (Rust)     │
│ Ubuntu 22+  │           │ Ubuntu 22+  │
│ virt crate  │    ...    │ virt crate  │
│ ┌────┐┌───┐ │           │ ┌────┐┌───┐ │
│ │VM 1││VM2│ │           │ │VM 1││VM2│ │
│ │CS2 ││CS2│ │           │ │CS2 ││CS2│ │
│ └────┘└───┘ │           │ └────┘└───┘ │
└─────────────┘           └─────────────┘
```

### Ключевые решения (принятые)

| Решение | Выбор | Обоснование |
|---|---|---|
| ОС в VM | Linux (Debian 12 + OpenBox) | RAM в 5–8x меньше Windows, Venus только на Linux |
| GPU | VirtIO-GPU Venus | Единственный Vulkan без passthrough |
| Память | zswap + KSM | zswap — кеш перед NVMe swap, KSM — дедупликация |
| Язык | Rust | Весь стек на Rust: сервер, воркер, утилиты |
| Windows 10 | ❌ Не используем | Избыточный RAM, нет Venus, лицензия |

---

## 2. Анализ кастомного Rust-сервера (license repo)

### 2.1 Текущая архитектура

Репозиторий `uselessgoddess/license` — это Rust-сервис для управления лицензиями и статистикой, построенный на:

| Компонент | Технология |
|---|---|
| Web-фреймворк | Axum 0.8 |
| Async runtime | Tokio |
| ORM | SeaORM (SQLite) |
| Telegram бот | Teloxide |
| Rate limiting | tower_governor |
| HTTP клиент | reqwest / wreq |
| Сериализация | serde + serde_json |

### 2.2 Плагинная система

Сервер использует trait-based плагинную архитектуру. Каждый плагин реализует `Plugin` trait с async `start` методом и работает в отдельной tokio-задаче с автоматическим перезапуском (5 сек задержка).

Текущие плагины:
- **Server** — HTTP REST API (Axum)
- **Telegram** — бот для администрирования
- **Cron** — периодические задачи (GC сессий, бэкапы, очистка)
- **Steam** — мониторинг бесплатных игр/предметов

### 2.3 Существующие API-эндпоинты

| Метод | Путь | Назначение |
|---|---|---|
| GET | `/health` | Health check |
| POST | `/api/heartbeat` | Валидация сессии клиента, проверка лицензии |
| POST | `/api/logout` | Завершение сессии |
| POST | `/api/metrics` | Приём сжатой статистики (base64 + gzip JSON) |
| GET | `/api/download` | Скачивание билда по токену |
| GET | `/api/cache/steam/*` | Кешированные данные Steam |
| POST | `/api/telegram/*` | Proxy для Telegram API |

### 2.4 Управление сессиями

- In-memory хранение через `DashMap<String, Vec<Session>>`
- Сессия истекает через 120 секунд без heartbeat
- Лимит сессий на лицензию (настраиваемый)
- Magic token (FNV-1a хеш session_id + server secret) для аутентификации
- Бан-лист для недавно разлогиненных сессий (30 минут)

### 2.5 Схема базы данных

**Users** — tg_user_id, balance, role (user/creator/admin), реферальная система
**Licenses** — key (UUID), тип (trial/pro), expires_at, max_sessions, hwid_hash
**Stats** — weekly_xp, total_xp, drops_count, instances, runtime_hours, meta (JSON)
**Builds** — version, file_path, changelog, is_active, downloads

### 2.6 Что нужно добавить для воркеров

Существующий сервер уже имеет хорошую базу. Для интеграции с воркерами нужно:

1. **Новые API-эндпоинты:**
   - `POST /api/worker/register` — регистрация воркера
   - `POST /api/worker/heartbeat` — heartbeat воркера (отличается от клиентского)
   - `GET /api/worker/account` — атомарное получение аккаунта из очереди
   - `POST /api/worker/account/complete` — отчёт о завершении фарма
   - `POST /api/worker/account/error` — отчёт об ошибке аккаунта
   - `GET /api/worker/credentials/{account_id}` — получение Steam-креденшалов

2. **Новые таблицы:**
   - `workers` — id, name, status, last_heartbeat, max_vms, current_vms
   - `farm_accounts` — steam_username, encrypted_password, shared_secret, identity_secret, refresh_token, status (queued/farming/done/error)
   - `farm_sessions` — account_id, worker_id, vm_name, started_at, ended_at, xp_earned, drops

3. **Миграции:** SeaORM миграции в существующем формате (`migration/src/`)

### 2.7 Почему не нужен отдельный MQTT/Redis

Существующий стек (Axum + SQLite) достаточен для 10–50 воркеров:

| Подход | MQTT + Redis (из первого исследования) | Расширение Axum REST (рекомендация) |
|---|---|---|
| Дополнительные зависимости | Mosquitto, Redis | Нет |
| Сложность деплоя | 3 сервиса | 1 сервис |
| Масштабируемость | Отлично (1000+ воркеров) | Достаточно (до ~100 воркеров) |
| Латентность команд | Низкая (push) | Низкая (long-polling / WebSocket) |
| Уже реализовано | Ничего | Heartbeat, сессии, метрики |
| Единый стек | Нет (Python/MQTT + Rust) | **Да (всё на Rust)** |

**Вывод:** Для текущего масштаба (10–50 воркеров) расширение существующего Axum-сервера проще и эффективнее, чем добавление MQTT + Redis. При необходимости масштабирования можно добавить WebSocket-канал для push-команд.

---

## 3. Выбор ОС для VM: Linux + OpenBox

### Сравнительная таблица

| Параметр | Windows 10 LTSC | Linux + OpenBox |
|---|---|---|
| CS2 native поддержка | Да (DirectX 11) | **Да (Vulkan, first-party)** |
| VAC совместимость | Полная | **Полная (официальная поддержка Valve)** |
| Idle RAM | 900 MB – 1.5 GB | **150–350 MB** |
| Холодный запуск (SSD) | 20–40 сек | **5–15 сек** |
| Vulkan в VM (без passthrough) | Недоступен (Venus — только Linux guest) | **Venus layer (QEMU 9.2+)** |
| NVIDIA Error 43 в VM | Да (требует спуфинга) | **Нет** |
| Стоимость лицензии | Требуется | **Бесплатно** |
| Размер установки | ~20–30 GB (stripped) | **~3–8 GB** |

### Вывод: Linux + OpenBox ✅

Linux выигрывает по всем ключевым параметрам. Рекомендуемый стек:
- **OS:** Debian 12 minimal (без DE)
- **WM:** OpenBox
- **Display:** X11 (Xvfb или VNC output)
- **GPU driver:** Mesa 24.2+ с Venus

---

## 4. KVM/QEMU: VirtIO-GPU Venus — наш фаворит

CS2 на Linux **требует Vulkan** — OpenGL (virgl) недостаточен.

### Сравнение подходов к GPU в VM

| Подход | Vulkan | Для CS2 | Примечание |
|---|---|---|---|
| VirtIO-GPU virgl | Нет (OpenGL) | ❌ | Недостаточно |
| **VirtIO-GPU Venus** | **Да (Vulkan 1.3)** | **✅** | **QEMU 9.2+, kernel 6.13+** |
| QXL | Нет (2D) | ❌ | Только удалённый рабочий стол |
| llvmpipe | Да (CPU) | ⚠️ (<15 FPS) | Запасной вариант |
| GPU Passthrough (VFIO) | Да | ✅✅ | Лучшая производительность, но привязка к GPU |

### Конфигурация Venus

QEMU командная строка:
```
-device virtio-vga-gl,blob=on,hostmem=4G,venus=on
-vga none
-display vnc=127.0.0.1:0
-object memory-backend-memfd,id=mem1,size=4G
-machine memory-backend=mem1
```

Libvirt XML эквивалент:
```xml
<devices>
  <video>
    <model type='virtio'/>
  </video>
  <graphics type='vnc' port='-1' autoport='yes' listen='127.0.0.1'/>
</devices>
```

**Требования на хосте:**
- QEMU ≥ 9.2.0
- Linux kernel ≥ 6.13
- Mesa ≥ 24.2
- Host GPU с поддержкой Vulkan (NVIDIA, AMD GFX9+, Intel Gen12+)

---

## 5. Спуфинг железа: MAC, серийники, SMBIOS

Каждая VM должна выглядеть как уникальный физический компьютер. Все настройки задаются в XML libvirt.

### 5.1 MAC-адрес

```xml
<interface type='network'>
  <mac address='00:1A:A0:3F:B2:77'/>   <!-- Real Intel OUI, не 52:54:00 (QEMU) -->
  <source network='worker-net'/>
  <model type='e1000e'/>               <!-- e1000e, NOT virtio-net -->
</interface>
```

### 5.2 Серийный номер диска

```xml
<disk type='file' device='disk'>
  <source file='/path/to/vm.qcow2'/>
  <target dev='sda' bus='sata'/>
  <serial>WD-WXE1A80K3KJH</serial>
</disk>
```

### 5.3 SMBIOS (полный спуфинг)

```xml
<domain type='kvm' xmlns:qemu='http://libvirt.org/schemas/domain/qemu/1.0'>
  <sysinfo type='smbios'>
    <bios>
      <entry name='vendor'>American Megatrends International, LLC.</entry>
      <entry name='version'>F.70</entry>
      <entry name='date'>11/02/2021</entry>
    </bios>
    <system>
      <entry name='manufacturer'>ASUS</entry>
      <entry name='product'>ROG STRIX B550-F GAMING</entry>
      <entry name='serial'>M80ABCDEF01234</entry>
    </system>
    <baseBoard>
      <entry name='manufacturer'>ASUSTeK COMPUTER INC.</entry>
      <entry name='product'>ROG STRIX B550-F GAMING</entry>
      <entry name='serial'>M80ABCDEF01234</entry>
    </baseBoard>
  </sysinfo>

  <os>
    <type arch='x86_64' machine='q35'>hvm</type>
    <smbios mode='sysinfo'/>
  </os>

  <features>
    <hyperv mode='custom'>
      <vendor_id state='on' value='GenuineIntel'/>
    </hyperv>
    <kvm>
      <hidden state='on'/>
    </kvm>
    <vmport state='off'/>
  </features>

  <cpu mode='host-passthrough' check='none'>
    <topology sockets='1' dies='1' cores='4' threads='2'/>
    <feature policy='disable' name='hypervisor'/>
  </cpu>

  <devices>
    <memballoon model='none'/>
  </devices>

  <qemu:commandline>
    <qemu:arg value='-cpu'/>
    <qemu:arg value='host,model_id=Intel(R) Core(TM) i9-10900K CPU @ 3.70GHz,hypervisor=off'/>
  </qemu:commandline>
</domain>
```

### 5.4 Что нельзя спрятать программно

| Вектор | Статус |
|---|---|
| RDTSC timing overhead (~1000–5000 cycles в KVM) | Требует патча ядра (kvm-rdtsc-hack) |
| WMI thermal/fan sensors (пусты в VM) | Нет решения |
| ACPI table строки (BOCHS/BXPC) | Патч исходников QEMU (qemu-anti-detection) |

---

## 6. Оптимизация памяти: zswap + KSM (приоритет)

### 6.1 Почему zswap, а не zram

| Характеристика | zram | zswap | Swap на NVMe |
|---|---|---|---|
| Латентность | Микросекунды | **Микросекунды** | ~100µs |
| Нужен диск | Нет | Да (backing store) | Да |
| При переполнении | ❌ Зависание 20-30 мин / OOM | **✅ Вытесняет в swap** | Работает |
| LRU eviction | Нет | **Да (shrinker daemon)** | N/A |
| Для KVM-хостов с NVMe | Не рекомендуется | **✅ Рекомендуется** | Только как fallback |

**Вывод:** zswap + NVMe swap — приоритет. zram не используем (конфликтует с zswap).

### 6.2 Конфигурация zswap для KVM-хоста

Параметры ядра (GRUB):
```
zswap.enabled=1 zswap.compressor=zstd zswap.max_pool_percent=25 zswap.shrinker_enabled=1
```

Sysctl для VM-хоста (`/etc/sysctl.d/99-vm-host-zswap.conf`):
```ini
vm.swappiness = 120          # Предпочитать zswap (сжатый RAM) перед сбросом кешей
vm.vfs_cache_pressure = 200  # Агрессивная очистка кешей dentry/inode
vm.dirty_background_ratio = 5
vm.dirty_ratio = 20
vm.page-cluster = 0          # Нет readahead для сжатого swap
```

### 6.3 Выбор компрессора: zstd

| Компрессор | Коэффициент сжатия | Скорость | CPU-нагрузка |
|---|---|---|---|
| lz4 | ~2.6:1 | Максимальная | Минимальная |
| **zstd** | **~3.5:1** | Умеренная | Умеренная |
| lz4hc | ~3.0:1 | Медленная | Высокая |

**zstd рекомендуется** для VM-хостов: CPU хоста обычно недозагружен (нагрузка внутри VM), а лучшее сжатие означает больше страниц в пуле. 8 GB пул (25% от 32 GB) с zstd кеширует ~28 GB swap-данных.

### 6.4 KSM — дедупликация страниц между VM

KSM ищет одинаковые страницы памяти в разных VM и объединяет их (CoW).

**Экономия для одинаковых CS2 VM:** ~30–50% дедупликация.

Конфигурация:
```
echo 1    > /sys/kernel/mm/ksm/run
echo 1000 > /sys/kernel/mm/ksm/pages_to_scan
echo 50   > /sys/kernel/mm/ksm/sleep_millisecs
```

**KSM и zswap — комплементарные технологии:** KSM дедуплицирует активную RAM → меньше страниц нужно свопить → zswap сжимает только overflow.

### 6.5 Стек оптимизации памяти

| Слой | Технология | Эффект |
|---|---|---|
| Активная RAM | KSM | Дедупликация ~30-50% |
| Swap-кеш | zswap (zstd) | Сжатие overflow в RAM |
| Swap-устройство | NVMe swap (16-32 GB) | Последний рубеж |

### 6.6 Реальные требования CS2 по RAM

| Режим | RAM |
|---|---|
| CS2 процесс при idle/загрузке | **800 MB – 1.2 GB RSS** |
| CS2 во время матча | **1.5–2.0 GB RSS** |

**500 MB — нереально** для CS2. Минимальная цель — **1.5–2 GB на VM**, с KSM эффективно ~900 MB после конвергенции.

### 6.7 Бюджет памяти (32 GB хост, 8 VM)

| Слой | До | После | Экономия |
|---|---|---|---|
| Provisioned | 16 GB | 16 GB | — |
| После KSM (~40%) | 16 GB | ~10 GB | ~6 GB |
| zswap (25% = 8 GB) | — | Кеширует ~28 GB overflow | Избегает disk I/O |
| Host overhead | — | ~3 GB | — |
| **Итого нужно** | 16 GB | **~13 GB** | Влезает в 32 GB |

---

## 7. CS2 в минимальном режиме (384×288)

### 7.1 Параметры запуска

```
-window -w 384 -h 288 -noborder -novid -nojoy -nohltv \
-softparticlesdefaultoff -forcenovsync \
+fps_max 20 +fps_max_menu 5 \
+mat_queue_mode 2 +r_dynamic 0 +mat_disable_fancy_blending 1
```

| Параметр | Работает | Эффект |
|---|---|---|
| `-window -w 384 -h 288` | ✅ | Минимальный размер окна |
| `-noborder` | ✅ | Без рамки |
| `-novid` | ✅ | Пропустить intro видео |
| `-nojoy` | ✅ | Отключить геймпад |
| `-forcenovsync` | ✅ | Отключить VSync |
| `+fps_max 20` | ✅ | Ограничить FPS |
| `-noreactlogin` | ❌ | Удалён из Steam |

### 7.2 Система дропов CS2 (2026)

**Система Weekly Care Package:**
- Нужно зарабатывать XP через официальный matchmaking
- Каждые 5000 XP = повышение уровня = Care Package
- Сброс еженедельно по средам 00:00 UTC
- **Prime Status обязателен**
- ~30 минут активной игры в неделю для дропа

### 7.3 Запуск Steam headless

Steam запускается через виртуальный дисплей внутри VM:
```
Xvfb :99 -screen 0 1024x768x24 &
export DISPLAY=:99
steam -silent -nofriendsui &
```

**CS2 НЕ работает без GPU** — нужен Venus (QEMU 9.2+) или реальный GPU passthrough.

---

## 8. Управление VM из Rust: virt crate + VNC

### 8.1 Архитектура управления

```
┌─────────── Хост ─────────────────┐
│                                   │
│  ┌─────────────┐                 │
│  │   Воркер    │──────┐          │
│  │   (Rust)    │      │          │
│  └─────────────┘      │          │
│         ▲             ▼          │
│    read frame   send commands    │
│    (VNC)        (VNC/QMP)        │
│         │             │          │
│    ┌────┴─────────────┴───────┐  │
│    │    QEMU VNC server       │  │
│    └────────────────┬─────────┘  │
└─────────────────────┼────────────┘
                       │
┌─────────── VM ───────┼────────────┐
│                       │           │
│              CS2 window           │
└───────────────────────────────────┘
```

### 8.2 virt crate для управления VM

Официальные Rust-биндинги к libvirt (`virt` crate v0.4.3, 185K downloads):

```rust
use virt::connect::Connect;
use virt::domain::Domain;

// Подключение к libvirtd
let conn = Connect::open(Some("qemu:///system"))?;

// Создание VM из XML
let dom = Domain::define_xml(&conn, &xml)?;
dom.create()?; // запуск

// Мониторинг
let info = dom.get_info()?;
println!("State: {}, Memory: {} KB", info.state, info.memory);

// Graceful shutdown
dom.shutdown()?;
```

### 8.3 VNC для захвата экрана и ввода

Для Rust доступны VNC-клиенты через `vnc-rs` или FFI к `libvncclient`. Базовый подход:

```rust
// Псевдокод VNC-контроллера
struct VmController {
    vnc: VncClient,
    domain: Domain,
}

impl VmController {
    fn capture_frame(&mut self) -> Image {
        self.vnc.get_framebuffer()
    }

    fn click(&mut self, x: u16, y: u16) {
        self.vnc.pointer_event(x, y, ButtonMask::LEFT);
        self.vnc.pointer_event(x, y, ButtonMask::empty());
    }

    fn key_press(&mut self, key: u32) {
        self.vnc.key_event(key, true);
        self.vnc.key_event(key, false);
    }
}
```

### 8.4 Альтернатива: QEMU QMP через virt crate

`virt` crate с `qemu` feature даёт доступ к QMP:

```rust
// Скриншот через QMP
let result = dom.qemu_monitor_command(
    r#"{"execute":"screendump","arguments":{"filename":"/tmp/frame.ppm"}}"#,
    0
)?;
```

### 8.5 Сравнение методов

| Метод | Скриншот | Ввод | Непрерывный поток | Rust crate |
|---|---|---|---|---|
| **VNC** | ✅ | ✅ | ✅ | vnc-rs / libvncclient FFI |
| **QEMU QMP** | ✅ | ✅ | Нет (snapshots) | virt (feature=qemu) |
| libvirt API | ✅ | Нет | Нет | virt |

**Рекомендация:** VNC для непрерывного управления, QMP через `virt` crate как fallback для разовых скриншотов.

---

## 9. Steam Login: автоматизация многоаккаунтного входа

### 9.1 Архитектура входа

```
[Центральный сервер]
    │
    │── Хранит: username, password, shared_secret,
    │   identity_secret, refresh_token (зашифрованные)
    │
    ▼
[Воркер → VM]
    │
    │── Запрашивает креденшалы у сервера
    │── Пробует refresh_token (быстро, без 2FA)
    │── Fallback: credential + TOTP логин
    │── Отправляет новый refresh_token обратно серверу
```

### 9.2 Алгоритм входа для каждой VM

1. **VM стартует** → воркер запрашивает аккаунт у сервера
2. **Сервер даёт:** account_name, password, shared_secret, cached refresh_token
3. **Попытка #1 — refresh_token:** Если есть валидный refresh_token → вход без пароля/2FA
4. **Попытка #2 — credential + TOTP:** Генерация TOTP-кода из shared_secret → вход с предоставленным кодом
5. **Успех:** Новый refresh_token → кеширование на сервере
6. **Неудача:** Отчёт об ошибке, сервер ротирует аккаунт

### 9.3 Steam Guard (TOTP)

Steam использует нестандартный TOTP: 5-символьные алфавитно-цифровые коды (не стандартные 6-цифровые).

Для автоматизации нужен `shared_secret` каждого аккаунта. С ним генерация TOTP-кода полностью неинтерактивна.

### 9.4 Rust-крейты для Steam-аутентификации

| Крейт | Назначение | Статус |
|---|---|---|
| **steamguard** | TOTP + confirmations + encrypted storage | Активная разработка |
| steam-totp | Только генерация TOTP-кодов | Лёгкий, ~396 SLoC |

**Рекомендация:** Использовать `steamguard` crate для TOTP и confirmations. Для полного login session management — реализовать HTTP-клиент к Steam Auth API (по образцу node-steam-session) на reqwest/tokio.

### 9.5 Критические файлы для персистенции

| Файл | Назначение |
|---|---|
| `ssfn*` (sentry) | Machine authorization token |
| `config.vdf` | Кешированные креденшалы |
| `loginusers.vdf` | Список аккаунтов на машине |
| `maFiles/` | 2FA секреты (shared_secret, identity_secret) |

### 9.6 Ключевые ограничения

- **Rate limiting:** Steam ограничивает попытки входа. Распределять логины по времени.
- **IP-привязка:** Множественные входы с одного IP могут вызвать дополнительную верификацию → прокси при необходимости.
- **Machine ID:** Каждая VM должна иметь уникальный machine ID.
- **Token expiry:** Refresh токены долгоживущие, но истекают → нужен graceful refresh.

---

## 10. Управление XML-конфигами libvirt из Rust

### 10.1 Сравнение подходов

| Критерий | format!() шаблоны | serde + quick-xml | virt-install (shell) | XML + virsh (shell) |
|---|---|---|---|---|
| **Type safety** | Низкая | **Высокая (compile-time)** | Нет | Нет |
| **Сложность** | Минимальная | Высокая | Минимальная | Минимальная |
| **Гибкость** | Средняя | **Высокая** | Низкая | Низкая |
| **Ошибки** | Runtime (невалидный XML) | Compile-time (serde) | Parse stderr | Parse stderr |
| **Мониторинг VM** | **Полный (events, stats)** | **Полный** | Нет | Ручной |
| **Параллельность** | **Отличная (threaded)** | **Отличная** | Process overhead | Script overhead |

### 10.2 Рекомендация

**Использовать `virt` crate + format!() XML шаблоны:**

```rust
use virt::connect::Connect;
use virt::domain::Domain;

fn create_cs2_vm(
    conn: &Connect,
    name: &str,
    memory_mb: u64,
    vcpus: u32,
    disk_path: &str,
    mac_addr: &str,
    disk_serial: &str,
    smbios: &SmbiosConfig,
) -> Result<Domain, virt::error::Error> {
    let xml = format!(r#"
<domain type='kvm' xmlns:qemu='http://libvirt.org/schemas/domain/qemu/1.0'>
  <name>{name}</name>
  <memory unit='MiB'>{memory_mb}</memory>
  <vcpu placement='static'>{vcpus}</vcpu>

  <sysinfo type='smbios'>
    <system>
      <entry name='manufacturer'>{manufacturer}</entry>
      <entry name='product'>{product}</entry>
      <entry name='serial'>{sys_serial}</entry>
    </system>
  </sysinfo>

  <os>
    <type arch='x86_64' machine='q35'>hvm</type>
    <smbios mode='sysinfo'/>
  </os>

  <features>
    <kvm><hidden state='on'/></kvm>
    <vmport state='off'/>
  </features>

  <cpu mode='host-passthrough' check='none'>
    <feature policy='disable' name='hypervisor'/>
  </cpu>

  <devices>
    <disk type='file' device='disk'>
      <driver name='qemu' type='qcow2' cache='writeback'/>
      <source file='{disk_path}'/>
      <target dev='sda' bus='sata'/>
      <serial>{disk_serial}</serial>
    </disk>
    <interface type='network'>
      <mac address='{mac_addr}'/>
      <source network='default'/>
      <model type='e1000e'/>
    </interface>
    <graphics type='vnc' port='-1' autoport='yes' listen='127.0.0.1'/>
    <video><model type='virtio'/></video>
    <memballoon model='none'/>
  </devices>

  <qemu:commandline>
    <qemu:arg value='-cpu'/>
    <qemu:arg value='host,hypervisor=off'/>
  </qemu:commandline>
</domain>
"#,
        name = name,
        memory_mb = memory_mb,
        vcpus = vcpus,
        disk_path = disk_path,
        mac_addr = mac_addr,
        disk_serial = disk_serial,
        manufacturer = smbios.manufacturer,
        product = smbios.product,
        sys_serial = smbios.serial,
    );

    let dom = Domain::define_xml(conn, &xml)?;
    dom.create()?;
    Ok(dom)
}
```

**Зависимости для Cargo.toml воркера:**
```toml
[dependencies]
virt = { version = "0.4", features = ["qemu"] }
quick-xml = { version = "0.36", features = ["serialize"] }  # для будущего upgrade
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
steamguard = "0.14"
```

**Системные зависимости:** `libvirt-dev` (build), `libvirtd` (runtime).

### 10.3 Почему не shell-обёртки

Воркер управляет N VM параллельно — нужны:
- Event-driven мониторинг (lifecycle events через libvirt)
- Atomic операции (создание/удаление без race conditions)
- Shared connection (один `Connect` на все потоки)
- Proper error handling (типизированные ошибки, не parse stderr)

Всё это даёт `virt` crate, но невозможно через `virt-install` / `virsh` обёртки.

---

## 11. Архитектура воркера: Install Script

### Сравнение

| Критерий | Custom ISO (Packer) | Install Script |
|---|---|---|
| Начальная сложность | Высокая (дни) | **Низкая (часы)** |
| Скорость обновлений | Пересборка | **Push новый бинарник** |
| Привязка к ОС | Да (конкретная сборка) | **Нет (любой Ubuntu 22.04+)** |

### Вывод: Install Script

Воркер — один Rust-бинарник + systemd unit. Скрипт установки:

1. Проверяет зависимости (KVM, libvirt, QEMU ≥ 9.2)
2. Устанавливает недостающие пакеты
3. Скачивает бинарник воркера
4. Создаёт systemd service
5. Настраивает zswap + KSM
6. Подключается к серверу

Нет смысла привязываться к конкретной сборке ОС — install script работает на любом Ubuntu 22.04+ / Debian 12+.

---

## 12. Параллельное управление VM и масштабирование

### 12.1 Быстрое клонирование (qcow2 backing store)

```
# Базовый образ (один раз)
virt-sysprep -d worker-base

# Thin clone (~1 сек, ~1 MB)
qemu-img create -f qcow2 -F qcow2 \
  -b /var/lib/libvirt/images/base.qcow2 \
  /var/lib/libvirt/images/worker-06.qcow2
```

10 VM с одним 4 GB base = ~4 GB суммарно вместо 40 GB.

### 12.2 Ограничения ресурсов

CPU pinning и квоты (libvirt XML):
```xml
<cputune>
  <vcpupin vcpu='0' cpuset='2'/>
  <vcpupin vcpu='1' cpuset='3'/>
  <period>100000</period>
  <quota>150000</quota>   <!-- 1.5 ядра -->
</cputune>
```

I/O throttle:
```xml
<iotune>
  <total_iops_sec>300</total_iops_sec>
  <read_bytes_sec>31457280</read_bytes_sec>   <!-- 30 MB/s -->
  <write_bytes_sec>15728640</write_bytes_sec>  <!-- 15 MB/s -->
</iotune>
```

### 12.3 Мониторинг через virt crate

```rust
use virt::connect::Connect;
use virt::domain::Domain;

fn monitor_all_vms(conn: &Connect) -> Result<(), Box<dyn std::error::Error>> {
    let domains = conn.list_all_domains(0)?;
    for dom in &domains {
        let info = dom.get_info()?;
        let name = dom.get_name()?;
        println!("{}: state={}, memory={} KB, cpus={}",
            name, info.state, info.memory, info.nr_virt_cpu);

        // Memory stats
        if let Ok(stats) = dom.memory_stats(0, 0) {
            for stat in &stats {
                println!("  mem stat tag={}: {}", stat.tag, stat.val);
            }
        }
    }
    Ok(())
}
```

### 12.4 Сколько VM влезет

| Хост | VMs комфортно | VMs максимум (с KSM) |
|---|---|---|
| 16 GB RAM / 4 core | 4–5 | 6–7 |
| 32 GB RAM / 8 core | 8–10 | **12–16** |
| 64 GB RAM / 16 core | 18–22 | 28–32 |

---

## 13. Коммуникация сервер-воркер: расширение Rust-сервера

### 13.1 Протокол: REST (существующий Axum) + WebSocket (для push)

| Операция | Метод | Инициатор |
|---|---|---|
| Регистрация воркера | POST `/api/worker/register` | Воркер |
| Heartbeat | POST `/api/worker/heartbeat` | Воркер (каждые 30 сек) |
| Получение аккаунта | GET `/api/worker/account` | Воркер (poll) |
| Отчёт о завершении | POST `/api/worker/account/complete` | Воркер |
| Срочные команды | WebSocket `/ws/worker` | Сервер (push) |

### 13.2 Аутентификация воркера

Существующая система `magic_token` (FNV-1a hash) может быть переиспользована. Каждый воркер получает API-ключ при регистрации.

```rust
// Воркер: заголовок авторизации
let client = reqwest::Client::new();
let resp = client.get(&format!("{}/api/worker/account", server_url))
    .header("Authorization", format!("Bearer {}", api_key))
    .send()
    .await?;
```

### 13.3 Очередь аккаунтов (SQLite)

Вместо Redis — SQLite (уже используется в сервере):

```sql
-- Атомарное получение аккаунта
UPDATE farm_accounts
SET status = 'farming', worker_id = ?, started_at = datetime('now')
WHERE id = (
    SELECT id FROM farm_accounts WHERE status = 'queued' LIMIT 1
)
RETURNING *;
```

SQLite с WAL mode обеспечивает достаточную конкурентность для 10–50 воркеров.

---

## 14. Обнаружение VM и изоляция отпечатков

### 14.1 Векторы детекции и митигация

| Вектор | Сложность фикса | Решение |
|---|---|---|
| CPUID hypervisor bit | ✅ Легко | `<feature policy='disable' name='hypervisor'/>` |
| KVM CPUID leaf | ✅ Легко | `<kvm><hidden state='on'/></kvm>` |
| MAC адрес (QEMU OUI) | ✅ Легко | Реальный vendor OUI |
| NIC model (virtio = VEN_1AF4) | ✅ Легко | `<model type='e1000e'/>` |
| Disk serial ("QEMU HARDDISK") | ✅ Легко | Кастомный `<serial>` |
| SMBIOS strings | ✅ Легко | `<sysinfo>` блок |
| RDTSC timing | ❗ Сложно | kvm-rdtsc-hack |
| ACPI table строки (BOCHS) | ❗ Сложно | qemu-anti-detection |
| WMI thermal/fan sensors | ❌ Нерешаемо | — |

### 14.2 Против VAC

VAC — серверный античит без kernel-level доступа. Базовый спуфинг (SMBIOS + CPUID) **достаточен**.

### 14.3 Сетевая изоляция

| Вариант | Сложность | Качество |
|---|---|---|
| Все VM через один IP (NAT) | Нет затрат | Плохая |
| WireGuard VPN per VM | Умеренная | Хорошая |
| Residential proxy per VM | Простая | Хорошая |
| Отдельные IP от провайдера | Средняя | Отличная |

---

## 15. Порядок реализации: Bottom-Up

### Сравнение подходов

| Критерий | Top-Down (Сервер → Воркер → VM) | Bottom-Up (VM → Воркер → Сервер) |
|---|---|---|
| Первый результат | Позже | **Быстрее** |
| Проверка концепции | Откладывается | **Сразу** |
| Риск "мёртвого кода" | Высокий | **Низкий** |
| Обнаружение проблем | Поздно | **Рано** |
| Мотивация | Низкая | **Высокая** |

### Вывод: Начинать снизу (Bottom-Up) ✅

**Критические риски внизу:** Venus GPU, CS2 в VM, zswap + KSM, Steam login — всё нужно проверить до написания серверного API. Если Venus не работает стабильно — вся архитектура меняется.

### Дорожная карта

**Фаза 1: Proof of Concept (1–2 недели)**
- [ ] Установить QEMU 9.2+ с Venus на хост
- [ ] Создать базовый образ VM (Debian 12 + OpenBox)
- [ ] Проверить Venus GPU (Vulkan) в гостевой ОС
- [ ] Установить Steam + CS2, запустить в 384×288
- [ ] Измерить реальное RAM (CS2 + ОС)
- [ ] Настроить zswap + KSM, запустить 2–3 VM
- [ ] Протестировать Steam login через `steamguard` crate

**Фаза 2: Воркер (2–3 недели)**
- [ ] Rust-проект воркера с `virt` crate
- [ ] Создание/уничтожение VM из XML-шаблона
- [ ] VNC-захват экрана + ввод
- [ ] Конвейер: VM → загрузка → Steam login → CS2
- [ ] Мониторинг VM (heartbeat, crash detection)
- [ ] Параллельное управление N VM

**Фаза 3: Интеграция с сервером (1–2 недели)**
- [ ] API для воркеров в license repo
- [ ] Протокол воркер ↔ сервер
- [ ] Очередь аккаунтов (SQLite)
- [ ] Интеграционное тестирование

**Фаза 4: Продакшн (1–2 недели)**
- [ ] Install script для деплоя воркера
- [ ] Мониторинг (Prometheus + Grafana)
- [ ] Обработка ошибок, авто-восстановление
- [ ] Масштабирование на несколько хостов

---

## 16. Итоговые рекомендации и архитектурная схема

### Технологический стек (всё на Rust)

| Компонент | Выбор | Обоснование |
|---|---|---|
| Сервер | Axum 0.8 + SeaORM (SQLite) | Уже реализован (license repo) |
| Бот | Teloxide (Telegram) | Уже реализован |
| Воркер | Rust + virt crate + VNC client | Type-safe управление VM |
| Host OS | Ubuntu 22.04+ LTS | Стабильный KVM/libvirt |
| VM OS | Debian 12 minimal + OpenBox | ~200 MB idle |
| VM GPU | VirtIO-GPU Venus | Единственный Vulkan без passthrough |
| VM память | 2 GB max + KSM + zswap | 30-50% экономия через KSM |
| VM клонирование | qcow2 backing store | ~1 сек, минимальное место |
| Steam login | steamguard crate + TOTP | Неинтерактивный вход |
| Коммуникация | REST (Axum) + WebSocket | Единый стек, нет лишних зависимостей |
| Спуфинг | SMBIOS + CPUID hide + e1000e | Достаточно против VAC |
| Мониторинг | Prometheus + libvirt-exporter + Grafana | Стандартный стек |

### Полная схема

```
┌────────────────────────────────────────────────────────┐
│              Центральный сервер (Rust/Axum)              │
│                                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ REST API     │  │   SQLite     │  │  Teloxide    │  │
│  │ (Axum)       │  │ (лицензии,  │  │  (Telegram)  │  │
│  │ + WebSocket  │  │  аккаунты)  │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└──────────────────────────┬─────────────────────────────┘
                            │ REST + WS
              ┌─────────────┼─────────────┐
              ▼             ▼             ▼
         ┌──────────┐  ┌──────────┐  ┌──────────┐
         │ Воркер 1 │  │ Воркер 2 │  │ Воркер N │
         │ (Rust)   │  │ (Rust)   │  │ (Rust)   │
         │ virt     │  │ virt     │  │ virt     │
         │ crate    │  │ crate    │  │ crate    │
         │          │  │          │  │          │
         │ KVM VMs: │  │ KVM VMs: │  │ KVM VMs: │
         │ ┌──┐┌──┐ │  │ ┌──┐┌──┐ │  │ ┌──┐┌──┐ │
         │ │v1││v2│ │  │ │v1││v2│ │  │ │v1││v2│ │
         │ └──┘└──┘ │  │ └──┘└──┘ │  │ └──┘└──┘ │
         └──────────┘  └──────────┘  └──────────┘
```

### Зависимости (Cargo.toml воркера)

```toml
[dependencies]
# VM management
virt = { version = "0.4", features = ["qemu"] }

# Async
tokio = { version = "1", features = ["full"] }

# HTTP client (для связи с сервером)
reqwest = { version = "0.12", features = ["json"] }

# Steam auth
steamguard = "0.14"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# XML (для будущего типизированного XML)
quick-xml = { version = "0.36", features = ["serialize"] }

# Error handling
anyhow = "1.0"
thiserror = "2.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## 17. Список литературы

### Rust-сервер и архитектура
- uselessgoddess/license — кастомный Rust-сервер (Axum + SeaORM + Teloxide)
- https://crates.io/crates/virt — официальные Rust-биндинги к libvirt (185K downloads)
- https://gitlab.com/libvirt/libvirt-rust — исходный код virt crate (557 commits)
- https://docs.rs/virt/latest/virt/ — API документация
- https://crates.io/crates/quick-xml — XML обработка (50x быстрее xml-rs)

### Steam аутентификация
- https://github.com/dyc3/steamguard-cli — Rust CLI для Steam 2FA (steamguard crate)
- https://github.com/DoctorMcKay/node-steam-session — reference implementation для Steam login
- https://github.com/DoctorMcKay/node-steam-totp — Steam TOTP генерация
- https://partner.steamgames.com/doc/features/auth — официальная документация Steam Auth

### KVM/QEMU и Venus
- https://docs.mesa3d.org/drivers/venus.html — Venus (Vulkan в VM)
- https://wiki.archlinux.org/title/QEMU/Guest_graphics_acceleration — GPU в QEMU
- https://www.collabora.com/news-and-blog/blog/2025/01/15/the-state-of-gfx-virtualization-using-virglrenderer/ — состояние GPU виртуализации
- https://libvirt.org/formatdomain.html — формат domain XML
- https://qemu-project.gitlab.io/qemu/interop/qemu-qmp-ref.html — QEMU QMP Reference

### Оптимизация памяти
- https://www.kernel.org/doc/html/latest/admin-guide/mm/zswap.html — документация ядра по zswap
- https://wiki.archlinux.org/title/Zswap — Arch Wiki zswap
- https://wiki.archlinux.org/title/Zram — Arch Wiki zram (для сравнения)
- https://www.linux-kvm.org/page/KSM — KSM для KVM
- https://pve.proxmox.com/wiki/Kernel_Samepage_Merging_(KSM) — KSM (Proxmox)

### CS2 и Steam
- https://store.steampowered.com/app/730/CounterStrike_2/ — системные требования CS2
- https://tradeit.gg/blog/cs2-drop-pool/ — система дропов CS2
- https://totalcsgo.com/launch-options — параметры запуска CS2

### Спуфинг и anti-detection
- https://github.com/zhaodice/qemu-anti-detection — патч QEMU
- https://secret.club/2020/04/13/how-anti-cheats-detect-system-emulation.html — детекция VM
- https://github.com/h33p/kvm-rdtsc-hack — патч ядра для RDTSC

### Деплоймент
- https://developer.hashicorp.com/packer/guides/automatic-operating-system-installs/preseed_ubuntu — Packer
- https://ubuntu.com/server/docs/explanation/intro-to/cloud-init/ — cloud-init
