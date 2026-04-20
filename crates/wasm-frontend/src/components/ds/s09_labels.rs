//! 09 — Labels.

/// Render the HTML for this section.
pub(crate) fn render() -> &'static str {
    r##"
    <section id="bars" class="pt-12 md:pt-16">
      <div class="grid md:grid-cols-[200px_1fr] gap-6 md:gap-12">
        <div>
          <div class="text-[12px] mono uppercase tracking-wider text-ink-500">09</div>
          <h2 class="mt-2 text-[24px] font-semibold tracking-tight">Labels</h2>
          <p class="mt-2 text-[13px] text-ink-500 leading-relaxed">
            28px tall, 6px radius, label inset 12px. Pastel fill with paired ink for
            text — 4.5:1 contrast minimum.
          </p>
        </div>
        <div class="flex flex-col items-start gap-2">
          <div class="bar bg-cat-blue text-cat-blueInk">Lorem ipsum dolor</div>
          <div class="bar bg-cat-pink text-cat-pinkInk">Sit amet</div>
          <div class="bar bg-cat-cream text-cat-creamInk">Consectetur</div>
          <div class="bar bg-cat-green text-cat-greenInk">Adipiscing elit</div>
          <div class="bar bg-cat-peach text-cat-peachInk">Sed do eiusmod</div>
          <div class="bar bg-cat-lilac text-cat-lilacInk">Tempor incididunt</div>
          <div class="bar bg-cat-teal text-cat-tealInk">Ut labore</div>
          <div class="bar bg-cat-rust text-cat-rustInk">Et dolore magna</div>
          <div class="bar bg-cat-plum text-cat-plumInk">Aliqua enim</div>
          <div class="bar bg-cat-slate text-cat-slateInk">Ad minim veniam</div>
        </div>
      </div>
    </section>
"##
}
