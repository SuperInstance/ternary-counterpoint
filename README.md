# Ternary Counterpoint — Species Counterpoint in Z₃ Ternary Music Theory

**Ternary Counterpoint** implements the rules of species counterpoint — the classical study of melodic writing and voice leading — adapted to a ternary pitch system where only three pitches exist: subdominant (-1), tonic (0), and dominant (+1). All intervals are computed modulo 3, making this counterpoint in the group Z₃. The crate classifies consonance/dissonance, analyzes voice leading smoothness, and enforces the rules of two-part writing.

## Why It Matters

Species counterpoint has been the pedagogical foundation of Western music theory since Fux's *Gradus ad Parnassum* (1725). By reducing the pitch space to three values (Z₃), this crate makes counterpoint fully computable: the state space is small enough to exhaustively analyze every possible two-part combination. This is not just a musical exercise — Z₃ counterpoint is isomorphic to the cyclic dominance dynamics (rock-paper-scissors) that govern ternary agent ecosystems. The voice-leading rules (smooth motion preferred, avoid parallel dissonances) translate directly to coordination rules for distributed agents.

## How It Works

### Ternary Intervals

The interval between two pitches is `(b - a) mod 3`, yielding three classifications:

- **Interval 0 (Unison)**: Perfect consonance — both voices on the same pitch
- **Interval 1 (Third)**: Imperfect consonance — one step apart in Z₃
- **Interval 2 (Tritone)**: Dissonance — the maximum distance in Z₃

### Voice Leading

A `Voice` is a sequence of pitches. Voice leading quality is measured by:

- **Smoothness**: All melodic intervals are ≤ 1 (stepwise motion in Z₃)
- **Leap count**: Number of intervals > 1 (a "leap" in Z₃ is interval 2, which wraps)
- **Range**: max - min across the voice's pitches

### Counterpoint Rules

The rules of ternary species counterpoint enforce:

1. **Consonance on strong beats**: Downbeat intervals must be Perfect or Imperfect consonance
2. **No parallel dissonances**: Two consecutive tritones between the same voices are forbidden
3. **Smooth voice leading**: Limit leaps; prefer stepwise (interval ≤ 1) motion
4. **Independent voices**: Voices should not move in parallel consistently

These rules are checked in O(n) for n note pairs.

## Quick Start

```rust
use ternary_counterpoint::{Voice, classify_interval, Consonance};

let cantus = Voice::new(vec![0, 1, -1, 0]); // tonic → dominant → subdominant → tonic
let counterpoint = Voice::new(vec![1, 0, 0, 1]); // dominant → tonic → tonic → dominant

// Check interval at first beat
let c = classify_interval(cantus.notes[0], counterpoint.notes[0]);
assert_eq!(c, Consonance::Imperfect); // interval 1 = third

// Voice leading
let intervals = cantus.melodic_intervals();
println!("Cantus has {} leaps", cantus.leap_count());
```

```bash
cargo add ternary-counterpoint
```

## API

| Type / Function | Description |
|---|---|
| `Pitch` | Alias for `i8` in {-1, 0, +1} |
| `interval(a, b)` | `(b - a) mod 3` → {0=unison, 1=third, 2=tritone} |
| `classify_interval(a, b)` | Returns `Perfect`, `Imperfect`, or `Dissonant` |
| `Voice` | Pitch sequence with `melodic_intervals()`, `is_smooth()`, `leap_count()`, `range()` |
| `Consonance` | Enum: Perfect, Imperfect, Dissonant |

## Architecture Notes

The Z₃ cyclic structure of ternary counterpoint is the same algebra that governs **SuperInstance** agent coordination: the three agent states {-1, 0, +1} rotate in Z₃ dominance cycles, and the voice-leading rules for smooth transitions translate directly to rules for graceful agent state changes. The γ + η = C conservation maps to the consonance constraint: total harmonic content is bounded. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Fux, Johann Joseph. *Gradus ad Parnassum* (1725), trans. Mann, 1965 — species counterpoint.
- Tymoczko, Dmitri. *A Geometry of Music*, Oxford UP, 2011 — mathematical music theory.
- Mazzola, Guerino. *The Topos of Music*, Birkhäuser, 2002 — algebraic music theory.

## License

MIT
