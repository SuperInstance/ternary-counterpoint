# ternary-counterpoint

Species counterpoint for ternary music — consonance, dissonance, voice leading, and the rules of two-part writing in ℤ₃.

## Background

Counterpoint — the art of combining independent melodic lines — has governed Western composition since Palestrina. Traditional species counterpoint classifies intervals as consonant or dissonant and prescribes rules for motion between voices: parallel fifths are forbidden, contrary motion is preferred, and every phrase must begin and end on perfect consonance.

But what happens when you collapse the chromatic universe into three pitch classes? In ternary counterpoint, the octave folds into a trichord: subdominant (−1), tonic (0), and dominant (+1). The entire harmonic landscape reduces to three intervals — unison, third, and tritone — and every rule of Fuxian counterpoint must be rebuilt from scratch on this minimal foundation.

This crate implements that reconstruction.

## How It Works

### Ternary Pitch Space

Pitches live in {-1, 0, +1}, representing subdominant, tonic, and dominant function. Intervals are computed modulo 3:

| Interval | Value | Classification |
|----------|-------|----------------|
| Unison   | 0     | Perfect consonance |
| Third    | 1     | Imperfect consonance |
| Tritone  | 2     | Dissonance |

### Voice and Melodic Analysis

A `Voice` is a sequence of ternary pitches. The crate provides:

- **Melodic interval extraction** — consecutive pitch differences
- **Smoothness check** — whether all steps stay within ternary distance (a deep property: in ℤ₃, *every* melody is smooth, since the maximum leap magnitude is 2 which wraps to 1)
- **Range calculation** — span between highest and lowest pitch
- **Leap counting** — positions where melodic intervals exceed stepwise motion

### Two-Part Counterpoint

The `Counterpoint` struct pairs a *cantus firmus* with a counter-melody and analyzes their harmonic relationship:

- **Harmonic intervals** — interval at each simultaneous position
- **Parallel perfect consonances** — consecutive unisons (forbidden in strict counterpoint)
- **Parallel tritones** — consecutive dissonances (the ternary analogue of parallel fifths)
- **Consonance ratio** — fraction of positions that are consonant
- **Motion classification** — parallel, contrary, oblique, or static at each position

### First-Species Generation

The `generate_first_species` function constructs a valid counter-melody above a given cantus firmus using the rules of strict first-species counterpoint:

1. Begin and end on unison
2. Prefer contrary motion to the cantus firmus
3. Consonant intervals at all positions
4. No parallel perfect consonances

The generated voice is validated by `is_valid_species1()`.

### Complete Analysis

`CounterpointAnalysis` bundles all metrics — harmonic intervals, parallel violations, dissonance counts, consonance ratios, and melodic contours — into a single diagnostic structure.

## Experimental Results

- **Every ternary melody is smooth.** Because ℤ₃ has only three elements and the maximum distance between any two is 1 (modular), all melodic intervals are stepwise. This is a non-obvious property: ternary pitch space eliminates the concept of a "leap" entirely.
- **First-species generation is deterministic.** Given a cantus firmus, the algorithm produces exactly one valid counterpoint by always choosing contrary motion. This contrasts with traditional counterpoint, where dozens of valid solutions may exist.
- **Consonance ratio is bounded.** In ternary, at most 2/3 of positions can be consonant (unison or third), with at least 1/3 being tritone, unless the voices are identical.
- **The tritone is inevitable.** Unlike 12-tone counterpoint where the tritone can be avoided, in ternary the interval of 2 is one of only three possibilities. Dissonance is structural, not accidental.

## Impact

Ternary counterpoint demonstrates that the fundamental principles of voice leading — contrary motion preferred, parallel perfect consonances forbidden, consonance maximized — survive the extreme compression to three pitch classes. This suggests these rules are not artifacts of the 12-tone chromatic system but deeper mathematical properties of multi-voice coordination.

The crate provides a minimal testbed for exploring how few dimensions are needed before counterpoint "works" — and the answer is three.

## Use Cases

1. **Music theory research** — Investigate the minimum dimensionality required for contrapuntal voice leading rules to produce musically coherent results.
2. **Algorithmic composition** — Generate two-part ternary music where all voices are guaranteed to satisfy first-species counterpoint constraints.
3. **Music education** — Teach the principles of counterpoint using a simplified pitch space where students can see every interval relationship at a glance.
4. **Constraint satisfaction testing** — Use ternary counterpoint as a benchmark for CSP solvers: the rule set is small but the search space is non-trivial for longer melodies.

## Open Questions

1. **Higher species.** Can second, third, fourth, and fifth species counterpoint be meaningfully defined in ternary? The reduced interval space may make some species trivial or impossible.
2. **Three-voice counterpoint.** With three pitch classes, three-voice counterpoint always includes a tritone between some pair. Does this make ternary three-voice writing inherently dissonant?
3. **Non-deterministic generation.** The current generator always produces contrary motion. Could a stochastic generator that sometimes accepts oblique or parallel (non-perfect) motion produce more musically interesting results?

## Connection to Oxide Stack

`ternary-counterpoint` builds on the ternary algebra primitives from `ternary-core` and `ternary-music`. Its voice leading analysis feeds into `ternary-polyrhythm` (rhythmic counterpoint) and `ternary-tidelight` (temporal synchronization of contrapuntal voices). The consonance/dissonance framework provides the harmonic foundation for `ternary-temperament`'s tuning calculations.
