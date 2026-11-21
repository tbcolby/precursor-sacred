# Sacred Geometry: agent notes

- Package `sacred`; GAM app name `gam::APP_NAME_SACRED` (manifest key `sacred`, context name "Sacred Geometry"); xous-names server `_Sacred Geometry_`.
- `core/` (`sacred-core`) holds the ten figures (`figures`), the Fibonacci spiral tiling (`spiral`) and the selector `Model`, with no Xous dependencies. Run `cd core && cargo test`.
- Trig is `precursor_appkit_core::fixed`; never add a local sine table.
- `src/` is the shell: `UxType::Framebuffer`, raw keys, no pump, lines through `Screen::lines`, circles as unfilled GAM circles (local helper; `Screen::circles` fills white), header/footer repainted over circle spill.
- Gate: `xous-dev-toolkit/bin/integrate --xous-root <tree> --grade <repo>`.
