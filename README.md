<div align="center">

```
██╗  ██╗ ██████╗ ██████╗ ██╗███████╗ ██████╗ ███╗   ██╗
██║  ██║██╔═══██╗██╔══██╗██║╚══███╔╝██╔═══██╗████╗  ██║
███████║██║   ██║██████╔╝██║  ███╔╝ ██║   ██║██╔██╗ ██║
██╔══██║██║   ██║██╔══██╗██║ ███╔╝  ██║   ██║██║╚██╗██║
██║  ██║╚██████╔╝██║  ██║██║███████╗╚██████╔╝██║ ╚████║
╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝╚══════╝ ╚═════╝ ╚═╝  ╚═══╝
               A N D R O I D   E D I T I O N
```

# Horizon Android

**High-Performance Native Android Edition of the Horizon Minecraft Java 26.3 Client**  
*Pure Ahead-of-Time Rust · Direct Vulkan Pipeline · Zero JVM Overhead*

[![Platform](https://img.shields.io/badge/Platform-Android%209%2B%20(API%2028%2B)-2ea44f?style=flat-square)](#system-requirements)
[![Architecture](https://img.shields.io/badge/Architecture-ARM64--v8a-blue?style=flat-square)](#system-requirements)
[![Engine](https://img.shields.io/badge/Engine-Native%20Rust%202024-B7410E?style=flat-square&logo=rust&logoColor=white)](#system-requirements)
[![Renderer](https://img.shields.io/badge/Renderer-Vulkan%201.1%2B-red?style=flat-square&logo=vulkan&logoColor=white)](#system-requirements)
[![Target](https://img.shields.io/badge/Minecraft-Java%2026.3-5D9C3F?style=flat-square)](#system-requirements)
[![License](https://img.shields.io/badge/License-MIT-blueviolet?style=flat-square)](LICENSE)

[Download APK](https://github.com/tubers9999/apex/releases) · [Release Notes](https://github.com/tubers9999/apex/releases/latest) · [Issue Tracker](https://github.com/tubers9999/apex/issues)

</div>

---

> [!IMPORTANT]
> ### 🤝 Collaboration & Upstream Synergy Notice
> **Horizon Android** is the dedicated, community-driven mobile port of the official [Horizon](https://github.com/horizon-rust/horizon) engine.
>
> - **PC Edition:** Created and maintained upstream by [Ludovic Marie (horizon-rust)](https://github.com/horizon-rust/horizon).
> - **Android Edition:** Actively ported, optimized, and maintained for ARM64 mobile devices by **RedGlitchX** ([@tubers9999](https://github.com/tubers9999)).
>
> 💡 *„The upstream team focuses on PC development, while I take care of the Android version. Whenever the PC version receives updates, new features, or engine improvements, we sync, adapt, and release the corresponding Android updates so mobile players always enjoy the latest Horizon experience.”*
>
> Upstream developers and contributors: feel free to coordinate with us when new upstream releases or breaking changes land!

---

## [>] Overview

**Horizon Android** is an ultra-fast, native Android port of the Minecraft Java Edition (target version 26.3) engine written from scratch in pure **Rust**. Unlike conventional wrappers that run desktop Java binaries inside an emulated JVM / Dalvik environment on top of Android, Horizon compiles directly to native AArch64 machine code, rendering via Vulkan with zero Java Virtual Machine overhead.

The result is instant cold boots (<1.5 seconds), minimal RAM utilization (~150MB), silky-smooth frame pacing, and competitive mobile-friendly features designed specifically for phone and tablet hardware.

---

## [>] Performance & Architecture Comparison

| Metric / Dimension | Horizon Android | PojavLauncher | Minecraft Bedrock |
| :--- | :---: | :---: | :---: |
| **Core Architecture** | Native Ahead-of-Time Rust | OpenJDK HotSpot JVM | Native C++ |
| **Memory Footprint** | **~140 - 220 MB** | 1,200 - 2,500 MB | ~400 - 800 MB |
| **Cold Boot Duration** | **< 1.5 seconds** | 15 - 35 seconds | 4 - 8 seconds |
| **Graphics Pipeline** | **Direct Vulkan (WGSL/Naga)** | OpenGL via GL4ES | Direct3D / Vulkan |
| **Data Packs & Mods** | **Integrated Modrinth Catalog** | Manual file transfers | Add-ons only |
| **Built-in Keystrokes & CPS** | **Integrated Native HUD** | Requires Forge/Fabric mods | Unavailable |
| **1-Tap Ultra FPS Booster** | **Native Culling Engine** | Requires external mods | Limited sliders |
| **PvP Hitmarker Sound** | **Zero-latency PCM Synthesizer** | Requires mod packs | Resource pack hack |
| **Touch Layout Engine** | **GoyControl & MCPE Polygons** | Virtual Java keyboard | Fixed presets |
| **Modding Support** | **Universal .hmod, .hnmod, .jar** | Standard Java jars | Add-ons / scripts |

---

## [>] Key Features

### [01] Universal Mod & Data Pack Support
* **In-App Modrinth Browser:** Search, browse, and 1-tap install mods, resource packs, shaders, and data packs directly from Modrinth without manual file downloads.
* **Automated Data Pack Injection:** Data packs are placed directly into `datapacks/`, replicated into world saves, and automatically set to `enabled` in `datapacks.json`.
* **Out-of-the-Box Mod Activation:** Pre-bundled mods (Minimap, Zoom, Recipes, SmartF3) active out-of-the-box, plus full discovery and loading of `.hmod`, `.hnmod`, `.jar`, and `.zip` content mods.

### [02] Competitive PvP Suite
* **Integrated Hitmarker Audio ("Ack" Sound):** Custom synthesized 90ms high-transient audio cue triggered immediately upon landing attacks on hostile mobs, aggressive targets, or enemy players in PvP. Completely in-memory with 0ms I/O latency.
* **Live Keystrokes HUD:** Real-time visual display for `[ W ]`, `[ A ]`, `[ S ]`, `[ D ]` and `[ SPACE ]`, illuminated in signature Crimson upon keypress.
* **Sliding-Window CPS Counters:** Continuous 1,000ms sliding-window frequency trackers for both Left-Click (LMB) and Right-Click (RMB) with live on-screen telemetry.

### [03] 1-Tap Ultra FPS Booster
* **Instant Hardware Optimization:** A single toggle in the Floating Hub instantly activates aggressive entity culling, cave occlusion culling, animated texture freezing, and GPU texture LOD bias.
* **Stable 60 - 120+ FPS:** Engineered specifically for mid-range and budget ARM Mali and Qualcomm Adreno chipsets to eliminate stuttering in heavy multiplayer lobbies.

### [04] Atmosphere & Built-in Launcher Media
* **High-Octane 135 BPM PvP Soundtrack:** Genuine high-energy Minecraft PvP montage loop with punchy 4-on-the-floor kick, rolling 16th-note hi-hats, distorted 808/saw bassline, and seamless crossfade.
* **60 FPS Live Animated Video Background:** Smooth undulating plasma waves, floating cyber embers/sparks, horizon neon grid glow, and liquid glass frosted sheen.
* **Universal Wallpaper Decoding:** Native support for PNG, JPEG, JPG, WebP, and GIF wallpapers with dynamic cache busting.
* **Device Storage Integration (SAF):** Full Android DocumentsProvider integration allowing 1-tap import of custom wallpapers, videos, and audio.

### [05] Dual-Mode Touch Engine & GoyControl
* **Comprehensive Aiming Modes:** Joystick or D-Pad navigation paired with either gesture tap-aiming or precision crosshair targeting.
* **Polygon Button Geometry:** Choose between Circle, Hexagon, Octagon, Square, and Diamond button outlines with granular opacity, scale, and color customization.
* **Glitch-Free State Transitions:** Elimination of touch latching bugs when toggling between control presets, opening modals, or switching between virtual mouse and touch aiming.

---

## [>] Gameplay & Visual Previews

| Joystick & Crosshair | D-Pad & Touch Aiming |
| :---: | :---: |
| ![Joystick Controls](docs/media/android/joystick-fox.png) | ![D-pad Controls](docs/media/android/dpad-river.png) |

| Hypixel Arcade Multiplayer | Landscape World Generation |
| :---: | :---: |
| ![Hypixel Arcade](docs/media/android/hypixel-arcade.png) | ![Snowy Landscape](docs/media/android/snow-landscape.png) |

---

## [>] Installation & Quick Start

1. **Download:** Grab the latest `horizon-arm64.apk` from [Releases](https://github.com/tubers9999/apex/releases).
2. **Install:** Install the APK onto your Android device (Android 9.0 or newer with ARM64 architecture).
3. **Launch:** Open Horizon. The client automatically downloads required vanilla Mojang assets and touch textures during the initial cold boot.
4. **Configure:** Use the launcher's *Appearance* and *Controls* tabs to select your preferred touch scheme, button shapes, and audio preferences.

---

## [>] Custom Theme Storage Structure

To supply your own custom media without using the in-app picker, place your assets directly into the client's internal storage:

```
Android/data/dev.horizon.android/files/
├── config.json              # Core settings and key bindings
├── saves/                   # Singleplayer world saves
├── mods/                    # Sandboxed .hmod native modules
├── datapacks/               # Active Minecraft data packs
└── theme/                   # Custom appearance and audio assets
    ├── background.png       # Custom launcher wallpaper (PNG/JPG/WebP/GIF)
    ├── background.mp4       # Custom animated launcher backdrop (MP4)
    ├── bgm.ogg              # Custom launcher & PvP background music (OGG)
    ├── hit.ogg              # Custom hitmarker sound override (OGG)
    └── ack.ogg              # Alternative hitmarker audio fallback (OGG)
```

---

## [>] System Requirements

* **Operating System:** Android 9.0 (API level 28) or higher
* **CPU Architecture:** 64-bit ARM (`aarch64-linux-android` / ARMv8-A or newer)
* **Graphics API:** Vulkan 1.1 or higher (Adreno 500+, Mali-G series, Immortalis)
* **RAM:** 2 GB minimum (3 GB+ recommended for extended render distances)
* **Storage:** ~500 MB free storage for client binaries and cached vanilla assets

---

## [>] License & Copyright

**Copyright (c) 2026 RedGlitchX. All rights reserved.**

Upstream Horizon engine by [Ludovic Marie (horizon-rust)](https://github.com/horizon-rust/horizon). Dependencies retain their respective licenses as enumerated in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Minecraft is a registered trademark of Mojang Synergies AB. This project is not affiliated with or endorsed by Mojang or Microsoft.
