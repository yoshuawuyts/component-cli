//! 01 — Color.

/// Render this section.
pub(crate) fn render() -> String {
    let content = r##"<div class="space-y-10">
          <!-- Surfaces -->
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Surfaces</h3>
            <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
              <div>
                <div class="swatch bg-canvas"></div>
                <div class="mt-2 text-[13px]">Canvas</div>
                <div class="text-[12px] text-ink-500 mono">#F4F4F5</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.967 .001 286)</div>
              </div>
              <div>
                <div class="swatch bg-surface"></div>
                <div class="mt-2 text-[13px]">Surface</div>
                <div class="text-[12px] text-ink-500 mono">#FFFFFF</div>
                <div class="text-[11px] text-ink-400 mono">oklch(1 0 0)</div>
              </div>
              <div>
                <div class="swatch bg-surfaceMuted"></div>
                <div class="mt-2 text-[13px]">Surface Muted</div>
                <div class="text-[12px] text-ink-500 mono">#E8E8EA</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.928 .003 286)</div>
              </div>
            </div>
          </div>

          <!-- Ink -->
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Ink</h3>
            <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
              <div>
                <div class="swatch bg-ink-900"></div>
                <div class="mt-2 text-[13px]">Ink 900</div>
                <div class="text-[12px] text-ink-500 mono">#18181B</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.210 .006 286)</div>
              </div>
              <div>
                <div class="swatch bg-ink-700"></div>
                <div class="mt-2 text-[13px]">Ink 700</div>
                <div class="text-[12px] text-ink-500 mono">#3F3F46</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.370 .013 286)</div>
              </div>
              <div>
                <div class="swatch bg-ink-500"></div>
                <div class="mt-2 text-[13px]">Ink 500</div>
                <div class="text-[12px] text-ink-500 mono">#71717A</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.552 .016 286)</div>
              </div>
              <div>
                <div class="swatch bg-ink-400"></div>
                <div class="mt-2 text-[13px]">Ink 400</div>
                <div class="text-[12px] text-ink-500 mono">#A1A1AA</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.705 .015 286)</div>
              </div>
              <div>
                <div class="swatch bg-ink-300"></div>
                <div class="mt-2 text-[13px]">Ink 300</div>
                <div class="text-[12px] text-ink-500 mono">#D4D4D8</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.871 .006 286)</div>
              </div>
            </div>
          </div>

          <!-- Lines -->
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Lines</h3>
            <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
              <div>
                <div class="swatch bg-line"></div>
                <div class="mt-2 text-[13px]">Line</div>
                <div class="text-[12px] text-ink-500 mono">#D4D4D8</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.871 .006 286)</div>
              </div>
              <div>
                <div class="swatch bg-lineSoft"></div>
                <div class="mt-2 text-[13px]">Line Soft</div>
                <div class="text-[12px] text-ink-500 mono">#E4E4E7</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.920 .004 286)</div>
              </div>
            </div>
          </div>

          <!-- Semantic -->
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Semantic</h3>
            <div class="grid grid-cols-2 md:grid-cols-3 gap-4">
              <div>
                <div class="swatch bg-positive"></div>
                <div class="mt-2 text-[13px]">Positive</div>
                <div class="text-[12px] text-ink-500 mono">#1F8A4C</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.561 .149 149)</div>
              </div>
              <div>
                <div class="swatch bg-cat-pinkInk"></div>
                <div class="mt-2 text-[13px]">Negative (pinkInk)</div>
                <div class="text-[12px] text-ink-500 mono">#9B4F5E</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.490 .080 13)</div>
              </div>
            </div>
          </div>

          <!-- Categorical -->
          <div>
            <h3 class="text-[13px] mono uppercase tracking-wider text-ink-500 mb-3">Categorical</h3>
            <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
              <div>
                <div class="swatch bg-cat-blue flex items-end p-3"><span
                    class="text-[12px] text-cat-blueInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Blue</div>
                <div class="text-[12px] text-ink-500 mono">#D6E4FF / #3D5A99</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.910 .046 264) / oklch(.430 .102 263)</div>
              </div>
              <div>
                <div class="swatch bg-cat-pink flex items-end p-3"><span
                    class="text-[12px] text-cat-pinkInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Pink</div>
                <div class="text-[12px] text-ink-500 mono">#FBD9DF / #9B4F5E</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.910 .037 9) / oklch(.490 .080 13)</div>
              </div>
              <div>
                <div class="swatch bg-cat-green flex items-end p-3"><span
                    class="text-[12px] text-cat-greenInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Green</div>
                <div class="text-[12px] text-ink-500 mono">#D2ECD8 / #3F7A52</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.918 .039 148) / oklch(.503 .089 149)</div>
              </div>
              <div>
                <div class="swatch bg-cat-peach flex items-end p-3"><span
                    class="text-[12px] text-cat-peachInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Peach</div>
                <div class="text-[12px] text-ink-500 mono">#F8E2C2 / #8E6529</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.911 .049 79) / oklch(.499 .089 70)</div>
              </div>
              <div>
                <div class="swatch bg-cat-lilac flex items-end p-3"><span
                    class="text-[12px] text-cat-lilacInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Lilac</div>
                <div class="text-[12px] text-ink-500 mono">#D5C8EF / #5A3D8C</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.852 .051 287) / oklch(.395 .120 287)</div>
              </div>
              <div>
                <div class="swatch bg-cat-cream flex items-end p-3"><span
                    class="text-[12px] text-cat-creamInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Cream</div>
                <div class="text-[12px] text-ink-500 mono">#F4ECC2 / #7A6A2A</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.937 .055 100) / oklch(.490 .073 96)</div>
              </div>
              <div>
                <div class="swatch bg-cat-teal flex items-end p-3"><span
                    class="text-[12px] text-cat-tealInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Teal</div>
                <div class="text-[12px] text-ink-500 mono">#BFE3EE / #1F6F87</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.890 .037 215) / oklch(.470 .080 224)</div>
              </div>
              <div>
                <div class="swatch bg-cat-rust flex items-end p-3"><span
                    class="text-[12px] text-cat-rustInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Rust</div>
                <div class="text-[12px] text-ink-500 mono">#F4D2C0 / #9F5536</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.880 .045 50) / oklch(.510 .104 42)</div>
              </div>
              <div>
                <div class="swatch bg-cat-plum flex items-end p-3"><span
                    class="text-[12px] text-cat-plumInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Plum</div>
                <div class="text-[12px] text-ink-500 mono">#E8C5E8 / #7E2E7E</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.851 .065 322) / oklch(.408 .135 326)</div>
              </div>
              <div>
                <div class="swatch bg-cat-slate flex items-end p-3"><span
                    class="text-[12px] text-cat-slateInk font-medium">Aa</span></div>
                <div class="mt-2 text-[13px]">Slate</div>
                <div class="text-[12px] text-ink-500 mono">#DADCE0 / #535A66</div>
                <div class="text-[11px] text-ink-400 mono">oklch(.882 .005 264) / oklch(.450 .015 257)</div>
              </div>
            </div>
          </div>
        </div>"##;
    super::section(
        "colors",
        "01",
        "Color",
        "Neutral surfaces and ink form the structural base. Pastel categoricals encode chart series with paired ink tones for legibility.",
        content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(render());
    }
}
