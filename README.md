# Proof timing on main

Written by CI after every push to `main` (`scripts/proof-timing-history.py` on `main`), from release builds on the phones of `scripts/devicefarm/devices.json` in AWS Device Farm. Two runs of one commit differ by up to 5 % on the Galaxy A16 and A56 and up to 15 % on the S25 and the iPhones. The data is in `history/`, a line per commit.

## Samsung Galaxy A16, Android 14, low tier

**Before:** [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) of 2026-11-05. **After:** [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) of 2026-11-13.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 4.98 s (download: 1.81 s, 6.5 MiB) | 9.94 s (download: 4.76 s, 11.6 MiB) | 15.55 s (download: 6.59 s, 21.6 MiB) | 35.93 s (download: 18.87 s, 41.9 MiB) | 69.14 s (download: 36.75 s, 84.2 MiB) | 108.35 s (download: 57.41 s, 138.6 MiB) | 110.03 s (download: 58.72 s, 133.1 MiB) |
| | after | 6.59 s (download: 3.46 s, 6.5 MiB) (+32 % **!**) | 10.37 s (download: 5.38 s, 11.6 MiB) (+4 %) | 19.24 s (download: 10.04 s, 21.6 MiB) (+24 % **!**) | 35.49 s (download: 18.64 s, 41.9 MiB) (-1 %) | 70.20 s (download: 38.12 s, 84.2 MiB) (+2 %) | 114.87 s (download: 63.58 s, 138.6 MiB) (+6 %) | 106.29 s (download: 57.90 s, 133.1 MiB) (-3 %) |
| warm | before | 0.88 s | 1.49 s | 2.41 s | 4.49 s | 8.19 s | 12.21 s | 12.59 s |
| | after | 0.84 s | 1.39 s (-7 %) | 2.46 s | 4.49 s | 7.98 s (-3 %) | 12.74 s (+4 %) | 12.24 s (-3 %) |
| first run after restart | before | 0.96 s | 1.74 s | 2.85 s | 4.96 s | 8.88 s | 13.97 s | 12.83 s |
| | after | 0.96 s | 1.68 s (-4 %) | 2.76 s (-3 %) | 4.83 s (-3 %) | 8.45 s (-5 %) | 13.37 s (-4 %) | 13.14 s (+2 %) |
| repeated after restart | before | 0.84 s | 1.39 s | 2.52 s | 4.44 s | 8.20 s | 12.80 s | 12.36 s |
| | after | 0.85 s | 1.47 s (+6 %) | 2.43 s (-4 %) | 4.52 s (+2 %) | 7.84 s (-4 %) | 12.90 s (+1 %) | 12.24 s (-1 %) |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.20 s | 0.26 s | 0.32 s | 0.39 s | 0.71 s | 0.82 s | 0.67 s |
| | after | 0.20 s | 0.25 s | 0.30 s | 0.41 s | 0.56 s (-21 % **!**) | 0.73 s (-10 %) | 0.81 s (+21 % **!**) |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.01 s | 0.01 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.01 s | 0.01 s |
| prove | before | 0.82 s | 1.43 s | 2.46 s | 4.51 s | 8.15 s | 12.81 s | 12.45 s |
| | after | 0.82 s | 1.39 s | 2.43 s | 4.48 s | 7.91 s (-3 %) | 12.66 s (-1 %) | 12.22 s (-2 %) |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.82 s | 1.39 s | 2.43 s | 4.48 s | 7.91 s | 12.66 s | 12.22 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.82 s | 1.43 s | 2.46 s | 4.51 s | 8.15 s | 12.81 s | 12.45 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.84 s | 1.42 s | 2.42 s | 4.44 s | 7.91 s | 12.71 s | 12.62 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.84 s | 1.42 s | 2.41 s | 4.54 s | 7.97 s | 12.90 s | 12.09 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.82 s | 1.43 s | 2.46 s | 4.47 s | 7.90 s | 12.63 s | 12.40 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.83 s | 1.39 s | 2.38 s | 4.54 s | 7.86 s | 12.65 s | 12.30 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.82 s | 1.40 s | 2.40 s | 4.47 s | 8.08 s | 12.71 s | 12.55 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.85 s | 1.42 s | 2.42 s | 4.45 s | 8.06 s | 12.63 s | 12.34 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.83 s | 1.42 s | 2.40 s | 4.40 s | 8.09 s | 12.57 s | 12.46 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.86 s | 1.41 s | 2.46 s | 4.48 s | 8.08 s | 12.76 s | 12.41 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.82 s | 1.37 s | 2.47 s | 4.39 s | 7.92 s | 12.89 s | 12.27 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.81 s | 1.43 s | 2.44 s | 4.46 s | 8.13 s | 12.89 s | 12.25 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.82 s | 1.38 s | 2.42 s | 4.50 s | 8.04 s | 12.57 s | 12.25 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.80 s | 1.34 s | 2.43 s | 4.52 s | 8.00 s | 12.50 s | 12.35 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.81 s | 1.42 s | 2.41 s | 4.50 s | 7.90 s | 12.41 s | 12.13 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.80 s | 1.40 s | 2.41 s | 4.57 s | 7.98 s | 12.68 s | 12.13 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.87 s | 1.36 s | 2.49 s | 4.44 s | 8.09 s | 12.65 s | 12.58 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.82 s | 1.39 s | 2.43 s | 4.54 s | 8.28 s | 12.53 s | 12.17 s |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.82 s | 1.37 s | 2.39 s | 4.45 s | 8.02 s | 12.67 s | 12.15 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.82 s | 1.35 s | 2.43 s | 4.44 s | 7.80 s | 12.77 s | 12.37 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.96 s | 1.68 s | 2.76 s | 4.83 s | 8.45 s | 13.37 s | 13.14 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.96 s | 1.74 s | 2.85 s | 4.96 s | 8.88 s | 13.97 s | 12.83 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 1.02 s | 1.69 s | 2.97 s | 4.89 s | 8.59 s | 14.05 s | 13.55 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 1.04 s | 1.77 s | 2.75 s | 5.00 s | 8.46 s | 13.73 s | 13.29 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.96 s | 1.68 s | 2.77 s | 4.80 s | 8.64 s | 13.40 s | 13.03 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.98 s | 1.71 s | 2.75 s | 4.92 s | 8.62 s | 13.42 s | 13.28 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 1.05 s | 1.72 s | 2.76 s | 4.90 s | 8.53 s | 13.70 s | 13.29 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.98 s | 1.68 s | 2.71 s | 4.87 s | 8.35 s | 13.76 s | 12.82 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.98 s | 1.60 s | 2.72 s | 4.75 s | 8.86 s | 13.56 s | 13.22 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 1.04 s | 1.65 s | 2.78 s | 5.10 s | 8.88 s | 13.76 s | 13.22 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.98 s | 1.68 s | 2.72 s | 4.80 s | 8.73 s | 13.97 s | 13.12 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.98 s | 1.66 s | 2.76 s | 4.83 s | 9.00 s | 13.60 s | 12.98 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 1.02 s | 1.71 s | 2.77 s | 5.08 s | 8.57 s | 13.20 s | 13.03 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 1.00 s | 1.70 s | 2.81 s | 5.03 s | 8.54 s | 13.29 s | 13.33 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.96 s | 1.63 s | 2.83 s | 4.84 s | 8.66 s | 13.54 s | 12.81 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.98 s | 1.66 s | 2.79 s | 5.04 s | 8.56 s | 13.44 s | 13.10 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 1.02 s | 1.66 s | 2.95 s | 4.99 s | 8.74 s | 13.57 s | 13.41 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 1.05 s | 1.69 s | 2.75 s | 4.75 s | 9.16 s | 13.53 s | 12.73 s |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.99 s | 1.78 s | 2.70 s | 4.88 s | 8.66 s | 13.75 s | 12.99 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.99 s | 1.66 s | 2.72 s | 5.04 s | 8.99 s | 13.81 s | 13.41 s |

## Samsung Galaxy A56, Android 15, mid tier

**Before:** [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) of 2026-11-05. **After:** [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) of 2026-11-13.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 3.06 s (download: 1.99 s, 6.5 MiB) | 4.02 s (download: 2.27 s, 11.6 MiB) | 6.43 s (download: 3.33 s, 21.6 MiB) | 12.34 s (download: 6.47 s, 41.9 MiB) | 24.57 s (download: 13.25 s, 84.2 MiB) | 39.27 s (download: 21.21 s, 138.6 MiB) | 39.70 s (download: 22.29 s, 133.1 MiB) |
| | after | 3.75 s (download: 2.70 s, 6.5 MiB) (+22 % **!**) | 5.02 s (download: 3.27 s, 11.6 MiB) (+25 % **!**) | 6.83 s (download: 3.78 s, 21.6 MiB) (+6 %) | 12.44 s (download: 6.63 s, 41.9 MiB) (+1 %) | 24.11 s (download: 12.79 s, 84.2 MiB) (-2 %) | 39.01 s (download: 20.84 s, 138.6 MiB) (-1 %) | 37.79 s (download: 20.47 s, 133.1 MiB) (-5 %) |
| warm | before | 0.27 s | 0.43 s | 0.75 s | 1.41 s | 2.72 s | 4.37 s | 4.26 s |
| | after | 0.25 s | 0.42 s | 0.76 s | 1.39 s | 2.72 s | 4.34 s | 4.31 s |
| first run after restart | before | 0.59 s | 0.59 s | 0.94 s | 1.61 s | 3.13 s | 4.79 s | 4.64 s |
| | after | 0.51 s (-14 %) | 0.61 s | 0.95 s | 1.63 s | 3.12 s | 4.93 s (+3 %) | 4.80 s (+3 %) |
| repeated after restart | before | 0.27 s | 0.42 s | 0.75 s | 1.39 s | 2.71 s | 4.33 s | 4.24 s |
| | after | 0.26 s | 0.43 s | 0.76 s | 1.37 s | 2.73 s | 4.47 s (+3 %) | 4.24 s |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.31 s | 0.14 s | 0.15 s | 0.20 s | 0.31 s | 0.35 s | 0.34 s |
| | after | 0.20 s (-34 % **!**) | 0.13 s | 0.16 s | 0.21 s | 0.29 s | 0.44 s (+27 % **!**) | 0.45 s (+31 % **!**) |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.27 s | 0.42 s | 0.75 s | 1.40 s | 2.72 s | 4.40 s | 4.28 s |
| | after | 0.25 s | 0.42 s | 0.76 s | 1.38 s | 2.74 s | 4.46 s (+2 %) | 4.28 s |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.25 s | 0.42 s | 0.76 s | 1.38 s | 2.74 s | 4.46 s | 4.28 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.27 s | 0.42 s | 0.75 s | 1.40 s | 2.72 s | 4.40 s | 4.28 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.26 s | 0.43 s | 0.76 s | 1.39 s | 2.70 s | 4.45 s | 4.24 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.26 s | 0.44 s | 0.77 s | 1.42 s | 2.72 s | 4.41 s | 4.26 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.26 s | 0.43 s | 0.76 s | 1.39 s | 2.75 s | 4.44 s | 4.31 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.25 s | 0.44 s | 0.77 s | 1.41 s | 2.73 s | 4.43 s | 4.31 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.27 s | 0.46 s | 0.79 s | 1.41 s | 2.80 s | 4.42 s | 4.32 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.26 s | 0.43 s | 0.77 s | 1.38 s | 2.74 s | 4.37 s | 4.25 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.25 s | 0.44 s | 0.76 s | 1.39 s | 2.71 s | 4.37 s | 4.30 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.26 s | 0.43 s | 0.76 s | 1.40 s | 2.69 s | 4.39 s | 4.30 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.26 s | 0.43 s | 0.76 s | 1.41 s | 2.72 s | 4.41 s | 4.28 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.26 s | 0.43 s | 0.75 s | 1.39 s | 2.72 s | 4.40 s | 4.32 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.26 s | 0.43 s | 0.77 s | 1.41 s | 2.74 s | 4.43 s | 4.28 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.26 s | 0.43 s | 0.77 s | 1.41 s | 2.75 s | 4.38 s | 4.40 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.26 s | 0.43 s | 0.81 s | 1.40 s | 2.75 s | 4.37 s | 4.26 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.26 s | 0.43 s | 0.76 s | 1.40 s | 2.75 s | 4.36 s | 4.28 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.26 s | 0.43 s | 0.76 s | 1.41 s | 2.77 s | 4.40 s | 4.31 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.25 s | 0.43 s | 0.76 s | 1.40 s | 2.76 s | 4.38 s | 4.24 s |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.26 s | 0.42 s | 0.75 s | 1.41 s | 2.74 s | 4.36 s | 4.27 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.26 s | 0.44 s | 0.76 s | 1.41 s | 2.79 s | 4.37 s | 4.29 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.51 s | 0.61 s | 0.95 s | 1.63 s | 3.12 s | 4.93 s | 4.80 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.59 s | 0.59 s | 0.94 s | 1.61 s | 3.13 s | 4.79 s | 4.64 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.53 s | 0.56 s | 0.95 s | 1.68 s | 3.45 s | 4.82 s | 4.62 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.52 s | 0.62 s | 0.95 s | 1.65 s | 3.00 s | 4.78 s | 4.64 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.52 s | 0.61 s | 0.96 s | 1.63 s | 3.05 s | 4.91 s | 4.79 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.52 s | 0.57 s | 0.96 s | 1.70 s | 3.05 s | 4.83 s | 4.82 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.53 s | 0.59 s | 0.96 s | 1.66 s | 3.07 s | 4.82 s | 4.82 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.59 s | 0.59 s | 0.92 s | 1.58 s | 3.01 s | 4.72 s | 4.60 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.54 s | 0.62 s | 1.01 s | 1.62 s | 3.01 s | 4.79 s | 4.71 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.51 s | 0.58 s | 0.92 s | 1.63 s | 2.94 s | 4.88 s | 4.81 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.64 s | 0.58 s | 0.95 s | 1.61 s | 3.03 s | 4.78 s | 4.66 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.51 s | 0.59 s | 0.96 s | 1.62 s | 3.06 s | 4.87 s | 4.75 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.52 s | 0.59 s | 0.95 s | 1.62 s | 3.06 s | 4.80 s | 4.78 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.52 s | 0.60 s | 0.96 s | 1.63 s | 3.11 s | 4.74 s | 4.74 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.53 s | 0.61 s | 0.95 s | 1.65 s | 3.13 s | 4.93 s | 4.84 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.51 s | 0.58 s | 0.96 s | 1.63 s | 3.34 s | 4.78 s | 4.66 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.55 s | 0.61 s | 0.99 s | 1.62 s | 3.03 s | 4.75 s | 4.68 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.54 s | 0.59 s | 0.95 s | 1.66 s | 3.10 s | 4.76 s | 4.60 s |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.53 s | 0.55 s | 0.92 s | 1.61 s | 3.09 s | 4.73 s | 4.67 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.52 s | 0.57 s | 0.94 s | 1.67 s | 3.24 s | 4.71 s | 4.63 s |

## Samsung Galaxy S25, Android 15, high tier

**Before:** [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) of 2026-11-05. **After:** [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) of 2026-11-13.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 13.23 s (download: 1.09 s, 6.5 MiB) | 2.27 s (download: 1.15 s, 11.6 MiB) | 4.01 s (download: 2.29 s, 21.6 MiB) | 7.62 s (download: 4.76 s, 41.9 MiB) | 13.92 s (download: 8.56 s, 84.2 MiB) | 23.72 s (download: 15.29 s, 138.6 MiB) | 22.30 s (download: 14.15 s, 133.1 MiB) |
| | after | 13.81 s (download: 1.19 s, 6.5 MiB) (+4 %) | 2.54 s (download: 1.46 s, 11.6 MiB) (+12 %) | 4.34 s (download: 2.56 s, 21.6 MiB) (+8 %) | 8.20 s (download: 5.24 s, 41.9 MiB) (+7 %) | 18.94 s (download: 13.43 s, 84.2 MiB) (+36 % **!**) | 27.41 s (download: 18.61 s, 138.6 MiB) (+16 %) | 24.19 s (download: 15.66 s, 133.1 MiB) (+8 %) |
| warm | before | 0.16 s | 0.19 s | 0.31 s | 0.50 s | 0.86 s | 1.40 s | 1.39 s |
| | after | 0.15 s | 0.20 s | 0.36 s | 0.51 s | 0.85 s | 1.53 s (+10 %) | 1.53 s (+11 %) |
| first run after restart | before | 0.50 s | 0.54 s | 0.66 s | 0.95 s | 1.51 s | 2.41 s |  |
| | after | 0.50 s | 0.64 s (+19 %) | 0.82 s (+25 % **!**) | 1.33 s (+40 % **!**) | 2.16 s (+43 % **!**) | 5.04 s (+110 % **!**) | 4.97 s |
| repeated after restart | before | 0.15 s | 0.20 s | 0.31 s | 0.49 s | 0.87 s |  |  |
| | after | 0.14 s | 0.20 s | 0.32 s | 0.80 s (+62 % **!**) | 1.83 s (+109 % **!**) | 3.34 s | 3.25 s |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.35 s | 0.34 s | 0.34 s | 0.46 s | 0.65 s | 0.98 s |  |
| | after | 0.35 s | 0.44 s (+28 % **!**) | 0.51 s (+50 % **!**) | 0.52 s (+14 %) | 0.77 s (+18 %) | 1.71 s (+73 % **!**) | 1.69 s |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.15 s | 0.18 s | 0.30 s | 0.49 s | 0.86 s | 1.41 s | 1.38 s |
| | after | 0.14 s | 0.19 s | 0.33 s | 0.65 s (+34 % **!**) | 1.13 s (+31 % **!**) | 2.43 s (+72 % **!**) | 2.38 s (+73 % **!**) |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.14 s | 0.19 s | 0.33 s | 0.65 s | 1.13 s | 2.43 s | 2.38 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.15 s | 0.18 s | 0.30 s | 0.49 s | 0.86 s | 1.41 s | 1.38 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.15 s | 0.19 s | 0.33 s | 0.62 s | 1.06 s | 1.90 s | 1.91 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.14 s | 0.19 s | 0.30 s | 0.48 s | 0.90 s | 2.07 s | 2.39 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.15 s | 0.20 s | 0.56 s | 0.63 s | 1.12 s | 2.14 s | 2.10 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.15 s | 0.20 s | 0.33 s | 0.50 s | 0.92 s | 2.41 s | 2.43 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.14 s | 0.19 s | 0.32 s | 0.51 s | 0.90 s | 1.98 s | 1.70 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.15 s | 0.19 s | 0.30 s | 0.53 s | 0.93 s | 2.17 s | 1.85 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.15 s | 0.23 s | 0.34 s | 0.53 s | 0.89 s | 1.47 s | 1.48 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.15 s | 0.21 s | 0.33 s | 0.49 s | 0.89 s | 1.89 s | 1.89 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.15 s | 0.19 s | 0.30 s | 0.48 s | 0.85 s | 1.40 s | 1.36 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.14 s | 0.19 s | 0.31 s | 0.48 s | 0.89 s | 1.89 s | 2.15 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.15 s | 0.19 s | 0.30 s | 0.57 s | 1.05 s | 2.14 s | 2.15 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.15 s | 0.20 s | 0.60 s | 0.69 s | 1.33 s | 2.22 s | 2.31 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.15 s | 0.20 s | 0.35 s | 0.59 s | 1.04 s | 2.05 s | 2.00 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.14 s | 0.19 s | 0.30 s | 0.59 s | 1.01 s | 1.68 s | 1.66 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.14 s | 0.19 s | 0.32 s | 0.48 s | 0.88 s | 1.89 s | 1.82 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.14 s | 0.19 s | 0.30 s | 0.48 s | 0.85 s | 1.39 s | 1.36 s |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.15 s | 0.19 s | 0.33 s | 0.52 s | 0.87 s | 1.78 s | 1.75 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.15 s | 0.19 s | 0.30 s | 0.49 s | 0.86 s | 1.40 s | 1.37 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.50 s | 0.64 s | 0.82 s | 1.33 s | 2.16 s | 5.04 s | 4.97 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.50 s | 0.54 s | 0.66 s | 0.95 s | 1.51 s | 2.41 s |  |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.56 s | 0.64 s | 0.91 s | 1.40 s | 2.16 s | 3.65 s | 3.55 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.50 s | 0.50 s | 0.64 s | 0.99 s | 1.67 s | 3.70 s | 5.55 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.64 s | 0.67 s | 0.82 s | 1.42 s | 2.27 s | 3.56 s | 3.57 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.56 s | 0.62 s | 0.87 s | 1.07 s | 1.94 s | 4.60 s | 4.94 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.47 s | 0.51 s | 0.66 s | 0.97 s | 1.58 s | 3.44 s | 4.00 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.50 s | 0.51 s | 0.74 s | 1.04 s | 1.72 s | 3.98 s | 4.36 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.50 s | 0.57 s | 0.76 s | 1.06 s | 1.69 s | 2.58 s |  |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.56 s | 0.66 s | 0.91 s | 1.23 s | 1.77 s | 3.66 s | 3.50 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.54 s | 0.52 s | 0.72 s | 0.96 s | 1.52 s | 2.33 s |  |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.50 s | 0.50 s | 0.67 s | 1.02 s | 1.67 s | 3.34 s | 4.61 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.49 s | 0.53 s | 0.63 s | 1.17 s | 1.93 s | 4.49 s | 4.72 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.63 s | 0.69 s | 0.85 s | 1.64 s | 2.62 s | 4.14 s | 3.89 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.55 s | 0.66 s | 0.94 s | 1.94 s | 2.20 s | 4.34 s | 4.30 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.50 s | 0.53 s | 0.66 s | 1.14 s | 1.84 s | 2.86 s | 2.78 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.48 s | 0.53 s | 0.70 s | 1.08 s | 1.75 s | 3.48 s | 3.37 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.47 s | 0.49 s | 0.65 s | 0.94 s | 1.51 s |  |  |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.57 s | 0.64 s | 0.80 s | 1.05 s | 1.73 s | 3.25 s | 3.19 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.49 s | 0.50 s | 0.65 s | 1.00 s | 1.65 s |  |  |

## iPhone 12, iOS 18.5, low tier

**Before:** [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) of 2026-11-05. **After:** [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) of 2026-11-13.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 4.48 s (download: 2.10 s, 6.5 MiB) | 3.36 s (download: 1.93 s, 11.6 MiB) | 5.34 s (download: 2.82 s, 21.6 MiB) | 9.46 s (download: 4.79 s, 41.9 MiB) | 18.60 s (download: 9.27 s, 84.2 MiB) | 31.66 s (download: 16.55 s, 138.6 MiB) | 20.75 s (download: 5.82 s, 133.1 MiB) |
| | after | 2.87 s (download: 0.41 s, 6.5 MiB) (-36 % **!**) | 2.12 s (download: 0.55 s, 11.6 MiB) (-37 % **!**) | 3.57 s (download: 0.96 s, 21.6 MiB) (-33 % **!**) | 6.69 s (download: 1.87 s, 41.9 MiB) (-29 % **!**) | 14.52 s (download: 3.73 s, 84.2 MiB) (-22 % **!**) | 22.09 s (download: 6.00 s, 138.6 MiB) (-30 % **!**) | 20.88 s (download: 5.77 s, 133.1 MiB) (+1 %) |
| warm | before | 0.15 s | 0.23 s | 0.41 s | 0.67 s | 1.30 s | 2.16 s | 2.24 s |
| | after | 0.14 s | 0.23 s | 0.42 s | 0.73 s (+9 %) | 1.30 s | 2.18 s | 2.35 s (+5 %) |
| first run after restart | before | 0.28 s | 0.38 s | 0.71 s | 1.14 s | 1.99 s | 3.17 s | 3.06 s |
| | after | 0.42 s (+52 % **!**) | 0.68 s (+79 % **!**) | 1.03 s (+45 % **!**) | 1.43 s (+26 % **!**) | 2.28 s (+15 %) | 3.63 s (+14 %) | 3.58 s (+17 %) |
| repeated after restart | before | 0.15 s | 0.26 s | 0.50 s | 0.85 s | 1.54 s | 2.51 s | 2.46 s |
| | after | 0.23 s (+55 % **!**) | 0.39 s (+49 % **!**) | 0.63 s (+25 % **!**) | 0.94 s (+10 %) | 1.70 s (+10 %) | 2.92 s (+16 %) | 2.90 s (+18 %) |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.10 s | 0.13 s | 0.20 s | 0.30 s | 0.51 s | 0.73 s | 0.68 s |
| | after | 0.17 s (+79 % **!**) | 0.27 s (+99 % **!**) | 0.35 s (+79 % **!**) | 0.47 s (+59 % **!**) | 0.67 s (+32 % **!**) | 1.01 s (+38 % **!**) | 0.84 s (+25 % **!**) |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.15 s | 0.25 s | 0.48 s | 0.77 s | 1.38 s | 2.30 s | 2.31 s |
| | after | 0.20 s (+35 % **!**) | 0.34 s (+34 % **!**) | 0.54 s (+13 %) | 0.83 s (+8 %) | 1.45 s (+5 %) | 2.40 s (+4 %) | 2.54 s (+10 %) |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.20 s | 0.34 s | 0.54 s | 0.83 s | 1.45 s | 2.40 s | 2.54 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.15 s | 0.25 s | 0.48 s | 0.77 s | 1.38 s | 2.30 s | 2.31 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.15 s | 0.23 s | 0.44 s | 0.72 s | 1.26 s | 2.05 s | 2.08 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.15 s | 0.24 s | 0.47 s | 0.76 s | 1.34 s | 2.25 s | 2.26 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.14 s | 0.23 s | 0.43 s | 0.71 s | 1.26 s | 2.06 s | 2.06 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.15 s | 0.26 s | 0.47 s | 0.76 s | 1.39 s | 2.30 s | 2.42 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.15 s | 0.23 s | 0.43 s | 0.71 s | 1.25 s | 2.06 s | 2.02 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.14 s | 0.23 s | 0.43 s | 0.72 s | 1.26 s | 2.07 s | 2.05 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.15 s | 0.24 s | 0.45 s | 0.73 s | 1.30 s | 2.16 s | 2.15 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.14 s | 0.23 s | 0.43 s | 0.71 s | 1.26 s | 2.06 s | 2.08 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.20 s | 0.31 s | 0.52 s | 0.80 s | 1.46 s | 2.43 s | 2.53 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.15 s | 0.23 s | 0.43 s | 0.72 s | 1.25 s | 2.08 s | 2.06 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.15 s | 0.23 s | 0.44 s | 0.72 s | 1.27 s | 2.14 s | 2.12 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.15 s | 0.24 s | 0.45 s | 0.74 s | 1.31 s | 2.19 s | 2.29 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.15 s | 0.24 s | 0.46 s | 0.75 s | 1.33 s | 2.24 s | 2.26 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.14 s | 0.23 s | 0.42 s | 0.67 s |  |  |  |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.15 s | 0.24 s | 0.46 s | 0.75 s | 1.34 s | 2.22 s | 2.18 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.15 s | 0.23 s |  |  |  |  |  |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.14 s | 0.23 s | 0.43 s | 0.71 s | 1.25 s | 2.03 s | 2.02 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.15 s | 0.23 s | 0.43 s | 0.72 s | 1.25 s | 2.04 s | 2.03 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.42 s | 0.68 s | 1.03 s | 1.43 s | 2.28 s | 3.63 s | 3.58 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.28 s | 0.38 s | 0.71 s | 1.14 s | 1.99 s | 3.17 s | 3.06 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.29 s | 0.36 s | 0.68 s | 1.07 s | 1.90 s | 3.08 s | 2.98 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.28 s | 0.42 s | 0.69 s | 1.11 s | 1.94 s | 3.11 s | 3.02 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.29 s | 0.42 s | 0.66 s | 1.07 s | 1.92 s | 3.05 s | 2.91 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.31 s | 0.45 s | 0.75 s | 1.20 s | 2.11 s | 3.37 s | 3.69 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.30 s | 0.37 s | 0.67 s | 1.07 s | 1.86 s | 3.02 s | 2.92 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.29 s | 0.38 s | 0.67 s | 1.08 s | 1.89 s | 3.06 s | 2.95 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.29 s | 0.42 s | 0.69 s | 1.12 s | 1.98 s | 3.17 s | 3.06 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.30 s | 0.38 s | 0.68 s | 1.11 s | 1.88 s | 3.06 s | 2.99 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.35 s | 0.58 s | 0.88 s | 1.30 s | 2.29 s | 3.48 s | 3.49 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.30 s | 0.36 s | 0.67 s | 1.07 s | 1.92 s | 3.04 s | 2.92 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.30 s | 0.36 s | 0.67 s | 1.04 s | 1.81 s | 2.97 s | 2.86 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.30 s | 0.38 s | 0.69 s | 1.13 s | 2.00 s | 3.17 s | 3.33 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.29 s | 0.37 s | 0.67 s | 1.10 s | 1.92 s | 3.08 s | 3.05 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 |  |  |  |  |  |  |  |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.29 s | 0.36 s | 0.66 s | 1.08 s | 1.86 s | 3.05 s | 2.90 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 |  |  |  |  |  |  |  |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.30 s | 0.36 s | 0.66 s | 1.06 s | 1.85 s | 2.99 s | 2.89 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.31 s | 0.36 s | 0.67 s | 1.08 s | 1.90 s | 3.02 s | 2.90 s |

## iPhone 15, iOS 18.5, mid tier

**Before:** [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) of 2026-11-05. **After:** [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) of 2026-11-13.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 2.44 s (download: 0.40 s, 6.5 MiB) | 1.70 s (download: 0.52 s, 11.6 MiB) | 3.01 s (download: 0.95 s, 21.6 MiB) | 5.59 s (download: 1.78 s, 41.9 MiB) | 11.03 s (download: 3.55 s, 84.2 MiB) | 17.74 s (download: 5.82 s, 138.6 MiB) | 17.31 s (download: 5.59 s, 133.1 MiB) |
| | after | 2.37 s (download: 0.38 s, 6.5 MiB) (-3 %) | 1.90 s (download: 0.52 s, 11.6 MiB) (+11 %) | 3.31 s (download: 0.97 s, 21.6 MiB) (+10 %) | 6.10 s (download: 2.01 s, 41.9 MiB) (+9 %) | 11.73 s (download: 3.57 s, 84.2 MiB) (+6 %) | 19.12 s (download: 5.86 s, 138.6 MiB) (+8 %) | 18.43 s (download: 5.63 s, 133.1 MiB) (+6 %) |
| warm | before | 0.12 s | 0.18 s | 0.32 s | 0.51 s | 0.88 s | 1.46 s | 1.43 s |
| | after | 0.11 s | 0.18 s | 0.31 s | 0.49 s | 0.88 s | 1.43 s | 1.41 s |
| first run after restart | before | 0.22 s | 0.28 s | 0.46 s | 0.76 s | 1.34 s | 2.23 s | 2.05 s |
| | after | 0.23 s | 0.32 s | 0.51 s (+11 %) | 0.86 s (+13 %) | 1.50 s (+12 %) | 2.29 s (+3 %) | 2.18 s (+6 %) |
| repeated after restart | before | 0.12 s | 0.18 s | 0.32 s | 0.52 s | 0.92 s | 1.55 s | 1.54 s |
| | after | 0.14 s | 0.20 s | 0.35 s | 0.61 s (+19 %) | 1.11 s (+20 %) | 1.73 s (+12 %) | 1.71 s (+11 %) |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.08 s | 0.10 s | 0.15 s | 0.25 s | 0.45 s | 0.69 s | 0.57 s |
| | after | 0.09 s | 0.12 s | 0.16 s | 0.26 s | 0.45 s | 0.66 s | 0.57 s |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.11 s | 0.17 s | 0.32 s | 0.51 s | 0.90 s | 1.49 s | 1.45 s |
| | after | 0.13 s | 0.20 s | 0.33 s | 0.55 s | 0.96 s (+6 %) | 1.54 s (+3 %) | 1.51 s (+4 %) |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.13 s | 0.20 s | 0.33 s | 0.55 s | 0.96 s | 1.54 s | 1.51 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.11 s | 0.17 s | 0.32 s | 0.51 s | 0.90 s | 1.49 s | 1.45 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.11 s | 0.17 s | 0.32 s | 0.50 s | 0.88 s | 1.47 s | 1.45 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.11 s | 0.17 s | 0.31 s | 0.50 s | 0.90 s | 1.48 s | 1.44 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.16 s | 0.28 s | 0.43 s | 0.73 s | 1.19 s | 1.69 s | 1.83 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.11 s | 0.17 s | 0.31 s | 0.50 s | 0.88 s | 1.48 s | 1.45 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.11 s | 0.17 s | 0.31 s | 0.51 s | 0.90 s | 1.49 s | 1.46 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.11 s | 0.17 s | 0.31 s | 0.50 s | 0.88 s | 1.48 s | 1.44 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.14 s | 0.22 s | 0.36 s | 0.65 s | 1.10 s | 1.70 s | 1.71 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.12 s | 0.18 s | 0.31 s | 0.50 s | 0.90 s | 1.49 s | 1.48 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.12 s | 0.18 s | 0.32 s | 0.53 s | 0.93 s | 1.53 s | 1.49 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.11 s | 0.18 s | 0.32 s | 0.51 s | 0.89 s | 1.49 s | 1.47 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.12 s | 0.18 s | 0.32 s | 0.53 s | 0.92 s | 1.54 s | 1.50 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.12 s | 0.18 s | 0.32 s | 0.55 s | 0.96 s | 1.52 s | 1.49 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.15 s | 0.21 s | 0.37 s | 0.66 s | 1.10 s | 1.73 s | 1.65 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.11 s | 0.18 s | 0.32 s | 0.51 s | 0.89 s | 1.49 s | 1.45 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.12 s | 0.18 s | 0.32 s | 0.51 s | 0.90 s | 1.52 s | 1.48 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.11 s | 0.18 s | 0.32 s | 0.51 s | 0.90 s | 1.50 s | 1.48 s |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.13 s | 0.19 s | 0.34 s | 0.56 s | 1.00 s | 1.63 s | 1.58 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.15 s | 0.25 s | 0.43 s | 0.70 s | 1.13 s | 1.69 s | 1.82 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.23 s | 0.32 s | 0.51 s | 0.86 s | 1.50 s | 2.29 s | 2.18 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.22 s | 0.28 s | 0.46 s | 0.76 s | 1.34 s | 2.23 s | 2.05 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.20 s | 0.28 s | 0.46 s | 0.76 s | 1.32 s | 1.99 s | 1.88 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.20 s | 0.27 s | 0.45 s | 0.73 s | 1.29 s | 2.11 s | 1.89 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.32 s | 0.59 s | 0.80 s | 1.16 s | 1.92 s | 2.78 s | 2.99 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.21 s | 0.27 s | 0.45 s | 0.74 s | 1.31 s | 2.14 s | 1.87 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.20 s | 0.28 s | 0.45 s | 0.75 s | 1.33 s | 2.14 s | 1.96 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.21 s | 0.27 s | 0.45 s | 0.73 s | 1.27 s | 2.12 s | 1.97 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.25 s | 0.35 s | 0.56 s | 0.95 s | 1.75 s | 2.50 s | 2.59 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.22 s | 0.28 s | 0.46 s | 0.76 s | 1.35 s | 2.19 s | 1.97 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.21 s | 0.29 s | 0.47 s | 0.80 s | 1.41 s | 2.29 s | 2.03 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.21 s | 0.27 s | 0.45 s | 0.75 s | 1.38 s | 2.20 s | 1.98 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.21 s | 0.28 s | 0.49 s | 0.80 s | 1.42 s | 2.36 s | 2.15 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.22 s | 0.29 s | 0.47 s | 0.92 s | 1.47 s | 2.28 s | 2.05 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.25 s | 0.36 s | 0.58 s | 0.98 s | 1.68 s | 2.70 s | 2.54 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.22 s | 0.27 s | 0.45 s | 0.76 s | 1.32 s | 2.17 s | 2.04 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.22 s | 0.29 s | 0.47 s | 0.77 s | 1.40 s | 2.27 s | 2.07 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.22 s | 0.28 s | 0.46 s | 0.76 s | 1.34 s | 1.99 s | 1.95 s |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.21 s | 0.30 s | 0.51 s | 0.86 s | 1.57 s | 2.37 s | 2.28 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.32 s | 0.55 s | 0.85 s | 1.15 s | 1.91 s | 2.81 s | 2.97 s |

## iPhone 17 Pro, iOS 26.3.1, high tier

**Before:** [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) of 2026-11-05. **After:** [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) of 2026-11-13.

| by scenario | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| first run, including download | before | 8.24 s (download: 0.35 s, 6.5 MiB) | 1.41 s (download: 0.51 s, 11.6 MiB) | 2.48 s (download: 0.92 s, 21.6 MiB) | 4.62 s (download: 1.73 s, 41.9 MiB) | 9.10 s (download: 3.43 s, 84.2 MiB) | 14.56 s (download: 5.64 s, 138.6 MiB) | 14.15 s (download: 5.40 s, 133.1 MiB) |
| | after | 8.14 s (download: 0.36 s, 6.5 MiB) (-1 %) | 1.40 s (download: 0.53 s, 11.6 MiB) | 2.71 s (download: 1.15 s, 21.6 MiB) (+9 %) | 4.74 s (download: 1.80 s, 41.9 MiB) (+3 %) | 9.66 s (download: 3.84 s, 84.2 MiB) (+6 %) | 15.18 s (download: 6.14 s, 138.6 MiB) (+4 %) | 14.15 s (download: 5.46 s, 133.1 MiB) |
| warm | before | 0.08 s | 0.12 s | 0.20 s | 0.33 s | 0.55 s | 0.80 s | 0.77 s |
| | after | 0.08 s | 0.11 s | 0.20 s | 0.34 s | 0.56 s | 0.81 s | 0.78 s |
| first run after restart | before | 0.16 s | 0.20 s | 0.32 s | 0.50 s | 0.82 s | 1.14 s | 1.13 s |
| | after | 0.15 s | 0.21 s | 0.31 s | 0.50 s | 0.81 s | 1.17 s | 1.14 s |
| repeated after restart | before | 0.09 s | 0.13 s | 0.22 s | 0.36 s | 0.61 s | 0.85 s | 0.85 s |
| | after | 0.09 s | 0.13 s | 0.22 s | 0.36 s | 0.60 s | 0.88 s | 0.85 s |

| by stage | | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| key load | before | 0.07 s | 0.07 s | 0.10 s | 0.14 s | 0.21 s | 0.29 s | 0.31 s |
| | after | 0.06 s | 0.08 s | 0.09 s | 0.14 s | 0.21 s | 0.29 s | 0.30 s |
| witness | before | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| | after | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s | 0.00 s |
| prove | before | 0.09 s | 0.12 s | 0.21 s | 0.35 s | 0.58 s | 0.82 s | 0.80 s |
| | after | 0.09 s | 0.12 s | 0.21 s | 0.35 s | 0.59 s | 0.84 s | 0.80 s |

### Proof, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.09 s | 0.12 s | 0.21 s | 0.35 s | 0.59 s | 0.84 s | 0.80 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.09 s | 0.12 s | 0.21 s | 0.35 s | 0.58 s | 0.82 s | 0.80 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.56 s | 0.80 s | 0.78 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.08 s | 0.12 s | 0.20 s | 0.34 s | 0.57 s | 0.81 s | 0.78 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.09 s | 0.12 s | 0.21 s | 0.36 s | 0.59 s | 0.84 s | 0.83 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.08 s | 0.12 s | 0.20 s | 0.34 s | 0.56 s | 0.79 s | 0.77 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.56 s | 0.80 s | 0.79 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.58 s | 0.80 s | 0.78 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.57 s | 0.81 s | 0.77 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.08 s | 0.12 s | 0.21 s | 0.35 s | 0.57 s | 0.82 s | 0.80 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.08 s | 0.12 s | 0.21 s | 0.35 s | 0.58 s | 0.82 s | 0.80 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.57 s | 0.81 s | 0.78 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.57 s | 0.80 s | 0.80 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.08 s | 0.12 s | 0.20 s | 0.35 s | 0.58 s | 0.81 s | 0.81 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.09 s | 0.12 s | 0.21 s | 0.36 s | 0.60 s | 0.85 s | 0.83 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.08 s | 0.11 s | 0.20 s | 0.34 s | 0.57 s | 0.78 s | 0.77 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.08 s | 0.11 s | 0.19 s | 0.33 s | 0.55 s | 0.78 s | 0.76 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.09 s | 0.13 s | 0.23 s | 0.38 s | 0.64 s | 0.92 s | 0.89 s |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.09 s | 0.12 s | 0.21 s | 0.35 s | 0.57 s | 0.83 s | 0.81 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.08 s | 0.11 s | 0.20 s | 0.34 s | 0.55 s | 0.78 s | 0.77 s |

### First run after a restart, last 20 commits

| commit | date | 1 → 2 | 2 → 2 | 4 → 2 | 8 → 2 | 16 → 2 | 24 → 2 | merge 24 → 1 |
|---|---|---|---|---|---|---|---|---|
| [`0d224e4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/0d224e4d9565b9194ea73eab108e5e937d2b8fdc) | 2026-11-13 | 0.15 s | 0.21 s | 0.31 s | 0.50 s | 0.81 s | 1.17 s | 1.14 s |
| [`c0fda16`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/c0fda16a22a85e74875359afe271aebf5dabe573) | 2026-11-05 | 0.16 s | 0.20 s | 0.32 s | 0.50 s | 0.82 s | 1.14 s | 1.13 s |
| [`5726c91`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5726c91130722ef0464ef59d101f21d6872c5c0a) | 2026-11-05 | 0.17 s | 0.19 s | 0.31 s | 0.50 s | 0.79 s | 1.11 s | 1.10 s |
| [`afbb4ec`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afbb4ec453c559620a0ed74de2a7ca1193d519a2) | 2026-11-05 | 0.15 s | 0.21 s | 0.30 s | 0.50 s | 0.81 s | 1.13 s | 1.09 s |
| [`5c183e5`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/5c183e5fd6a3dba5768573178c716ae331abd512) | 2026-11-05 | 0.17 s | 0.21 s | 0.34 s | 0.52 s | 0.84 s | 1.18 s | 1.18 s |
| [`e84c2c8`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e84c2c812f909379db4947b265f7a952bc5c4e23) | 2026-11-05 | 0.16 s | 0.19 s | 0.30 s | 0.48 s | 0.78 s | 1.09 s | 1.10 s |
| [`161510a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/161510a8cec60057372f3d2c291efff688291836) | 2026-11-03 | 0.15 s | 0.19 s | 0.30 s | 0.50 s | 0.76 s | 1.12 s | 1.12 s |
| [`d89eb76`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/d89eb76ef57c0117579e4398ad8c7c1fa14a2c26) | 2026-11-03 | 0.16 s | 0.20 s | 0.31 s | 0.50 s | 0.79 s | 1.13 s | 1.11 s |
| [`68d596a`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/68d596afde8a87e93d56766e4fc55ee19ab49109) | 2026-11-03 | 0.15 s | 0.20 s | 0.30 s | 0.49 s | 0.77 s | 1.13 s | 1.10 s |
| [`455dffe`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455dffe7128844b9b5f1fd892facc8112c154e96) | 2026-11-02 | 0.16 s | 0.20 s | 0.32 s | 0.51 s | 0.81 s | 1.16 s | 1.13 s |
| [`eb54573`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/eb54573996bb0f339a0b7ba5fe2f5922c077efa5) | 2026-10-29 | 0.16 s | 0.21 s | 0.31 s | 0.49 s | 0.80 s | 1.16 s | 1.12 s |
| [`a8bc647`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/a8bc647f2f74544731b499b62742f147efc43f48) | 2026-10-29 | 0.16 s | 0.20 s | 0.30 s | 0.49 s | 0.80 s | 1.14 s | 1.11 s |
| [`b8247fc`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/b8247fc4dde19068e78dd53c201da59a3f86a8a9) | 2026-10-28 | 0.16 s | 0.19 s | 0.30 s | 0.51 s | 0.81 s | 1.14 s | 1.12 s |
| [`1c78bf4`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/1c78bf4993e9023536b2bb71fc5df55744665bf4) | 2026-10-27 | 0.15 s | 0.20 s | 0.33 s | 0.50 s | 0.82 s | 1.15 s | 1.12 s |
| [`afda065`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/afda06572dc104ec6c1cb2e096958dbfce896fd0) | 2026-10-27 | 0.16 s | 0.21 s | 0.33 s | 0.51 s | 0.83 s | 1.17 s | 1.18 s |
| [`8c6af08`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/8c6af08e61ca4b0bc4b68ee3c75cbefa1b2667b6) | 2026-10-27 | 0.16 s | 0.19 s | 0.30 s | 0.50 s | 0.78 s | 1.08 s | 1.09 s |
| [`e30b328`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/e30b328a5d18f58935ab297a695d7fc12ad8c53a) | 2026-10-26 | 0.15 s | 0.19 s | 0.29 s | 0.47 s | 0.80 s | 1.09 s | 1.08 s |
| [`418d376`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/418d37618e67cd4ba68d8b1f5ff50890b54039a8) | 2026-10-26 | 0.17 s | 0.23 s | 0.36 s | 0.54 s | 0.92 s | 1.29 s | 1.28 s |
| [`455be9e`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/455be9ee158cbba0fa0c63ca37e5de678fb8fe6d) | 2026-10-26 | 0.16 s | 0.21 s | 0.32 s | 0.50 s | 0.80 s | 1.16 s | 1.14 s |
| [`842ed6b`](https://github.com/helius-labs/privacy-ring-mobile-sdk/commit/842ed6b9dc8953fe88cf1c3bf79dfc3d8b1d525b) | 2026-10-26 | 0.15 s | 0.19 s | 0.29 s | 0.48 s | 0.76 s | 1.09 s | 1.09 s |
