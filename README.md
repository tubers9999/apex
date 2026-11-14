# Horizon

**A native Ahead-of-Time Rust client for Minecraft Java Edition 26.3.**

Horizon is an independent reimplementation of the Minecraft Java Edition engine written in Rust. It does not run desktop Java bytecode inside an emulated JVM or Dalvik wrapper; every subsystem compiles directly to native AArch64 and x86_64 machine code, rendering via Vulkan with low overhead.

[Releases](https://github.com/tubers9999/apex/releases) · [Documentation](docs/) · [Upstream Repository](https://github.com/horizon-rust/horizon)

---

### Unofficial Project Notice

Horizon is an independent community project. It is not affiliated with, endorsed by, or connected to Mojang Studios, Microsoft Corporation, or any of their subsidiaries. Minecraft is a registered trademark of Mojang Synergies AB.

- **PC Edition:** Created and maintained upstream by [Ludovic Marie (horizon-rust)](https://github.com/horizon-rust/horizon).
- **Android Port:** Maintained and optimized for ARM64 mobile hardware by [RedGlitchX (@tubers9999)](https://github.com/tubers9999).
- For upstream synchronization, engine parity, and collaboration, visit the [official repository](https://github.com/horizon-rust/horizon).

---

## Core Characteristics

| Dimension | Horizon | Traditional JVM Wrapper |
| :--- | :--- | :--- |
| **Execution** | Native Rust (Ahead-of-Time) | OpenJDK HotSpot JVM |
| **Memory Baseline** | ~140–220 MB | 1,200–2,500 MB |
| **Cold Startup** | < 1.5 seconds | 15–35 seconds |
| **Graphics** | Vulkan 1.1 (Naga WGSL) | OpenGL via GL4ES |
| **Modding Sandbox** | WebAssembly (`.hmod` via Wasmtime) | Bytecode injection (Forge/Fabric unsupported) |
| **Garbage Collection** | Deterministic RAII (0ms GC pause) | Stop-the-world JVM GC cycles |

### Fast World Loading & Terrain Streaming
Chunk meshing and terrain generation routines run directly on native threads without intermediate object allocation, significantly reducing stuttering during chunk boundary crossings.

### Low Memory Footprint
By eliminating Java object headers and heap fragmentation, the client operates comfortably within a ~160 MB memory envelope on mobile and embedded hardware.

### Native Vulkan Pipeline
Custom WGSL shaders compiled with Naga communicate directly with Qualcomm Adreno and ARM Mali GPU drivers, bypassing OpenGL compatibility translation layers.

### WebAssembly Sandboxed Mods
Mod extensions compile to WebAssembly (`.hmod`) and run in a deterministic, memory-safe Wasmtime sandbox with strict capability boundaries.

---

## Supported Version Matrix

Horizon targets an exact engine specification verified against upstream test suites.

| Target | Architecture | Supported | Status |
| :--- | :--- | :--- | :--- |
| **Minecraft Java 26.3** | ARM64-v8a | Yes | Verified cryptographically against `research-lock.json` |
| **Minecraft Java 26.2** | Any | No | Unsupported |
| **Minecraft Java 1.20.x** | Any | No | Unsupported |
| **Forge / Fabric / NeoForge** | Any | No | Bytecode mods unsupported; use native `.hmod` |

---

## Legitimate Setup & Preparation

Horizon does not download or redistribute copyrighted Mojang assets. To build or run Horizon, you must provide your own legally purchased copy of Minecraft Java Edition 26.3.

1. **Verify Ownership**  
   Ensure you have an active Microsoft account with a legally purchased copy of Minecraft Java Edition 26.3.

2. **Prepare Inputs**  
   Run the prepare tool to extract official client assets and verify their SHA-1 signatures against `research-lock.json`:
   ```bash
   python3 tools/android_prepare.py --java-home "$JAVA_HOME"
   ```

3. **Install ARM64 Package**  
   Install the release package onto your ARM64 Android device:
   ```bash
   adb install -r dist/horizon-arm64.apk
   ```

---

<details>
<summary><strong>Versiunea în limba română (Romanian version)</strong></summary>

<br>

# Horizon

**Un client nativ Ahead-of-Time în Rust pentru Minecraft Java Edition 26.3.**

Horizon este o reimplementare independentă a motorului Minecraft Java Edition scrisă în Rust. Nu rulează bytecode Java printr-un emulator JVM sau Dalvik; fiecare subsistem este compilat direct în cod mașină nativ AArch64 și x86_64, cu randare directă în Vulkan.

### Notă privind caracterul neoficial

Horizon este un proiect comunitar independent. Nu este afiliat, aprobat sau asociat cu Mojang Studios, Microsoft Corporation sau oricare dintre filialele acestora. Minecraft este o marcă înregistrată a Mojang Synergies AB.

- **Ediția PC:** Creată și menținută upstream de [Ludovic Marie (horizon-rust)](https://github.com/horizon-rust/horizon).
- **Portarea Android:** Menținută și optimizată pentru dispozitive mobile ARM64 de [RedGlitchX (@tubers9999)](https://github.com/tubers9999).
- Pentru sincronizare upstream, paritate și colaborare, consultați [repository-ul oficial](https://github.com/horizon-rust/horizon).

### Caracteristici de bază

- **Execuție nativă în Rust:** Fără dependențe de mașina virtuală Java, eliminând ciclurile de pauză Garbage Collector.
- **Consum redus de memorie:** Rulează stabil într-o amprentă de ~140–220 MB RAM.
- **Randare prin Vulkan:** Pipeline direct WGSL/Naga optimizat pentru plăcile grafice mobile Qualcomm Adreno și ARM Mali.
- **Moduri sigure în WebAssembly (`.hmod`):** Extensii izolate într-un sandbox Wasmtime cu memorie deterministă. Modurile clasice Forge/Fabric nu sunt suportate.

### Compatibilitate

Doar **Minecraft Java Edition 26.3** este suportat și implementat conform specificațiilor din `research-lock.json`. Versiunile mai vechi și modurile Java nu sunt compatibile.

### Pregătire și cerință de deținere legală

Horizon nu redistribuie resurse originale Mojang. Pentru a compila și rula clientul, este necesară propria copie licențiată de Minecraft Java Edition 26.3, verificată cu scriptul `tools/android_prepare.py`.

</details>
