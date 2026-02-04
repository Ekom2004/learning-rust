# MX8 Video Industry Stories

> Real-world scenarios showing how the artifact layer saves compute

---

## Story 1: Netflix — The Transcoding Farm

### The Scene
Netflix has 15,000+ titles. Every title needs to be transcoded into **2,000+ versions**:
- Different resolutions (4K, 1080p, 720p, 480p)
- Different codecs (H.264, H.265, VP9, AV1)
- Different bitrates
- Different audio tracks

### Without MX8
```
Monday:
  New movie uploaded: "The Last Adventure" (2 hours, 4K)
  Transcode to 2,000 versions → 500 GPU-hours → $2,500
  
Tuesday:
  Director uploads "fixed" version (2 frames changed in credits)
  Re-transcode ALL 2,000 versions → 500 GPU-hours → $2,500
  
Wednesday:
  Audio mix updated (video untouched)
  Re-transcode ALL versions → 500 GPU-hours → $2,500
  
Week total: $7,500 for one movie
```

### With MX8
```
Monday:
  New movie uploaded
  Transcode to 2,000 versions → 500 GPU-hours → $2,500
  Each frame/version stored as artifact
  
Tuesday:
  Director uploads "fixed" version
  MX8 compares: "2 frames changed out of 172,800"
  Only transcode 2 frames × 2,000 versions = 0.5 GPU-hours → $2.50
  
Wednesday:
  Audio mix updated
  MX8 sees: "Video unchanged"
  Re-mux audio only → 5 minutes → $0.10
  
Week total: $2,502.60
```

### Savings
| | Without MX8 | With MX8 | Savings |
|---|-------------|----------|---------|
| Week | $7,500 | $2,503 | **66%** |
| Year (1000 titles) | $7.5M | $2.5M | **$5M saved** |

---

## Story 2: ILM (Industrial Light & Magic) — VFX Rendering

### The Scene
ILM is rendering a space battle scene for a Marvel movie:
- 500 frames (about 20 seconds)
- Each frame takes 4 hours to render
- Total: 2,000 GPU-hours per version

### Without MX8
```
Version 1:
  Director: "Render the scene"
  2,000 GPU-hours → $10,000 → 3 days on render farm
  
Version 2:
  Director: "Make the explosion bigger"
  One asset changed
  Re-render entire scene: 2,000 GPU-hours → $10,000 → 3 days
  
Version 3:
  Director: "Actually, make it 20% smaller"
  Re-render entire scene: 2,000 GPU-hours → $10,000 → 3 days
  
Version 4:
  Director: "Go back to version 2, but change the color"
  Re-render entire scene: 2,000 GPU-hours → $10,000 → 3 days
  
Total: $40,000, 12 days
```

### With MX8
```
Version 1:
  Render scene → 2,000 GPU-hours → $10,000
  Each layer/pass stored as artifact:
    - Background_pass: artifact_a7f3...
    - Ship_layer: artifact_b2e4...
    - Explosion_layer: artifact_c9d1...
    - Composite: artifact_final...
  
Version 2:
  Director: "Make the explosion bigger"
  MX8 checks: Background unchanged ✓, Ship unchanged ✓
  Only re-render: Explosion_layer + Composite
  100 GPU-hours → $500 → 6 hours
  
Version 3:
  Director: "Make it 20% smaller"
  Re-render: Explosion_layer + Composite only
  100 GPU-hours → $500 → 6 hours
  
Version 4:
  Director: "Go back to version 2, but change the color"
  MX8: "Version 2 explosion exists as artifact"
  Only re-render: Color grade + Composite
  20 GPU-hours → $100 → 1 hour
  
Total: $11,100, 3.5 days
```

### Savings
| | Without MX8 | With MX8 | Savings |
|---|-------------|----------|---------|
| This scene | $40,000 | $11,100 | **72%** |
| Typical movie (500 scenes) | $20M | $5.5M | **$14.5M saved** |

---

## Story 3: Runway — AI Video Generation

### The Scene
Runway is training their Gen-3 video generation model on 10 million video clips.
Each video needs:
- Frame extraction
- Embedding generation (CLIP, etc.)
- Caption generation
- Quality scoring

### Without MX8
```
Training Run 1:
  Extract frames from 10M videos → 100,000 GPU-hours → $200k
  Generate embeddings → 50,000 GPU-hours → $100k
  Generate captions → 20,000 GPU-hours → $40k
  Total preprocessing: $340k
  
Training Run 2 (next week):
  New model architecture, same data
  Full preprocessing again: $340k
  
Training Run 3 (two weeks later):
  Fine-tuning, same data
  Full preprocessing again: $340k
  
Monthly cost: $1M+ just in preprocessing
```

### With MX8
```
Training Run 1:
  Process 10M videos → $340k
  Every frame, embedding, caption = artifact
  Hash(video + model_version + settings) = artifact
  
Training Run 2:
  Same videos? Artifacts exist.
  Preprocessing: $0
  
Training Run 3:
  95% same videos, 5% new
  Preprocessing: $17k (5% of $340k)
  
Monthly cost: $357k (once), then ~$17k/month
```

### Savings
| | Without MX8 | With MX8 | Savings |
|---|-------------|----------|---------|
| 3 months | $1M | $391k | **61%** |
| Year | $4M | $544k | **$3.5M saved** |

---

## Story 4: YouTube — Processing 500 Hours/Minute

### The Scene
YouTube receives 500 hours of video every minute.
Each video needs:
- Transcoding (multiple resolutions)
- Thumbnail generation
- Content moderation (AI)
- Ad suitability scoring
- Auto-captions
- Content ID matching

### Without MX8
```
Every upload:
  - Transcode: compute
  - Thumbnails: compute
  - Moderation: compute
  - Everything: compute
  
Problem: Duplicate uploads
  - Same video uploaded by 100 different channels
  - Same processing × 100
  
Problem: Reuploads
  - Creator re-uploads with new title
  - Same video, full reprocessing
  
Annual waste: Billions of dollars
```

### With MX8
```
Video uploaded → Hash content (not filename)

Hash matches existing artifact?
  YES → All processing results exist
        - Transcode complete ✓
        - Thumbnails exist ✓
        - Moderation done ✓
        Just serve existing artifacts
  
  NO → Process and create artifacts

Result:
  - 100 duplicate uploads = 1 processing job
  - Reuploads = instant (same content hash)
  - Slightly modified = only process diff
```

### Savings
| Scenario | Without MX8 | With MX8 |
|----------|-------------|----------|
| Duplicate uploads (30%) | Full reprocess | Zero reprocess |
| Re-uploads (10%) | Full reprocess | Zero reprocess |
| Minor edits (15%) | Full reprocess | 5% reprocess |
| **Total compute saved** | - | **40-50%** |

At YouTube scale (estimated $2B/year in video processing):
**Savings: $800M - $1B/year**

---

## Story 5: ESPN — Live Sports + VOD

### The Scene
ESPN broadcasts live games, then serves VOD versions.
A single football game:
- Live: 4K HDR feed
- Near-live: Multiple delay feeds (different regions)
- Highlights: Auto-generated clips
- VOD: Full game + condensed version
- Social: Clips for Twitter, TikTok, Instagram

### Without MX8
```
Live broadcast:
  Original 4K → Live encoding → Done
  
Post-game (within 1 hour):
  Re-ingest same footage
  Re-transcode for VOD: 2 hours compute
  Re-analyze for highlights: 1 hour compute
  Re-cut social clips: 30 min compute
  
Next day:
  Re-cut for morning show: uses same footage
  Re-process: 30 min compute
  
Each game: 4+ hours of redundant compute
```

### With MX8
```
Live broadcast:
  Every frame → artifact (while encoding live)
  Hash(frame + timecode) = artifact
  
Post-game:
  VOD version? Frames are artifacts → instant assembly
  Highlights? Frames exist → just cut points
  Social clips? Same frames → instant
  
Next day:
  Morning show needs clip?
  Artifacts exist → instant delivery
  
Each game: Live encode only. Everything else: artifact lookup.
```

### Savings
| | Without MX8 | With MX8 | Savings |
|---|-------------|----------|---------|
| Per game | 4 hours compute | 30 min compute | **87%** |
| Per season (1000 games) | 4,000 hours | 500 hours | **$350k saved** |

---

## Summary: Why They'll Pay

| Customer | Current Annual Waste | MX8 Savings | They'll Pay |
|----------|---------------------|-------------|-------------|
| Netflix | ~$50M | $15-25M | $1-5M/year |
| ILM/VFX studios | ~$50M | $30-40M | $2-5M/year |
| Runway/Pika | ~$5M | $3M | $500k/year |
| YouTube | ~$2B | $800M-1B | $50-100M/year |
| ESPN/Broadcast | ~$10M | $5M | $500k-1M/year |

---

## The One-Liner for Video

> "Every frame computed once.  
> Every transcode remembered.  
> Every render an artifact.  
> Never reprocess what's already been done."

---

*Created: February 2026*
