//! 11 — Table.

use html::tables::{Table, TableBody, TableCell, TableHead, TableRow};
use html::text_content::Division;

const CODE_CHIP: &str = "mono text-[11px] text-ink-700 bg-surfaceMuted px-1 py-0.5 rounded-sm";

fn def_row(name: &Division, desc: TableCell) -> TableRow {
    let name_str = name.to_string();
    TableRow::builder()
        .class("border-b border-lineSoft")
        .table_cell(|td| {
            td.class("py-3 pr-6 align-baseline whitespace-nowrap")
                .text(name_str)
        })
        .push(desc)
        .build()
}

fn build_def_table() -> Table {
    let mut tbody = TableBody::builder();
    tbody.class("text-ink-900");

    // version — string · required
    tbody.push(def_row(
        &Division::builder()
            .span(|s| s.class("mono").text("version"))
            .span(|s| {
                s.class("ml-2 text-[11px] text-ink-500")
                    .text("string \u{00b7} required")
            })
            .build(),
        TableCell::builder()
            .class("py-3 align-baseline text-ink-700")
            .text("SemVer tag, e.g. ")
            .span(|s| s.class("mono").text("0.2.4"))
            .text(".")
            .build(),
    ));

    // signature — string · optional
    tbody.push(def_row(
        &Division::builder()
            .span(|s| s.class("mono").text("signature"))
            .span(|s| {
                s.class("ml-2 text-[11px] text-ink-500")
                    .text("string \u{00b7} optional")
            })
            .build(),
        TableCell::builder()
            .class("py-3 align-baseline text-ink-700")
            .text("Detached cosign-compatible signature, base64 encoded.")
            .build(),
    ));

    // --scope — enum chips
    tbody.push(def_row(
        &Division::builder()
            .span(|s| s.class("mono").text("--scope"))
            .span(|s| {
                s.class("ml-2 inline-flex items-baseline gap-1")
                    .code(|c| c.class(CODE_CHIP).text("user"))
                    .code(|c| c.class(CODE_CHIP).text("project"))
                    .code(|c| c.class(CODE_CHIP).text("system"))
            })
            .build(),
        TableCell::builder()
            .class("py-3 align-baseline text-ink-700")
            .text("Where to write the registry entry. Defaults to ")
            .span(|s| s.class("mono").text("user"))
            .text(".")
            .build(),
    ));

    // --config <path>
    tbody.push(def_row(
        &Division::builder()
            .span(|s| s.class("mono").text("--config"))
            .span(|s| s.class("mono text-ink-400").text("\u{003c}path\u{003e}"))
            .span(|s| s.class("ml-2 text-[11px] text-ink-500").text("path"))
            .build(),
        TableCell::builder()
            .class("py-3 align-baseline text-ink-700")
            .text("Override the project config file location.")
            .build(),
    ));

    Table::builder()
        .class("w-full min-w-[560px] text-[13px]")
        .push(tbody.build())
        .build()
}

pub(crate) struct TabEntry {
    pub(crate) status_class: &'static str,
    pub(crate) status_text: &'static str,
    pub(crate) code: &'static str,
    pub(crate) calls_extra: &'static str,
    pub(crate) calls: &'static str,
    pub(crate) latency_extra: &'static str,
    pub(crate) latency: &'static str,
    pub(crate) meaning: &'static str,
}

pub(crate) const TAB_ENTRIES: &[TabEntry] = &[
    TabEntry {
        status_class: "id-http-status id-http-status-2xx",
        status_text: "200",
        code: "ok",
        calls_extra: "",
        calls: "1 240",
        latency_extra: "",
        latency: "38 ms",
        meaning: "Request succeeded.",
    },
    TabEntry {
        status_class: "id-http-status id-http-status-4xx",
        status_text: "400",
        code: "invalid_request",
        calls_extra: "",
        calls: "12",
        latency_extra: "",
        latency: "4 ms",
        meaning: "Malformed parameter or request body.",
    },
    TabEntry {
        status_class: "id-http-status id-http-status-4xx",
        status_text: "401",
        code: "unauthenticated",
        calls_extra: " text-ink-400",
        calls: "0",
        latency_extra: " text-ink-400",
        latency: "\u{2014}",
        meaning: "Missing or invalid bearer token.",
    },
    TabEntry {
        status_class: "id-http-status id-http-status-5xx",
        status_text: "500",
        code: "internal_error",
        calls_extra: "",
        calls: "1",
        latency_extra: " text-negative",
        latency: "612 ms",
        meaning: "Unexpected server-side failure.",
    },
];

fn build_tab_table(entries: &[TabEntry]) -> Table {
    let thead = TableHead::builder()
        .table_row(|tr| {
            tr.class("text-ink-400")
                .table_header(|th| {
                    th.class("text-left font-normal py-3 pr-6 w-[80px]")
                        .text("Status")
                })
                .table_header(|th| {
                    th.class("text-left font-normal py-3 pr-6 w-[180px]")
                        .text("Code")
                })
                .table_header(|th| {
                    th.class("text-right font-normal py-3 pr-6 w-[100px]")
                        .text("Calls")
                })
                .table_header(|th| {
                    th.class("text-right font-normal py-3 pr-6 w-[120px]")
                        .text("Avg latency")
                })
                .table_header(|th| th.class("text-left font-normal py-3").text("Meaning"))
        })
        .build();

    let mut tbody = TableBody::builder();
    tbody.class("text-ink-900");
    for e in entries {
        let row = TableRow::builder()
            .class("border-t-[1.5px] border-lineSoft")
            .table_cell(|td| {
                td.class("py-3 pr-6 align-baseline")
                    .span(|s| s.class(e.status_class).text(e.status_text))
            })
            .table_cell(|td| {
                td.class("py-3 pr-6 align-baseline mono text-ink-700")
                    .text(e.code)
            })
            .table_cell(|td| {
                td.class(format!(
                    "py-3 pr-6 align-baseline text-right tabular-nums{}",
                    e.calls_extra
                ))
                .text(e.calls)
            })
            .table_cell(|td| {
                td.class(format!(
                    "py-3 pr-6 align-baseline text-right tabular-nums{}",
                    e.latency_extra
                ))
                .text(e.latency)
            })
            .table_cell(|td| td.class("py-3 align-baseline text-ink-700").text(e.meaning))
            .build();
        tbody.push(row);
    }

    Table::builder()
        .class("w-full min-w-[560px] text-[13px]")
        .push(thead)
        .push(tbody.build())
        .build()
}

const DEF_DESC: &str = r#"Used for request body fields, CLI flags, environment variables, schema docs — anywhere each row is "name + what it does." No <code class="mono text-[12px]">&lt;thead&gt;</code> because column labels would just repeat what the cells already show. Left column auto-sizes with <code class="mono text-[12px]">whitespace-nowrap</code>: mono identifier in <code class="mono text-[12px]">ink-900</code> (with optional <span class="mono text-ink-400">&lt;placeholder&gt;</span> in <code class="mono text-[12px]">ink-400</code>), then a meta slot on the same baseline holding either an 11px <code class="mono text-[12px]">ink-500</code> qualifier (<code class="mono text-[12px]">string · required</code>, <code class="mono text-[12px]">path</code>) or enum values as small mono chips (<code class="mono text-[12px]">bg-surfaceMuted px-1 py-0.5 rounded-sm</code>). Right column is reading-flow prose at <code class="mono text-[12px]">ink-700</code>."#;

const TAB_DESC: &str = r#"Used when columns warrant labels — error / response code reference, telemetry, anything with multiple typed columns. Headers are quiet: <code class="mono text-[12px]">text-ink-400 font-normal</code> at the body type size, same case as the data — they label columns without shouting. The first body row's <code class="mono text-[12px]">border-t-[1.5px] border-lineSoft</code> is the only rule between header and body, identical to the row separators below it. Numeric columns get <code class="mono text-[12px]">tabular-nums text-right</code>; zeros and N/A drop to <code class="mono text-[12px]">ink-400</code>; negatives or out-of-band values use <code class="mono text-[12px]">text-negative</code>; totals use <code class="mono text-[12px]">font-medium</code>. Categorical leading columns use the <code class="mono text-[12px]">.id-http-status</code> pill family in a fixed-width column so the next column aligns down the page."#;

/// Render this section.
pub(crate) fn render(
    section_id: &str,
    num: &str,
    title: &str,
    desc: &str,
    tab_entries: &[TabEntry],
) -> String {
    let content = Division::builder()
        .class("space-y-12")
        .division(|d| {
            d.division(|l| {
                l.class("text-[12px] text-ink-500 mb-3")
                    .text("Definition \u{00b7} identifier and meaning")
            })
            .division(|w| {
                w.class("overflow-x-auto border-t-[1.5px] border-lineSoft")
                    .push(build_def_table())
            })
            .paragraph(|p| p.class("mt-3 text-[12px] text-ink-500").text(DEF_DESC))
        })
        .division(|d| {
            d.division(|l| {
                l.class("text-[12px] text-ink-500 mb-3")
                    .text("Tabular \u{00b7} labeled columns with status and metrics")
            })
            .division(|w| {
                w.class("overflow-x-auto border-t-[1.5px] border-lineSoft")
                    .push(build_tab_table(tab_entries))
            })
            .paragraph(|p| p.class("mt-3 text-[12px] text-ink-500").text(TAB_DESC))
        })
        .build()
        .to_string();

    super::section(section_id, num, title, desc, &content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot() {
        insta::assert_snapshot!(crate::components::ds::pretty_html(&render(
            "table",
            "11",
            "Table",
            r##"Two patterns cover everything: a <strong>definition</strong> table (no <code class="mono text-[12px]">&lt;thead&gt;</code>, identifier on the left, meaning on the right) and a <strong>tabular</strong> table (labeled columns, <code class="mono text-[12px]">tabular-nums</code> for figures). 13px body, 1.5px soft row separators (<code class="mono text-[12px]">border-lineSoft</code>), <code class="mono text-[12px]">py-3</code> rows. When the leading column is a category, use the <a href="#c-item-details" class="text-ink-700 underline decoration-line decoration-1 underline-offset-[3px] hover:text-ink-900">.id-http-status</a> pill family."##,
            TAB_ENTRIES,
        )));
    }
}
