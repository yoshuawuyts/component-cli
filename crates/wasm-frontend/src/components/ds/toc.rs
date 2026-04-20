//! Table of contents.

/// Render the HTML for this section.
pub(crate) fn render() -> &'static str {
    r##"
    <nav class="py-6 grid grid-flow-col grid-rows-12 md:grid-rows-6 gap-y-2 gap-x-6 text-[13px]">
      <a href="#colors" class="text-ink-700 hover:text-ink-900">01 — Color</a>
      <a href="#typography" class="text-ink-700 hover:text-ink-900">02 — Typography</a>
      <a href="#spacing" class="text-ink-700 hover:text-ink-900">03 — Spacing & Radius</a>
      <a href="#elevation" class="text-ink-700 hover:text-ink-900">04 — Elevation</a>
      <a href="#buttons" class="text-ink-700 hover:text-ink-900">05 — Buttons</a>
      <a href="#tabs" class="text-ink-700 hover:text-ink-900">06 — Tabs & Pills</a>
      <a href="#nav" class="text-ink-700 hover:text-ink-900">07 — Navigation</a>
      <a href="#code" class="text-ink-700 hover:text-ink-900">08 — Code Samples</a>
      <a href="#bars" class="text-ink-700 hover:text-ink-900">09 — Labels</a>
      <a href="#tooltip" class="text-ink-700 hover:text-ink-900">10 — Tooltip</a>
      <a href="#table" class="text-ink-700 hover:text-ink-900">11 — Table</a>
      <a href="#icons" class="text-ink-700 hover:text-ink-900">12 — Icons</a>
      <a href="#fields" class="text-ink-700 hover:text-ink-900">13 — Form Fields</a>
      <a href="#toggles" class="text-ink-700 hover:text-ink-900">14 — Checkbox · Radio · Switch</a>
      <a href="#badges" class="text-ink-700 hover:text-ink-900">15 — Badges</a>
      <a href="#dropdown" class="text-ink-700 hover:text-ink-900">16 — Dropdown</a>
      <a href="#modal" class="text-ink-700 hover:text-ink-900">17 — Modal</a>
      <a href="#breadcrumb" class="text-ink-700 hover:text-ink-900">18 — Breadcrumb & Pagination</a>
      <a href="#progress" class="text-ink-700 hover:text-ink-900">19 — Progress & Spinner</a>
      <a href="#empty" class="text-ink-700 hover:text-ink-900">20 — Empty State</a>
      <a href="#grid" class="text-ink-700 hover:text-ink-900">21 — Grid</a>
      <a href="#regions" class="text-ink-700 hover:text-ink-900">22 — Regions</a>
      <a href="#motion" class="text-ink-700 hover:text-ink-900">23 — Motion</a>
      <a href="#details" class="text-ink-700 hover:text-ink-900">24 — Details</a>
      <span class="text-ink-400 mono uppercase tracking-wider text-[11px] mt-2">Components</span>
      <a href="#c-sidebar" class="text-ink-700 hover:text-ink-900">C01 — Nested Sidebar</a>
      <a href="#c-toc" class="text-ink-700 hover:text-ink-900">C02 — On This Page</a>
      <a href="#c-page-header" class="text-ink-700 hover:text-ink-900">C03 — Page Header</a>
      <a href="#c-item-list" class="text-ink-700 hover:text-ink-900">C04 — Item List</a>
      <a href="#c-item-details" class="text-ink-700 hover:text-ink-900">C05 — Item Details</a>
      <a href="#c-navbar" class="text-ink-700 hover:text-ink-900">C06 — Navbar</a>
    </nav>
"##
}
