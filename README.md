# Proof timing on main

Written by CI after every push to `main` (`scripts/proof-timing-history.py` on `main`), from release builds on the phones of `scripts/devicefarm/devices.json` in AWS Device Farm. Two runs of one commit differ by up to 5 % on the Galaxy A16 and A56 and up to 15 % on the S25 and the iPhones. The data is in `history/`, a line per commit.

## Samsung Galaxy A16, Android 14, low tier

**Before:** [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) of 2026-11-03. **After:** [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) of 2026-11-05.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 5.32 s (download: 2.04 s, 6.5 MiB) | 9.94 s (download: 4.89 s, 11.6 MiB) | 15.94 s (download: 6.99 s, 21.6 MiB) | 35.93 s (download: 19.32 s, 41.9 MiB) | 70.00 s (download: 38.44 s, 84.2 MiB) | 115.20 s (download: 63.58 s, 138.6 MiB) | 110.89 s (download: 59.95 s, 133.1 MiB) |
| | after | 5.39 s (download: 2.19 s, 6.5 MiB) (+1 %) | 10.35 s (download: 5.10 s, 11.6 MiB) (+4 %) | 18.30 s (download: 9.61 s, 21.6 MiB) (+15 %) | 36.47 s (download: 19.29 s, 41.9 MiB) (+2 %) | 70.57 s (download: 38.62 s, 84.2 MiB) (+1 %) | 112.94 s (download: 62.08 s, 138.6 MiB) (-2 %) | 109.03 s (download: 60.22 s, 133.1 MiB) (-2 %) |
| warm | before | 0.83 s | 1.48 s | 2.41 s | 4.53 s | 8.33 s | 12.91 s | 12.11 s |
| | after | 0.86 s | 1.40 s (-5 %) | 2.42 s | 4.42 s (-2 %) | 7.71 s (-7 %) | 13.07 s (+1 %) | 12.16 s |
| first run after restart | before | 1.05 s | 1.72 s | 2.76 s | 4.90 s | 8.53 s | 13.70 s | 13.29 s |
| | after | 0.98 s (-7 %) | 1.71 s | 2.75 s | 4.92 s | 8.62 s (+1 %) | 13.42 s (-2 %) | 13.28 s |
| repeated after restart | before | 0.86 s | 1.41 s | 2.43 s | 4.49 s | 8.42 s | 12.71 s | 12.55 s |
| | after | 0.86 s | 1.42 s | 2.39 s | 4.60 s (+2 %) | 7.88 s (-6 %) | 12.60 s (-1 %) | 12.21 s (-3 %) |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.21 s | 0.27 s | 0.32 s | 0.39 s | 0.65 s | 0.95 s | 0.68 s |
| | after | 0.21 s | 0.26 s | 0.30 s | 0.36 s | 0.61 s | 0.76 s (-20 % **!**) | 0.77 s (+13 %) |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.01 s | 0.01 s |
| prove | before | 0.82 s | 1.40 s | 2.40 s | 4.47 s | 8.08 s | 12.71 s | 12.55 s |
| | after | 0.83 s | 1.39 s | 2.38 s | 4.54 s (+1 %) | 7.86 s (-3 %) | 12.65 s | 12.30 s (-2 %) |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.83 s | 1.39 s | 2.38 s | 4.54 s | 7.86 s | 12.65 s | 12.30 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.82 s | 1.40 s | 2.40 s | 4.47 s | 8.08 s | 12.71 s | 12.55 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.85 s | 1.42 s | 2.42 s | 4.45 s | 8.06 s | 12.63 s | 12.34 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.83 s | 1.42 s | 2.40 s | 4.40 s | 8.09 s | 12.57 s | 12.46 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.86 s | 1.41 s | 2.46 s | 4.48 s | 8.08 s | 12.76 s | 12.41 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.82 s | 1.37 s | 2.47 s | 4.39 s | 7.92 s | 12.89 s | 12.27 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.81 s | 1.43 s | 2.44 s | 4.46 s | 8.13 s | 12.89 s | 12.25 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.82 s | 1.38 s | 2.42 s | 4.50 s | 8.04 s | 12.57 s | 12.25 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.80 s | 1.34 s | 2.43 s | 4.52 s | 8.00 s | 12.50 s | 12.35 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.81 s | 1.42 s | 2.41 s | 4.50 s | 7.90 s | 12.41 s | 12.13 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.80 s | 1.40 s | 2.41 s | 4.57 s | 7.98 s | 12.68 s | 12.13 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.87 s | 1.36 s | 2.49 s | 4.44 s | 8.09 s | 12.65 s | 12.58 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.82 s | 1.39 s | 2.43 s | 4.54 s | 8.28 s | 12.53 s | 12.17 s |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.82 s | 1.37 s | 2.39 s | 4.45 s | 8.02 s | 12.67 s | 12.15 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.82 s | 1.35 s | 2.43 s | 4.44 s | 7.80 s | 12.77 s | 12.37 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.82 s | 1.37 s | 2.45 s | 4.52 s | 8.08 s | 12.52 s | 12.33 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.81 s | 1.39 s | 2.44 s | 4.47 s | 7.93 s | 12.54 s | 12.13 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.82 s | 1.42 s | 2.45 s | 4.44 s | 8.18 s | 12.74 s | 12.50 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.84 s | 1.39 s | 2.45 s | 4.39 s | 7.88 s | 12.61 s | 12.27 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.81 s | 1.44 s | 2.42 s | 4.43 s | 8.08 s | 12.51 s | 12.10 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.98 s | 1.71 s | 2.75 s | 4.92 s | 8.62 s | 13.42 s | 13.28 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 1.05 s | 1.72 s | 2.76 s | 4.90 s | 8.53 s | 13.70 s | 13.29 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.98 s | 1.68 s | 2.71 s | 4.87 s | 8.35 s | 13.76 s | 12.82 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.98 s | 1.60 s | 2.72 s | 4.75 s | 8.86 s | 13.56 s | 13.22 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 1.04 s | 1.65 s | 2.78 s | 5.10 s | 8.88 s | 13.76 s | 13.22 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.98 s | 1.68 s | 2.72 s | 4.80 s | 8.73 s | 13.97 s | 13.12 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.98 s | 1.66 s | 2.76 s | 4.83 s | 9.00 s | 13.60 s | 12.98 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 1.02 s | 1.71 s | 2.77 s | 5.08 s | 8.57 s | 13.20 s | 13.03 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 1.00 s | 1.70 s | 2.81 s | 5.03 s | 8.54 s | 13.29 s | 13.33 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.96 s | 1.63 s | 2.83 s | 4.84 s | 8.66 s | 13.54 s | 12.81 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.98 s | 1.66 s | 2.79 s | 5.04 s | 8.56 s | 13.44 s | 13.10 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 1.02 s | 1.66 s | 2.95 s | 4.99 s | 8.74 s | 13.57 s | 13.41 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 1.05 s | 1.69 s | 2.75 s | 4.75 s | 9.16 s | 13.53 s | 12.73 s |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.99 s | 1.78 s | 2.70 s | 4.88 s | 8.66 s | 13.75 s | 12.99 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.99 s | 1.66 s | 2.72 s | 5.04 s | 8.99 s | 13.81 s | 13.41 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.99 s | 1.68 s | 2.71 s | 4.77 s | 8.64 s | 13.24 s | 13.23 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 1.03 s | 1.64 s | 2.77 s | 5.01 s | 8.53 s | 13.61 s | 12.86 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.99 s | 1.73 s | 2.99 s | 4.86 s | 8.59 s | 13.48 s | 13.41 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 1.00 s | 1.72 s | 2.86 s | 4.77 s | 8.54 s | 13.81 s | 12.91 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.98 s | 1.76 s | 2.75 s | 4.87 s | 8.54 s | 13.70 s | 13.22 s |

## Samsung Galaxy A56, Android 15, mid tier

**Before:** [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) of 2026-11-03. **After:** [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) of 2026-11-05.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 62.60 s (download: 61.51 s, 6.5 MiB) | 20.20 s (download: 18.08 s, 11.6 MiB) | 58.88 s (download: 55.77 s, 21.6 MiB) | 86.67 s (download: 80.84 s, 41.9 MiB) | 70.26 s (download: 58.63 s, 84.2 MiB) | 296.74 s (download: 278.13 s, 138.6 MiB) | 338.60 s (download: 321.07 s, 133.1 MiB) |
| | after | 2.43 s (download: 1.36 s, 6.5 MiB) (-96 % **!**) | 3.50 s (download: 1.76 s, 11.6 MiB) (-83 % **!**) | 6.34 s (download: 3.30 s, 21.6 MiB) (-89 % **!**) | 12.70 s (download: 6.92 s, 41.9 MiB) (-85 % **!**) | 23.76 s (download: 12.40 s, 84.2 MiB) (-66 % **!**) | 38.94 s (download: 20.89 s, 138.6 MiB) (-87 % **!**) | 38.83 s (download: 21.38 s, 133.1 MiB) (-89 % **!**) |
| warm | before | 0.27 s | 0.54 s | 0.80 s | 1.42 s | 2.79 s | 4.41 s | 4.30 s |
| | after | 0.25 s | 0.43 s (-21 % **!**) | 0.75 s | 1.41 s | 2.71 s (-3 %) | 4.34 s (-2 %) | 4.28 s |
| first run after restart | before | 0.53 s | 0.59 s | 0.96 s | 1.66 s | 3.07 s | 4.82 s | 4.82 s |
| | after | 0.52 s | 0.57 s | 0.96 s | 1.70 s | 3.05 s | 4.83 s | 4.82 s |
| repeated after restart | before | 0.26 s | 0.43 s | 0.77 s | 1.40 s | 2.71 s | 4.39 s | 4.29 s |
| | after | 0.26 s | 0.44 s | 0.77 s | 1.40 s | 2.71 s | 4.43 s | 4.34 s |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.24 s | 0.13 s | 0.15 s | 0.22 s | 0.25 s | 0.38 s | 0.42 s |
| | after | 0.21 s | 0.11 s | 0.15 s | 0.22 s | 0.28 s | 0.35 s | 0.42 s |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.27 s | 0.46 s | 0.79 s | 1.41 s | 2.80 s | 4.42 s | 4.32 s |
| | after | 0.25 s | 0.44 s | 0.77 s | 1.41 s | 2.73 s (-2 %) | 4.43 s | 4.31 s |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.25 s | 0.44 s | 0.77 s | 1.41 s | 2.73 s | 4.43 s | 4.31 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.27 s | 0.46 s | 0.79 s | 1.41 s | 2.80 s | 4.42 s | 4.32 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.26 s | 0.43 s | 0.77 s | 1.38 s | 2.74 s | 4.37 s | 4.25 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.25 s | 0.44 s | 0.76 s | 1.39 s | 2.71 s | 4.37 s | 4.30 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.26 s | 0.43 s | 0.76 s | 1.40 s | 2.69 s | 4.39 s | 4.30 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.26 s | 0.43 s | 0.76 s | 1.41 s | 2.72 s | 4.41 s | 4.28 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.26 s | 0.43 s | 0.75 s | 1.39 s | 2.72 s | 4.40 s | 4.32 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.26 s | 0.43 s | 0.77 s | 1.41 s | 2.74 s | 4.43 s | 4.28 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.26 s | 0.43 s | 0.77 s | 1.41 s | 2.75 s | 4.38 s | 4.40 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.26 s | 0.43 s | 0.81 s | 1.40 s | 2.75 s | 4.37 s | 4.26 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.26 s | 0.43 s | 0.76 s | 1.40 s | 2.75 s | 4.36 s | 4.28 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.26 s | 0.43 s | 0.76 s | 1.41 s | 2.77 s | 4.40 s | 4.31 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.25 s | 0.43 s | 0.76 s | 1.40 s | 2.76 s | 4.38 s | 4.24 s |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.26 s | 0.42 s | 0.75 s | 1.41 s | 2.74 s | 4.36 s | 4.27 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.26 s | 0.44 s | 0.76 s | 1.41 s | 2.79 s | 4.37 s | 4.29 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.27 s | 0.43 s | 0.77 s | 1.40 s | 2.74 s | 4.36 s | 4.28 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.26 s | 0.43 s | 0.76 s | 1.40 s | 2.75 s | 4.41 s | 4.27 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.26 s | 0.43 s | 0.76 s | 1.39 s | 2.71 s | 4.41 s | 4.26 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.26 s | 0.43 s | 0.76 s | 1.40 s | 2.75 s | 4.40 s | 4.29 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.25 s | 0.43 s | 0.77 s | 1.40 s | 2.75 s | 4.39 s | 4.28 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.52 s | 0.57 s | 0.96 s | 1.70 s | 3.05 s | 4.83 s | 4.82 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.53 s | 0.59 s | 0.96 s | 1.66 s | 3.07 s | 4.82 s | 4.82 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.59 s | 0.59 s | 0.92 s | 1.58 s | 3.01 s | 4.72 s | 4.60 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.54 s | 0.62 s | 1.01 s | 1.62 s | 3.01 s | 4.79 s | 4.71 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.51 s | 0.58 s | 0.92 s | 1.63 s | 2.94 s | 4.88 s | 4.81 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.64 s | 0.58 s | 0.95 s | 1.61 s | 3.03 s | 4.78 s | 4.66 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.51 s | 0.59 s | 0.96 s | 1.62 s | 3.06 s | 4.87 s | 4.75 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.52 s | 0.59 s | 0.95 s | 1.62 s | 3.06 s | 4.80 s | 4.78 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.52 s | 0.60 s | 0.96 s | 1.63 s | 3.11 s | 4.74 s | 4.74 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.53 s | 0.61 s | 0.95 s | 1.65 s | 3.13 s | 4.93 s | 4.84 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.51 s | 0.58 s | 0.96 s | 1.63 s | 3.34 s | 4.78 s | 4.66 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.55 s | 0.61 s | 0.99 s | 1.62 s | 3.03 s | 4.75 s | 4.68 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.54 s | 0.59 s | 0.95 s | 1.66 s | 3.10 s | 4.76 s | 4.60 s |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.53 s | 0.55 s | 0.92 s | 1.61 s | 3.09 s | 4.73 s | 4.67 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.52 s | 0.57 s | 0.94 s | 1.67 s | 3.24 s | 4.71 s | 4.63 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.52 s | 0.60 s | 1.01 s | 1.60 s | 3.10 s | 4.82 s | 4.68 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.53 s | 0.59 s | 0.94 s | 1.62 s | 3.19 s | 4.87 s | 4.79 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.51 s | 0.56 s | 0.95 s | 1.63 s | 3.08 s | 4.88 s | 4.76 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.52 s | 0.60 s | 0.95 s | 1.65 s | 3.17 s | 4.75 s | 4.71 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.55 s | 0.58 s | 0.93 s | 1.69 s | 3.17 s | 4.91 s | 4.86 s |

## Samsung Galaxy S25, Android 15, high tier

**Before:** [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) of 2026-11-03. **After:** [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) of 2026-11-05.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 15.02 s (download: 1.97 s, 6.5 MiB) | 4.65 s (download: 3.43 s, 11.6 MiB) | 7.49 s (download: 5.34 s, 21.6 MiB) | 13.07 s (download: 10.13 s, 41.9 MiB) | 24.59 s (download: 19.04 s, 84.2 MiB) | 40.26 s (download: 31.50 s, 138.6 MiB) | 35.44 s (download: 26.94 s, 133.1 MiB) |
| | after | 14.79 s (download: 1.79 s, 6.5 MiB) (-1 %) | 3.87 s (download: 2.46 s, 11.6 MiB) (-17 %) | 4.47 s (download: 2.37 s, 21.6 MiB) (-40 % **!**) | 8.18 s (download: 5.25 s, 41.9 MiB) (-37 % **!**) | 15.35 s (download: 9.85 s, 84.2 MiB) (-38 % **!**) | 23.74 s (download: 15.01 s, 138.6 MiB) (-41 % **!**) | 23.09 s (download: 14.48 s, 133.1 MiB) (-35 % **!**) |
| warm | before | 0.15 s | 0.21 s | 0.35 s | 0.54 s | 0.92 s | 1.54 s | 1.49 s |
| | after | 0.15 s | 0.21 s | 0.34 s | 0.53 s | 0.91 s | 1.53 s | 1.58 s (+6 %) |
| first run after restart | before | 0.47 s | 0.51 s | 0.66 s | 0.97 s | 1.58 s | 3.44 s | 4.00 s |
| | after | 0.56 s (+20 %) | 0.62 s (+22 % **!**) | 0.87 s (+32 % **!**) | 1.07 s (+11 %) | 1.94 s (+23 % **!**) | 4.60 s (+34 % **!**) | 4.94 s (+24 % **!**) |
| repeated after restart | before | 0.14 s | 0.20 s | 0.31 s | 0.50 s | 0.91 s | 3.33 s | 1.93 s |
| | after | 0.14 s | 0.20 s | 0.31 s | 0.48 s | 0.85 s (-6 %) | 3.29 s | 3.29 s (+71 % **!**) |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.32 s | 0.30 s | 0.36 s | 0.47 s | 0.69 s | 1.01 s | 1.90 s |
| | after | 0.41 s (+29 % **!**) | 0.42 s (+37 % **!**) | 0.51 s (+40 % **!**) | 0.58 s (+22 % **!**) | 0.88 s (+27 % **!**) | 1.27 s (+26 % **!**) | 1.66 s (-13 %) |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.14 s | 0.19 s | 0.32 s | 0.51 s | 0.90 s | 1.98 s | 1.70 s |
| | after | 0.15 s | 0.20 s | 0.33 s | 0.50 s | 0.92 s | 2.41 s (+21 % **!**) | 2.43 s (+43 % **!**) |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.15 s | 0.20 s | 0.33 s | 0.50 s | 0.92 s | 2.41 s | 2.43 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.14 s | 0.19 s | 0.32 s | 0.51 s | 0.90 s | 1.98 s | 1.70 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.15 s | 0.19 s | 0.30 s | 0.53 s | 0.93 s | 2.17 s | 1.85 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.15 s | 0.23 s | 0.34 s | 0.53 s | 0.89 s | 1.47 s | 1.48 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.15 s | 0.21 s | 0.33 s | 0.49 s | 0.89 s | 1.89 s | 1.89 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.15 s | 0.19 s | 0.30 s | 0.48 s | 0.85 s | 1.40 s | 1.36 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.14 s | 0.19 s | 0.31 s | 0.48 s | 0.89 s | 1.89 s | 2.15 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.15 s | 0.19 s | 0.30 s | 0.57 s | 1.05 s | 2.14 s | 2.15 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.15 s | 0.20 s | 0.60 s | 0.69 s | 1.33 s | 2.22 s | 2.31 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.15 s | 0.20 s | 0.35 s | 0.59 s | 1.04 s | 2.05 s | 2.00 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.14 s | 0.19 s | 0.30 s | 0.59 s | 1.01 s | 1.68 s | 1.66 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.14 s | 0.19 s | 0.32 s | 0.48 s | 0.88 s | 1.89 s | 1.82 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.14 s | 0.19 s | 0.30 s | 0.48 s | 0.85 s | 1.39 s | 1.36 s |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.15 s | 0.19 s | 0.33 s | 0.52 s | 0.87 s | 1.78 s | 1.75 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.15 s | 0.19 s | 0.30 s | 0.49 s | 0.86 s | 1.40 s | 1.37 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.15 s | 0.20 s | 0.32 s | 0.63 s | 1.13 s | 1.94 s | 2.04 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.15 s | 0.19 s | 0.30 s | 0.51 s | 0.89 s | 1.46 s | 1.42 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.14 s | 0.20 s | 0.31 s | 0.48 s | 0.85 s | 1.40 s | 1.36 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.14 s | 0.19 s | 0.30 s | 0.48 s | 0.85 s | 1.40 s | 1.36 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.14 s | 0.19 s | 0.30 s | 0.48 s | 0.86 s | 1.71 s | 2.12 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.56 s | 0.62 s | 0.87 s | 1.07 s | 1.94 s | 4.60 s | 4.94 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.47 s | 0.51 s | 0.66 s | 0.97 s | 1.58 s | 3.44 s | 4.00 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.50 s | 0.51 s | 0.74 s | 1.04 s | 1.72 s | 3.98 s | 4.36 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.50 s | 0.57 s | 0.76 s | 1.06 s | 1.69 s | 2.58 s |  |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.56 s | 0.66 s | 0.91 s | 1.23 s | 1.77 s | 3.66 s | 3.50 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.54 s | 0.52 s | 0.72 s | 0.96 s | 1.52 s | 2.33 s |  |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.50 s | 0.50 s | 0.67 s | 1.02 s | 1.67 s | 3.34 s | 4.61 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.49 s | 0.53 s | 0.63 s | 1.17 s | 1.93 s | 4.49 s | 4.72 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.63 s | 0.69 s | 0.85 s | 1.64 s | 2.62 s | 4.14 s | 3.89 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.55 s | 0.66 s | 0.94 s | 1.94 s | 2.20 s | 4.34 s | 4.30 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.50 s | 0.53 s | 0.66 s | 1.14 s | 1.84 s | 2.86 s | 2.78 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.48 s | 0.53 s | 0.70 s | 1.08 s | 1.75 s | 3.48 s | 3.37 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.47 s | 0.49 s | 0.65 s | 0.94 s | 1.51 s |  |  |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.57 s | 0.64 s | 0.80 s | 1.05 s | 1.73 s | 3.25 s | 3.19 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.49 s | 0.50 s | 0.65 s | 1.00 s | 1.65 s |  |  |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.64 s | 0.67 s | 0.94 s | 1.45 s | 2.24 s | 3.69 s | 3.51 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.51 s | 0.54 s | 0.66 s | 1.00 s | 1.62 s | 2.48 s |  |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.51 s | 0.55 s | 0.72 s | 1.02 s | 1.68 s | 2.55 s |  |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.51 s | 0.54 s | 0.70 s | 0.96 s | 1.52 s | 2.33 s |  |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.51 s | 0.50 s | 0.65 s | 0.97 s | 1.59 s | 2.94 s | 4.60 s |

## iPhone 12, iOS 18.5, low tier

**Before:** [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) of 2026-11-03. **After:** [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) of 2026-11-05.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 2.93 s (download: 0.48 s, 6.5 MiB) | 2.02 s (download: 0.54 s, 11.6 MiB) | 3.60 s (download: 0.94 s, 21.6 MiB) | 6.70 s (download: 1.91 s, 41.9 MiB) | 13.29 s (download: 3.61 s, 84.2 MiB) | 21.45 s (download: 6.06 s, 138.6 MiB) | 20.26 s (download: 5.78 s, 133.1 MiB) |
| | after | 4.00 s (download: 1.29 s, 6.5 MiB) (+37 % **!**) | 3.02 s (download: 1.53 s, 11.6 MiB) (+50 % **!**) | 5.43 s (download: 2.69 s, 21.6 MiB) (+51 % **!**) | 10.71 s (download: 5.58 s, 41.9 MiB) (+60 % **!**) | 21.49 s (download: 11.47 s, 84.2 MiB) (+62 % **!**) | 34.97 s (download: 18.45 s, 138.6 MiB) (+63 % **!**) | 22.36 s (download: 5.78 s, 133.1 MiB) (+10 %) |
| warm | before | 0.14 s | 0.23 s | 0.42 s | 0.67 s | 1.20 s | 1.99 s | 1.99 s |
| | after | 0.14 s | 0.23 s | 0.41 s | 0.68 s | 1.31 s (+9 %) | 2.19 s (+10 %) | 2.28 s (+15 %) |
| first run after restart | before | 0.30 s | 0.37 s | 0.67 s | 1.07 s | 1.86 s | 3.02 s | 2.92 s |
| | after | 0.31 s | 0.45 s (+21 % **!**) | 0.75 s (+11 %) | 1.20 s (+12 %) | 2.11 s (+13 %) | 3.37 s (+12 %) | 3.69 s (+26 % **!**) |
| repeated after restart | before | 0.15 s | 0.23 s | 0.43 s | 0.74 s | 1.35 s | 2.20 s | 2.15 s |
| | after | 0.16 s | 0.26 s | 0.49 s (+14 %) | 0.84 s (+14 %) | 1.54 s (+15 %) | 2.60 s (+19 %) | 2.80 s (+30 % **!**) |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.12 s | 0.14 s | 0.22 s | 0.35 s | 0.58 s | 0.88 s | 0.85 s |
| | after | 0.11 s | 0.17 s | 0.25 s | 0.37 s | 0.62 s | 0.95 s (+7 %) | 1.12 s (+31 % **!**) |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.15 s | 0.23 s | 0.43 s | 0.71 s | 1.25 s | 2.06 s | 2.02 s |
| | after | 0.15 s | 0.26 s | 0.47 s | 0.76 s | 1.39 s (+12 %) | 2.30 s (+12 %) | 2.42 s (+20 %) |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.15 s | 0.26 s | 0.47 s | 0.76 s | 1.39 s | 2.30 s | 2.42 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.15 s | 0.23 s | 0.43 s | 0.71 s | 1.25 s | 2.06 s | 2.02 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.14 s | 0.23 s | 0.43 s | 0.72 s | 1.26 s | 2.07 s | 2.05 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.15 s | 0.24 s | 0.45 s | 0.73 s | 1.30 s | 2.16 s | 2.15 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.14 s | 0.23 s | 0.43 s | 0.71 s | 1.26 s | 2.06 s | 2.08 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.20 s | 0.31 s | 0.52 s | 0.80 s | 1.46 s | 2.43 s | 2.53 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.15 s | 0.23 s | 0.43 s | 0.72 s | 1.25 s | 2.08 s | 2.06 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.15 s | 0.23 s | 0.44 s | 0.72 s | 1.27 s | 2.14 s | 2.12 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.15 s | 0.24 s | 0.45 s | 0.74 s | 1.31 s | 2.19 s | 2.29 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.15 s | 0.24 s | 0.46 s | 0.75 s | 1.33 s | 2.24 s | 2.26 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.14 s | 0.23 s | 0.42 s | 0.67 s |  |  |  |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.15 s | 0.24 s | 0.46 s | 0.75 s | 1.34 s | 2.22 s | 2.18 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.15 s | 0.23 s |  |  |  |  |  |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.14 s | 0.23 s | 0.43 s | 0.71 s | 1.25 s | 2.03 s | 2.02 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.15 s | 0.23 s | 0.43 s | 0.72 s | 1.25 s | 2.04 s | 2.03 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.14 s | 0.23 s | 0.45 s | 0.74 s | 1.29 s | 2.17 s | 2.14 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.14 s | 0.23 s | 0.46 s | 0.73 s | 1.30 s | 2.20 s | 2.32 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.14 s | 0.24 s | 0.45 s | 0.73 s | 1.30 s | 2.18 s | 2.31 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.15 s | 0.24 s | 0.45 s | 0.75 s | 1.33 s | 2.21 s | 2.18 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.15 s | 0.23 s | 0.44 s | 0.72 s | 1.26 s | 2.08 s | 2.07 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.31 s | 0.45 s | 0.75 s | 1.20 s | 2.11 s | 3.37 s | 3.69 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.30 s | 0.37 s | 0.67 s | 1.07 s | 1.86 s | 3.02 s | 2.92 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.29 s | 0.38 s | 0.67 s | 1.08 s | 1.89 s | 3.06 s | 2.95 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.29 s | 0.42 s | 0.69 s | 1.12 s | 1.98 s | 3.17 s | 3.06 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.30 s | 0.38 s | 0.68 s | 1.11 s | 1.88 s | 3.06 s | 2.99 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.35 s | 0.58 s | 0.88 s | 1.30 s | 2.29 s | 3.48 s | 3.49 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.30 s | 0.36 s | 0.67 s | 1.07 s | 1.92 s | 3.04 s | 2.92 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.30 s | 0.36 s | 0.67 s | 1.04 s | 1.81 s | 2.97 s | 2.86 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.30 s | 0.38 s | 0.69 s | 1.13 s | 2.00 s | 3.17 s | 3.33 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.29 s | 0.37 s | 0.67 s | 1.10 s | 1.92 s | 3.08 s | 3.05 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 |  |  |  |  |  |  |  |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.29 s | 0.36 s | 0.66 s | 1.08 s | 1.86 s | 3.05 s | 2.90 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 |  |  |  |  |  |  |  |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.30 s | 0.36 s | 0.66 s | 1.06 s | 1.85 s | 2.99 s | 2.89 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.31 s | 0.36 s | 0.67 s | 1.08 s | 1.90 s | 3.02 s | 2.90 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.29 s | 0.38 s | 0.70 s | 1.12 s | 1.95 s | 3.15 s | 3.05 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.28 s | 0.36 s | 0.67 s | 1.06 s | 1.88 s | 3.02 s | 3.35 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.29 s | 0.36 s | 0.67 s | 1.07 s | 1.87 s | 2.97 s | 3.30 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.29 s | 0.35 s | 0.67 s | 1.09 s | 1.89 s | 3.02 s | 2.90 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.30 s | 0.36 s | 0.66 s | 1.07 s | 1.90 s | 3.06 s | 2.96 s |

## iPhone 15, iOS 18.5, mid tier

**Before:** [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) of 2026-11-03. **After:** [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) of 2026-11-05.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 2.31 s (download: 0.34 s, 6.5 MiB) | 1.72 s (download: 0.51 s, 11.6 MiB) | 3.08 s (download: 0.92 s, 21.6 MiB) | 5.73 s (download: 1.99 s, 41.9 MiB) | 10.90 s (download: 3.53 s, 84.2 MiB) | 17.67 s (download: 5.79 s, 138.6 MiB) | 17.25 s (download: 5.56 s, 133.1 MiB) |
| | after | 2.39 s (download: 0.37 s, 6.5 MiB) (+3 %) | 1.71 s (download: 0.51 s, 11.6 MiB) | 3.09 s (download: 0.93 s, 21.6 MiB) | 6.01 s (download: 1.80 s, 41.9 MiB) (+5 %) | 11.25 s (download: 3.64 s, 84.2 MiB) (+3 %) | 18.22 s (download: 6.02 s, 138.6 MiB) (+3 %) | 17.25 s (download: 5.66 s, 133.1 MiB) |
| warm | before | 0.11 s | 0.17 s | 0.31 s | 0.50 s | 0.89 s | 1.45 s | 1.42 s |
| | after | 0.11 s | 0.17 s | 0.31 s | 0.50 s | 0.88 s | 1.45 s | 1.43 s |
| first run after restart | before | 0.20 s | 0.28 s | 0.45 s | 0.75 s | 1.33 s | 2.14 s | 1.96 s |
| | after | 0.21 s | 0.27 s | 0.45 s | 0.74 s | 1.31 s | 2.14 s | 1.87 s (-4 %) |
| repeated after restart | before | 0.12 s | 0.18 s | 0.32 s | 0.52 s | 0.92 s | 1.55 s | 1.55 s |
| | after | 0.11 s | 0.18 s | 0.32 s | 0.50 s | 0.89 s | 1.50 s (-3 %) | 1.51 s |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.08 s | 0.10 s | 0.14 s | 0.24 s | 0.42 s | 0.62 s | 0.45 s |
| | after | 0.08 s | 0.10 s | 0.14 s | 0.23 s | 0.43 s | 0.63 s | 0.40 s |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.11 s | 0.17 s | 0.31 s | 0.51 s | 0.90 s | 1.49 s | 1.46 s |
| | after | 0.11 s | 0.17 s | 0.31 s | 0.50 s | 0.88 s | 1.48 s | 1.45 s |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.11 s | 0.17 s | 0.31 s | 0.50 s | 0.88 s | 1.48 s | 1.45 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.11 s | 0.17 s | 0.31 s | 0.51 s | 0.90 s | 1.49 s | 1.46 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.11 s | 0.17 s | 0.31 s | 0.50 s | 0.88 s | 1.48 s | 1.44 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.14 s | 0.22 s | 0.36 s | 0.65 s | 1.10 s | 1.70 s | 1.71 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.12 s | 0.18 s | 0.31 s | 0.50 s | 0.90 s | 1.49 s | 1.48 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.12 s | 0.18 s | 0.32 s | 0.53 s | 0.93 s | 1.53 s | 1.49 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.11 s | 0.18 s | 0.32 s | 0.51 s | 0.89 s | 1.49 s | 1.47 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.12 s | 0.18 s | 0.32 s | 0.53 s | 0.92 s | 1.54 s | 1.50 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.12 s | 0.18 s | 0.32 s | 0.55 s | 0.96 s | 1.52 s | 1.49 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.15 s | 0.21 s | 0.37 s | 0.66 s | 1.10 s | 1.73 s | 1.65 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.11 s | 0.18 s | 0.32 s | 0.51 s | 0.89 s | 1.49 s | 1.45 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.12 s | 0.18 s | 0.32 s | 0.51 s | 0.90 s | 1.52 s | 1.48 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.11 s | 0.18 s | 0.32 s | 0.51 s | 0.90 s | 1.50 s | 1.48 s |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.13 s | 0.19 s | 0.34 s | 0.56 s | 1.00 s | 1.63 s | 1.58 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.15 s | 0.25 s | 0.43 s | 0.70 s | 1.13 s | 1.69 s | 1.82 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.11 s | 0.17 s | 0.31 s | 0.50 s | 0.88 s | 1.49 s | 1.45 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.13 s | 0.18 s | 0.32 s | 0.52 s | 0.92 s | 1.45 s | 1.44 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.15 s | 0.21 s | 0.38 s | 0.65 s | 1.15 s | 1.79 s | 1.70 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.12 s | 0.18 s | 0.32 s | 0.52 s | 0.94 s | 1.52 s | 1.50 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.11 s | 0.17 s | 0.32 s | 0.50 s | 0.88 s | 1.47 s | 1.44 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.21 s | 0.27 s | 0.45 s | 0.74 s | 1.31 s | 2.14 s | 1.87 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.20 s | 0.28 s | 0.45 s | 0.75 s | 1.33 s | 2.14 s | 1.96 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.21 s | 0.27 s | 0.45 s | 0.73 s | 1.27 s | 2.12 s | 1.97 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.25 s | 0.35 s | 0.56 s | 0.95 s | 1.75 s | 2.50 s | 2.59 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.22 s | 0.28 s | 0.46 s | 0.76 s | 1.35 s | 2.19 s | 1.97 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.21 s | 0.29 s | 0.47 s | 0.80 s | 1.41 s | 2.29 s | 2.03 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.21 s | 0.27 s | 0.45 s | 0.75 s | 1.38 s | 2.20 s | 1.98 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.21 s | 0.28 s | 0.49 s | 0.80 s | 1.42 s | 2.36 s | 2.15 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.22 s | 0.29 s | 0.47 s | 0.92 s | 1.47 s | 2.28 s | 2.05 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.25 s | 0.36 s | 0.58 s | 0.98 s | 1.68 s | 2.70 s | 2.54 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.22 s | 0.27 s | 0.45 s | 0.76 s | 1.32 s | 2.17 s | 2.04 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.22 s | 0.29 s | 0.47 s | 0.77 s | 1.40 s | 2.27 s | 2.07 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.22 s | 0.28 s | 0.46 s | 0.76 s | 1.34 s | 1.99 s | 1.95 s |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.21 s | 0.30 s | 0.51 s | 0.86 s | 1.57 s | 2.37 s | 2.28 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.32 s | 0.55 s | 0.85 s | 1.15 s | 1.91 s | 2.81 s | 2.97 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.21 s | 0.28 s | 0.46 s | 0.77 s | 1.36 s | 2.10 s | 1.93 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.22 s | 0.29 s | 0.48 s | 0.80 s | 1.41 s | 2.18 s | 1.94 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.24 s | 0.38 s | 0.59 s | 1.00 s | 1.69 s | 2.75 s | 2.63 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.22 s | 0.29 s | 0.48 s | 0.79 s | 1.39 s | 2.27 s | 2.04 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.22 s | 0.27 s | 0.46 s | 0.74 s | 1.32 s | 2.16 s | 2.01 s |

## iPhone 17 Pro, iOS 26.3.1, high tier

**Before:** [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) of 2026-11-03. **After:** [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) of 2026-11-05.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 8.77 s (download: 0.85 s, 6.5 MiB) | 2.44 s (download: 1.59 s, 11.6 MiB) | 3.75 s (download: 2.24 s, 21.6 MiB) | 7.10 s (download: 4.28 s, 41.9 MiB) | 13.75 s (download: 8.24 s, 84.2 MiB) | 22.17 s (download: 13.55 s, 138.6 MiB) | 22.49 s (download: 14.08 s, 133.1 MiB) |
| | after | 7.95 s (download: 0.26 s, 6.5 MiB) (-9 %) | 1.13 s (download: 0.28 s, 11.6 MiB) (-54 % **!**) | 2.27 s (download: 0.75 s, 21.6 MiB) (-39 % **!**) | 3.88 s (download: 0.98 s, 41.9 MiB) (-45 % **!**) | 7.60 s (download: 1.94 s, 84.2 MiB) (-45 % **!**) | 12.21 s (download: 3.38 s, 138.6 MiB) (-45 % **!**) | 11.47 s (download: 2.93 s, 133.1 MiB) (-49 % **!**) |
| warm | before | 0.08 s | 0.11 s | 0.19 s | 0.33 s | 0.56 s | 0.77 s | 0.77 s |
| | after | 0.08 s | 0.11 s | 0.19 s | 0.32 s | 0.55 s | 0.78 s | 0.74 s |
| first run after restart | before | 0.15 s | 0.19 s | 0.30 s | 0.50 s | 0.76 s | 1.12 s | 1.12 s |
| | after | 0.16 s | 0.19 s | 0.30 s | 0.48 s | 0.78 s | 1.09 s | 1.10 s |
| repeated after restart | before | 0.09 s | 0.12 s | 0.21 s | 0.35 s | 0.59 s | 0.84 s | 0.82 s |
| | after | 0.09 s | 0.12 s | 0.21 s | 0.34 s | 0.57 s | 0.82 s | 0.81 s |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.06 s | 0.07 s | 0.09 s | 0.14 s | 0.20 s | 0.29 s | 0.29 s |
| | after | 0.07 s | 0.07 s | 0.09 s | 0.13 s | 0.20 s | 0.28 s | 0.29 s |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.56 s | 0.80 s | 0.79 s |
| | after | 0.08 s | 0.12 s | 0.20 s | 0.34 s | 0.56 s | 0.79 s | 0.77 s |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.08 s | 0.12 s | 0.20 s | 0.34 s | 0.56 s | 0.79 s | 0.77 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.56 s | 0.80 s | 0.79 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.58 s | 0.80 s | 0.78 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.57 s | 0.81 s | 0.77 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.08 s | 0.12 s | 0.21 s | 0.35 s | 0.57 s | 0.82 s | 0.80 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.08 s | 0.12 s | 0.21 s | 0.35 s | 0.58 s | 0.82 s | 0.80 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.57 s | 0.81 s | 0.78 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.57 s | 0.80 s | 0.80 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.58 s | 0.81 s | 0.81 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.09 s | 0.12 s | 0.21 s | 0.36 s | 0.60 s | 0.85 s | 0.83 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.08 s | 0.11 s | 0.20 s | 0.34 s | 0.57 s | 0.78 s | 0.77 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.08 s | 0.11 s | 0.19 s | 0.33 s | 0.55 s | 0.78 s | 0.76 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.09 s | 0.13 s | 0.23 s | 0.38 s | 0.64 s | 0.92 s | 0.89 s |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.09 s | 0.12 s | 0.21 s | 0.35 s | 0.57 s | 0.83 s | 0.81 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.08 s | 0.11 s | 0.20 s | 0.34 s | 0.55 s | 0.78 s | 0.77 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.09 s | 0.12 s | 0.21 s | 0.35 s | 0.58 s | 0.84 s | 0.80 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.09 s | 0.12 s | 0.24 s | 0.37 s | 0.60 s | 0.86 s | 0.85 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.09 s | 0.12 s | 0.20 s | 0.34 s | 0.56 s | 0.81 s | 0.79 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.08 s | 0.11 s | 0.20 s | 0.34 s | 0.55 s | 0.79 s | 0.78 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.09 s | 0.12 s | 0.20 s | 0.34 s | 0.58 s | 0.81 s | 0.78 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`e84c2c8`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.16 s | 0.19 s | 0.30 s | 0.48 s | 0.78 s | 1.09 s | 1.10 s |
| [`161510a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.15 s | 0.19 s | 0.30 s | 0.50 s | 0.76 s | 1.12 s | 1.12 s |
| [`d89eb76`](https://github.com/helius-labs/zolana-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.16 s | 0.20 s | 0.31 s | 0.50 s | 0.79 s | 1.13 s | 1.11 s |
| [`68d596a`](https://github.com/helius-labs/zolana-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.15 s | 0.20 s | 0.30 s | 0.49 s | 0.77 s | 1.13 s | 1.10 s |
| [`455dffe`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.16 s | 0.20 s | 0.32 s | 0.51 s | 0.81 s | 1.16 s | 1.13 s |
| [`eb54573`](https://github.com/helius-labs/zolana-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.16 s | 0.21 s | 0.31 s | 0.49 s | 0.80 s | 1.16 s | 1.12 s |
| [`a8bc647`](https://github.com/helius-labs/zolana-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.16 s | 0.20 s | 0.30 s | 0.49 s | 0.80 s | 1.14 s | 1.11 s |
| [`b8247fc`](https://github.com/helius-labs/zolana-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.16 s | 0.19 s | 0.30 s | 0.51 s | 0.81 s | 1.14 s | 1.12 s |
| [`1c78bf4`](https://github.com/helius-labs/zolana-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.15 s | 0.20 s | 0.33 s | 0.50 s | 0.82 s | 1.15 s | 1.12 s |
| [`afda065`](https://github.com/helius-labs/zolana-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.16 s | 0.21 s | 0.33 s | 0.51 s | 0.83 s | 1.17 s | 1.18 s |
| [`8c6af08`](https://github.com/helius-labs/zolana-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.16 s | 0.19 s | 0.30 s | 0.50 s | 0.78 s | 1.08 s | 1.09 s |
| [`e30b328`](https://github.com/helius-labs/zolana-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.15 s | 0.19 s | 0.29 s | 0.47 s | 0.80 s | 1.09 s | 1.08 s |
| [`418d376`](https://github.com/helius-labs/zolana-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.17 s | 0.23 s | 0.36 s | 0.54 s | 0.92 s | 1.29 s | 1.28 s |
| [`455be9e`](https://github.com/helius-labs/zolana-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.16 s | 0.21 s | 0.32 s | 0.50 s | 0.80 s | 1.16 s | 1.14 s |
| [`842ed6b`](https://github.com/helius-labs/zolana-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.15 s | 0.19 s | 0.29 s | 0.48 s | 0.76 s | 1.09 s | 1.09 s |
| [`3457ad0`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3457ad00eca9cacb4294e39032c6b339cf47e019) | 2026-10-25 | 0.17 s | 0.21 s | 0.33 s | 0.51 s | 0.81 s | 1.17 s | 1.15 s |
| [`c13ac6c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/c13ac6cb4072d7a9bac4067fcfd1f7db1ec90f60) | 2026-10-23 | 0.17 s | 0.22 s | 0.34 s | 0.53 s | 0.85 s | 1.21 s | 1.18 s |
| [`964d396`](https://github.com/helius-labs/zolana-mobile-sdk/commit/964d396cf2443cecce962763d87f78655226c7cf) | 2026-10-23 | 0.16 s | 0.19 s | 0.31 s | 0.50 s | 0.79 s | 1.14 s | 1.13 s |
| [`3dd795c`](https://github.com/helius-labs/zolana-mobile-sdk/commit/3dd795c879d76401167acd449f0c32a31d24e021) | 2026-10-23 | 0.15 s | 0.19 s | 0.30 s | 0.48 s | 0.78 s | 1.09 s | 1.10 s |
| [`030dc90`](https://github.com/helius-labs/zolana-mobile-sdk/commit/030dc902f09661f34b94700f3c8e5fc192051ca9) | 2026-10-23 | 0.16 s | 0.20 s | 0.32 s | 0.49 s | 0.81 s | 1.14 s | 1.12 s |
