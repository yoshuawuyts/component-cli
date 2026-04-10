//! Navigation bar component.

/// A breadcrumb segment: (label, optional href).
pub(crate) struct Crumb {
    /// Display text.
    pub label: String,
    /// Link target, or `None` for the current (last) segment.
    pub href: Option<String>,
}

/// Render the site navigation bar with home link, breadcrumbs, and search.
#[must_use]
pub(crate) fn render(crumbs: &[Crumb]) -> String {
    let mut breadcrumb_html = String::new();
    for crumb in crumbs {
        breadcrumb_html.push_str(r#" <span class="text-fg-faint mx-1">/</span> "#);
        if let Some(href) = &crumb.href {
            use std::fmt::Write;
            write!(
                breadcrumb_html,
                r#"<a href="{href}" class="text-fg-muted hover:text-fg transition-colors">{label}</a>"#,
                label = crumb.label
            )
            .unwrap();
        } else {
            use std::fmt::Write;
            write!(
                breadcrumb_html,
                r#"<span class="text-fg">{label}</span>"#,
                label = crumb.label
            )
            .unwrap();
        }
    }

    format!(
        r#"<nav class="w-full max-w-6xl mx-auto px-6 sm:px-8 pt-6 pb-4 flex items-center justify-between gap-4" aria-label="Main">
  <div class="flex items-center text-sm min-w-0">
    <a href="/" id="bunny" class="text-lg font-medium text-fg hover:text-accent transition-colors shrink-0" style="cursor:pointer">(๑╹ᆺ╹)</a>{breadcrumb_html}
  </div>
  <form action="/search" method="get" class="flex max-w-sm shrink-0">
    <input type="search" name="q" placeholder="Search…" aria-label="Search" class="w-40 px-3 py-1.5 text-sm border-2 border-fg bg-page text-fg focus:border-accent focus:outline-none">
    <button type="submit" class="px-4 py-1.5 text-sm bg-fg text-page border-2 border-fg border-l-0 transition-transform hover:scale-105">Search</button>
  </form>
  <script>
  (function(){{
    var b=document.getElementById('bunny');
    if(!b)return;
    var anims=[
      ['(๑╹ᆺ╹)','(๑°ᆺ°)!','(๑◉ᆺ◉)!!'],
      ['(๑╹ᆺ╹)','(๑°ᆺ°)♪','ヽ(๑≧ᆺ≦)ノ'],
      ['(๑╹ᆺ╹)','(๑╹ᆺ╹)>','(๑°ᆺ°)>>']
    ];
    var seq=anims[Math.floor(Math.random()*anims.length)];
    var timer=null;
    b.addEventListener('mouseenter',function(){{
      if(timer)return;
      b.textContent=seq[1];
      timer=setTimeout(function(){{
        b.textContent=seq[2];
      }},80);
    }});
    b.addEventListener('mouseleave',function(){{
      if(timer){{clearTimeout(timer);timer=null;}}
      b.textContent=seq[0];
    }});
  }})();
  </script>
</nav>"#,
    )
}
