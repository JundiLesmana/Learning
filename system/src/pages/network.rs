use axum::response::Html;
use crate::pages::layout::layout;
use crate::pages::network_topology::{
    topology_css, topology_markup, topology_core_script, topology_ui_script,
    topology_scene_script,
};
use crate::pages::network_monitoring::monitoring_html;
use crate::pages::network_defense::defense_html;
use crate::pages::network_troubleshooting::troubleshooting_html;

pub async fn handler() -> Html<String> {
    let body = format!(r##"
{topology_css}

{topology_markup}

<script src="https://unpkg.com/three@0.160.0/build/three.min.js"></script>

{topology_core_script}

{topology_ui_script}

{monitoring_html}

{defense_html}

{troubleshooting_html}

</main>
</div>

{topology_scene_script}
"##,
        topology_css = topology_css(),
        topology_markup = topology_markup(),
        topology_core_script = topology_core_script(),
        topology_ui_script = topology_ui_script(),
        monitoring_html = monitoring_html(),
        defense_html = defense_html(),
        troubleshooting_html = troubleshooting_html(),
        topology_scene_script = topology_scene_script(),
    );
    Html(layout("Jaringan Komputer", "network", "/Learning/", &body))
}