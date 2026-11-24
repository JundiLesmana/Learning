// Shared layout helper used by all pages
pub fn layout(title: &str, active: &str, body: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="id" data-theme="dark">
<head>
  <meta charset="UTF-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>{title} — Learning Notes</title>
  <link rel="preconnect" href="https://fonts.googleapis.com"/>
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet"/>
  <style>
    /* =============================================
       CSS CUSTOM PROPERTIES — LIQUID GLASS THEME
    ============================================= */
    :root {{
      --radius-sm: 8px;
      --radius-md: 14px;
      --radius-lg: 22px;
      --radius-xl: 32px;
      --transition: 240ms cubic-bezier(0.4, 0, 0.2, 1);
    }}

    [data-theme="dark"] {{
      --bg-base:        #0a0a0f;
      --bg-layer:       #101018;
      --bg-elevated:    #16161f;
      --glass-bg:       rgba(255,255,255,0.045);
      --glass-border:   rgba(255,255,255,0.09);
      --glass-hover:    rgba(255,255,255,0.08);
      --glass-active:   rgba(255,255,255,0.12);
      --glass-blur:     blur(28px) saturate(1.6);
      --text-primary:   #f0f0f4;
      --text-secondary: #9898b0;
      --text-muted:     #55556a;
      --accent:         #6c8eff;
      --accent-soft:    rgba(108,142,255,0.14);
      --accent-border:  rgba(108,142,255,0.28);
      --success:        #4ade80;
      --warning:        #fbbf24;
      --shadow-sm:      0 2px 8px rgba(0,0,0,0.5);
      --shadow-md:      0 4px 24px rgba(0,0,0,0.6);
      --shadow-lg:      0 8px 48px rgba(0,0,0,0.7);
      --glow:           0 0 40px rgba(108,142,255,0.08);
    }}

    [data-theme="light"] {{
      --bg-base:        #f2f2f7;
      --bg-layer:       #ffffff;
      --bg-elevated:    #f9f9fc;
      --glass-bg:       rgba(255,255,255,0.65);
      --glass-border:   rgba(0,0,0,0.08);
      --glass-hover:    rgba(255,255,255,0.80);
      --glass-active:   rgba(255,255,255,0.90);
      --glass-blur:     blur(28px) saturate(1.8);
      --text-primary:   #1a1a2e;
      --text-secondary: #5a5a72;
      --text-muted:     #9999b2;
      --accent:         #3a6bdf;
      --accent-soft:    rgba(58,107,223,0.10);
      --accent-border:  rgba(58,107,223,0.22);
      --success:        #16a34a;
      --warning:        #d97706;
      --shadow-sm:      0 2px 8px rgba(0,0,0,0.08);
      --shadow-md:      0 4px 24px rgba(0,0,0,0.10);
      --shadow-lg:      0 8px 48px rgba(0,0,0,0.12);
      --glow:           0 0 40px rgba(58,107,223,0.06);
    }}

    /* =============================================
       RESET & BASE
    ============================================= */
    *, *::before, *::after {{ box-sizing: border-box; margin: 0; padding: 0; }}

    html {{ scroll-behavior: smooth; }}

    body {{
      font-family: 'Inter', system-ui, -apple-system, sans-serif;
      background: var(--bg-base);
      color: var(--text-primary);
      min-height: 100vh;
      line-height: 1.6;
      font-size: 15px;
      transition: background var(--transition), color var(--transition);
      -webkit-font-smoothing: antialiased;
    }}

    /* =============================================
       NAVBAR
    ============================================= */
    .navbar {{
      position: fixed;
      top: 16px;
      left: 50%;
      transform: translateX(-50%);
      width: calc(100% - 48px);
      max-width: 980px;
      z-index: 1000;
      background: var(--glass-bg);
      border: 1px solid var(--glass-border);
      backdrop-filter: var(--glass-blur);
      -webkit-backdrop-filter: var(--glass-blur);
      border-radius: var(--radius-xl);
      padding: 0 20px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      height: 56px;
      box-shadow: var(--shadow-md), var(--glow);
      transition: background var(--transition), border-color var(--transition), box-shadow var(--transition);
    }}

    .navbar-brand {{
      font-weight: 700;
      font-size: 15px;
      color: var(--text-primary);
      text-decoration: none;
      letter-spacing: -0.02em;
    }}

    .navbar-brand span {{
      color: var(--accent);
    }}

    .navbar-nav {{
      display: flex;
      align-items: center;
      gap: 4px;
      list-style: none;
    }}

    .navbar-nav a {{
      display: block;
      padding: 7px 14px;
      border-radius: var(--radius-md);
      text-decoration: none;
      color: var(--text-secondary);
      font-size: 13.5px;
      font-weight: 500;
      transition: color var(--transition), background var(--transition);
      white-space: nowrap;
    }}

    .navbar-nav a:hover {{
      color: var(--text-primary);
      background: var(--glass-hover);
    }}

    .navbar-nav a.active {{
      color: var(--accent);
      background: var(--accent-soft);
    }}

    /* Theme toggle button */
    .btn-theme {{
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 7px;
      padding: 7px 14px;
      border-radius: var(--radius-md);
      border: 1px solid var(--glass-border);
      background: var(--glass-bg);
      color: var(--text-secondary);
      font-size: 12.5px;
      font-weight: 500;
      cursor: pointer;
      transition: all var(--transition);
      white-space: nowrap;
      font-family: inherit;
      letter-spacing: 0.01em;
    }}

    .btn-theme:hover {{
      background: var(--glass-hover);
      color: var(--text-primary);
      border-color: var(--accent-border);
    }}

    .btn-theme svg {{
      width: 14px;
      height: 14px;
      flex-shrink: 0;
      transition: transform var(--transition), opacity var(--transition);
    }}

    /* =============================================
       PAGE WRAPPER
    ============================================= */
    .page-wrapper {{
      padding-top: 96px;
      min-height: 100vh;
    }}

    /* =============================================
       GLASS CARD
    ============================================= */
    .glass {{
      background: var(--glass-bg);
      border: 1px solid var(--glass-border);
      backdrop-filter: var(--glass-blur);
      -webkit-backdrop-filter: var(--glass-blur);
      border-radius: var(--radius-lg);
      transition: background var(--transition), border-color var(--transition);
    }}

    .glass:hover {{
      background: var(--glass-hover);
    }}

    /* =============================================
       CONTAINER
    ============================================= */
    .container {{
      max-width: 980px;
      margin: 0 auto;
      padding: 0 24px;
    }}

    /* =============================================
       TYPOGRAPHY
    ============================================= */
    h1, h2, h3, h4 {{
      line-height: 1.25;
      letter-spacing: -0.025em;
      color: var(--text-primary);
    }}

    h1 {{ font-size: clamp(2rem, 4vw, 3rem); font-weight: 700; }}
    h2 {{ font-size: clamp(1.4rem, 3vw, 2rem); font-weight: 700; }}
    h3 {{ font-size: 1.15rem; font-weight: 600; }}
    h4 {{ font-size: 0.95rem; font-weight: 600; }}

    p {{ color: var(--text-secondary); }}

    .label {{
      display: inline-flex;
      align-items: center;
      gap: 6px;
      padding: 4px 12px;
      border-radius: 100px;
      border: 1px solid var(--accent-border);
      background: var(--accent-soft);
      color: var(--accent);
      font-size: 11.5px;
      font-weight: 600;
      letter-spacing: 0.06em;
      text-transform: uppercase;
    }}

    /* =============================================
       SECTION SPACING
    ============================================= */
    section {{ padding: 64px 0; }}
    .section-sm {{ padding: 40px 0; }}

    /* =============================================
       DIVIDER
    ============================================= */
    .divider {{
      height: 1px;
      background: var(--glass-border);
      border: none;
      margin: 0;
    }}

    /* =============================================
       FOOTER
    ============================================= */
    footer {{
      border-top: 1px solid var(--glass-border);
      padding: 28px 0;
      text-align: center;
      color: var(--text-muted);
      font-size: 12.5px;
    }}

    /* =============================================
       SCROLLBAR
    ============================================= */
    ::-webkit-scrollbar {{ width: 6px; height: 6px; }}
    ::-webkit-scrollbar-track {{ background: transparent; }}
    ::-webkit-scrollbar-thumb {{ background: var(--glass-border); border-radius: 99px; }}
    ::-webkit-scrollbar-thumb:hover {{ background: var(--text-muted); }}

    /* =============================================
       SELECTION
    ============================================= */
    ::selection {{
      background: var(--accent-soft);
      color: var(--accent);
    }}
  </style>
</head>
<body>

<!-- NAVBAR -->
<nav class="navbar" role="navigation" aria-label="Main navigation">
  <a class="navbar-brand" href="./">Learning<span>.</span></a>
  <ul class="navbar-nav">
    <li><a href="./" class="{nav_home}">Beranda</a></li>
    <li><a href="./about/" class="{nav_about}">About</a></li>
    <li><a href="./mylearn/" class="{nav_mylearn}">MyLearn</a></li>
  </ul>
  <button class="btn-theme" onclick="toggleTheme()" id="themeBtn" aria-label="Toggle theme">
    <svg id="iconSun" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
      <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
    </svg>
    <svg id="iconMoon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display:none">
      <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
    </svg>
    <span id="themeLabel">Light</span>
  </button>
</nav>

<!-- CONTENT -->
<div class="page-wrapper">
  {body}
  <footer>
    <div class="container">
      Learning Notes &mdash; {year}
    </div>
  </footer>
</div>

<script>
  // ---- Theme ----
  (function() {{
    var saved = localStorage.getItem('theme') || 'dark';
    applyTheme(saved);
  }})();

  function applyTheme(t) {{
    document.documentElement.setAttribute('data-theme', t);
    localStorage.setItem('theme', t);
    var sun  = document.getElementById('iconSun');
    var moon = document.getElementById('iconMoon');
    var lbl  = document.getElementById('themeLabel');
    if (t === 'dark') {{
      sun.style.display  = 'block';
      moon.style.display = 'none';
      if (lbl) lbl.textContent = 'Light';
    }} else {{
      sun.style.display  = 'none';
      moon.style.display = 'block';
      if (lbl) lbl.textContent = 'Dark';
    }}
  }}

  function toggleTheme() {{
    var current = document.documentElement.getAttribute('data-theme') || 'dark';
    applyTheme(current === 'dark' ? 'light' : 'dark');
  }}

  // ---- active nav ----
  (function() {{
    var links = document.querySelectorAll('.navbar-nav a');
    links.forEach(function(a) {{
      if (a.getAttribute('href') === window.location.pathname) {{
        a.classList.add('active');
      }} else {{
        a.classList.remove('active');
      }}
    }});
  }})();
</script>
</body>
</html>"#,
        title = title,
        nav_home    = if active == "home"    { "active" } else { "" },
        nav_about   = if active == "about"   { "active" } else { "" },
        nav_mylearn = if active == "mylearn" || active == "insert" || active == "bubble" { "active" } else { "" },
        body = body,
        year = "2025"
    )
}
