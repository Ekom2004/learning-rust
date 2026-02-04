# MX8 Data Loader Architecture

> Stream terabytes of training data with gigabytes of RAM

---

## High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                     Python API Layer                        │
│                  (PyO3 bindings)                           │
│   mx8.DataLoader("data/", batch_size=32, prefetch=4)       │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     Rust Core                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Index     │  │  Prefetch   │  │   Buffer    │        │
│  │   Manager   │  │   Engine    │  │   Pool      │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     I/O Layer                               │
│            Direct NVMe access (io_uring on Linux)          │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                     Storage                                 │
│                   NVMe SSD (1TB+)                          │
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. Python API Layer

**Purpose:** Simple interface for ML engineers

```python
from mx8 import DataLoader

# Create loader
loader = DataLoader(
    path="data/train/",      # Path to dataset
    batch_size=32,           # Samples per batch
    prefetch=4,              # Batches to prefetch
    shuffle=True,            # Random order
    num_workers=4,           # Parallel readers
)

# Use in training loop
for batch in loader:
    model.train(batch)
```

**Implementation:** PyO3 (Rust → Python bindings)

---

### 2. Index Manager

**Purpose:** Know what data exists and where

```rust
struct IndexManager {
    files: Vec<FileEntry>,      // All files in dataset
    total_samples: usize,       // Total count
    shuffle_indices: Vec<usize>,// Randomized order
}

struct FileEntry {
    path: PathBuf,
    offset: u64,      // Start position
    length: u64,      // Size in bytes
    num_samples: u32, // Samples in this file
}
```

**Responsibilities:**
- Scan dataset on init
- Track file locations
- Handle shuffling
- Map sample index → file + offset

---

### 3. Prefetch Engine

**Purpose:** Load next batches while GPU trains on current

```rust
struct PrefetchEngine {
    queue: VecDeque<Batch>,     // Prefetched batches ready to serve
    pending: Vec<JoinHandle>,    // In-flight loads
    prefetch_count: usize,       // How many to keep ready
}

impl PrefetchEngine {
    fn get_next_batch(&mut self) -> Batch {
        // 1. Return from queue (instant)
        // 2. Trigger load for next batch (async)
    }
    
    fn prefetch_worker(&self) {
        loop {
            // Load batches in background
            // Keep queue full
        }
    }
}
```

**Key insight:** While GPU processes batch N, we load batches N+1, N+2, N+3...

---

### 4. Buffer Pool

**Purpose:** Reuse memory, avoid allocations

```rust
struct BufferPool {
    buffers: Vec<Buffer>,       // Pre-allocated buffers
    available: VecDeque<usize>, // Free buffer indices
}

impl BufferPool {
    fn acquire(&mut self) -> &mut Buffer {
        // Get a free buffer (no allocation)
    }
    
    fn release(&mut self, buffer_id: usize) {
        // Return buffer to pool
    }
}
```

**Why:** Allocating memory is slow. Pre-allocate and reuse.

---

### 5. I/O Layer

**Purpose:** Fast reads from NVMe

```rust
struct IoEngine {
    // Linux: io_uring for async I/O
    // macOS: kqueue
    // Windows: IOCP
}

impl IoEngine {
    fn read_async(&self, file: &Path, offset: u64, length: u64) -> Future<Buffer> {
        // Submit read to OS
        // Returns immediately
        // Data arrives later
    }
}
```

**Key technologies:**
- **io_uring** (Linux) — fastest async I/O
- **mmap** — memory-mapped files
- **O_DIRECT** — bypass OS cache, direct to NVMe

---

## Data Flow

```
Step 1: Init
  └─→ Scan dataset → Build index

Step 2: Training starts
  └─→ Prefetch first N batches

Step 3: Training loop
  ┌─────────────────────────────────────────┐
  │  GPU trains on Batch 0                  │
  │         │                               │
  │         │ (in parallel)                 │
  │         ▼                               │
  │  Prefetch loads Batch 1, 2, 3, 4        │
  └─────────────────────────────────────────┘

Step 4: Next iteration
  ┌─────────────────────────────────────────┐
  │  Return Batch 1 (instant, already in RAM)│
  │  GPU trains on Batch 1                  │
  │         │                               │
  │         │ (in parallel)                 │
  │         ▼                               │
  │  Prefetch loads Batch 5                 │
  └─────────────────────────────────────────┘

Result: GPU never waits for data!
```

---

## Memory Budget

```
With prefetch=4 and batch_size=32:

Per sample: ~10 MB (e.g., 1024x1024 image)
Per batch:  32 × 10 MB = 320 MB
Prefetch:   4 × 320 MB = 1.28 GB

Total RAM needed: ~2 GB (for terabyte dataset!)
```

---

## File Formats Supported

| Format | Type | Implementation |
|--------|------|----------------|
| Raw binary | Images/tensors | Direct read |
| NumPy .npy | Arrays | Parse header + data |
| PNG/JPEG | Images | Decode on load |
| Video | Frames | Decode via ffmpeg |
| Custom | Any | User-defined decoder |

---

## Thread Model

```
┌─────────────────────┐
│   Main Thread       │
│   (Python)          │
│   - Calls next()    │
│   - Feeds GPU       │
└─────────────────────┘
          │
          │ request batch
          ▼
┌─────────────────────┐
│   Prefetch Thread   │
│   (Rust)            │
│   - Manages queue   │
│   - Coordinates I/O │
└─────────────────────┘
          │
          │ dispatch reads
          ▼
┌─────────────────────────────────────────┐
│   I/O Worker Pool (N threads)           │
│   - Read from NVMe                      │
│   - Decode if needed                    │
│   - Fill buffers                        │
└─────────────────────────────────────────┘
```

---

## Configuration

```python
loader = DataLoader(
    # Required
    path="data/",
    
    # Batching
    batch_size=32,
    drop_last=True,      # Drop incomplete final batch
    
    # Prefetching
    prefetch=4,          # Batches to keep ready
    num_workers=4,       # Parallel readers
    
    # Shuffling
    shuffle=True,
    seed=42,
    
    # Memory
    pin_memory=True,     # For faster GPU transfer
    
    # Advanced
    io_backend="uring",  # uring, mmap, sync
)
```

---

## Build Order

| Phase | What to Build | Complexity |
|-------|---------------|------------|
| 1 | Single-file sync reader | Low |
| 2 | Multi-file index | Low |
| 3 | Basic prefetching (1 thread) | Medium |
| 4 | Buffer pool | Medium |
| 5 | Multi-threaded I/O | Medium |
| 6 | io_uring integration | High |
| 7 | Python bindings (PyO3) | Medium |
| 8 | Shuffle + sampling | Low |

---

## MVP (Minimum Viable Product)

For first release, just need:
- ✅ Scan folder of files
- ✅ Load batches sequentially
- ✅ Basic prefetching (2-4 batches)
- ✅ Python bindings
- ✅ Works with PyTorch

Skip for MVP:
- ❌ io_uring (use simple file reads)
- ❌ Complex shuffling
- ❌ Video support
- ❌ Distributed training

---

## Success Metrics

| Metric | Target |
|--------|--------|
| GPU utilization | 95%+ (vs 50-70% typical) |
| RAM usage | 10x less than dataset size |
| Throughput | Saturate NVMe (~5 GB/s) |
| Latency | < 1ms per batch (from cache) |

---

*Created: February 2026*
