//! Copy-to-clipboard heading component.
//!
//! Renders an h2 heading with a copy button that appears on hover.
//! Includes the embedded JavaScript for clipboard interaction.

use crate::components::icon::{self, Icon, IconSize};

/// Render a page heading with a copy-to-clipboard button.
///
/// The copy button appears on hover and copies `fqn` to the clipboard.
/// After copying, the icon changes to a checkmark for 2 seconds.
///
/// - `name`: displayed heading text
/// - `subtitle`: small label below (e.g. "Interface", "World")
/// - `fqn`: fully-qualified name copied to clipboard
/// - `color_class`: Tailwind text color for the heading (e.g. `"text-wit-iface"`)
/// - `docs_html`: rendered markdown docs to show below the heading
/// - `version`: optional version badge shown inline with the heading
pub(crate) fn heading_with_copy(
    name: &str,
    subtitle: &str,
    fqn: &str,
    color_class: &str,
    docs_html: &str,
) -> String {
    heading_with_copy_and_version(name, subtitle, fqn, color_class, docs_html, None)
}

/// Render a page heading with copy button and optional version badge.
pub(crate) fn heading_with_copy_and_version(
    name: &str,
    subtitle: &str,
    fqn: &str,
    color_class: &str,
    docs_html: &str,
    version: Option<&str>,
) -> String {
    let copy_icon = icon::svg(Icon::Copy, IconSize::Md);
    let check_icon = icon::svg(Icon::Check, IconSize::Md);

    let version_badge = version.map_or_else(String::new, |v| {
        format!(
            r#" <span class="text-[11px] font-normal px-1.5 py-0.5 rounded bg-surfaceMuted text-ink-500 ml-1">{v}</span>"#
        )
    });

    format!(
        r#"<div class="max-w-3xl mb-6">
  <h2 class="text-[28px] sm:text-[36px] font-semibold tracking-tight flex items-baseline gap-2 group flex-wrap">
    <span class="{color_class}">{name}</span>{version_badge}
    <button id="copy-fqn-btn" class="text-ink-400 hover:text-ink-900 transition-opacity cursor-pointer opacity-0 group-hover:opacity-100" style="font-size:0.5em;vertical-align:middle" title="Copy item path to clipboard">{copy_icon}</button>
  </h2>
  <span class="text-[13px] text-ink-500 mt-1 block">{subtitle}</span>
  <div class="mt-4">{docs_html}</div>
</div>
<script>
(function(){{
  var btn=document.getElementById('copy-fqn-btn');
  var copyIcon="{copy_icon}";
  var checkIcon="{check_icon}";
  btn.addEventListener('click',function(){{
    navigator.clipboard.writeText('{fqn}').then(function(){{
      btn.innerHTML=checkIcon;
      setTimeout(function(){{btn.innerHTML=copyIcon}},2000);
    }});
  }});
}})();
</script>"#
    )
}
