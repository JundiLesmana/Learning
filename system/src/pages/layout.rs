// Shared layout helper used by all pages
use std::time::{SystemTime, UNIX_EPOCH};

pub fn layout(title: &str, active: &str, base: &str, body: &str) -> String {
    let year = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() / 31_536_000 + 1970;

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
    .loader-overlay {{
      position: fixed;
      inset: 0;
      z-index: 9999;
      background: var(--bg-base);
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      transition: opacity 0.8s ease;
    }}

    .loader-hidden {{
      opacity: 0;
      pointer-events: none;
    }}

    /* 3D Book Character */
    .book-stage {{
      perspective: 1000px;
      width: 260px;
      height: 320px;
      display: flex;
      align-items: center;
      justify-content: center;
      animation: float 4s ease-in-out infinite;
    }}

    @keyframes float {{
      0%, 100% {{ transform: translateY(0); }}
      50% {{ transform: translateY(-15px); }}
    }}

    .book-character {{
      position: relative;
      width: 140px;
      height: 200px;
      transform-style: preserve-3d;
      animation: bounce 3s ease-in-out infinite;
    }}

    @keyframes bounce {{
      0%, 100% {{ transform: rotateX(0) rotateY(0); }}
      25% {{ transform: rotateX(4deg) rotateY(4deg); }}
      75% {{ transform: rotateX(-4deg) rotateY(-4deg); }}
    }}

    .book-part {{
      position: absolute;
      transform-style: preserve-3d;
    }}

    /* Flying book on the LEFT side (replacing blue line glitch) */
    .flying-book {{
      position: absolute;
      left: -90px;
      top: 20px;
      width: 70px;
      height: 100px;
      transform-style: preserve-3d;
      animation: flyBook 5s ease-in-out infinite;
    }}

    @keyframes flyBook {{
      0%, 100% {{ transform: translateX(0) translateY(0) rotateZ(0); }}
      25% {{ transform: translateX(-10px) translateY(-15px) rotateZ(-8deg); }}
      50% {{ transform: translateX(0) translateY(0) rotateZ(0); }}
      75% {{ transform: translateX(10px) translateY(-10px) rotateZ(8deg); }}
    }}

    .book-cover {{
      width: 140px;
      height: 200px;
      background: linear-gradient(135deg, #6c8eff 0%, #3a6bdf 100%);
      border-radius: 6px;
      box-shadow: 0 10px 30px rgba(0,0,0,0.3);
      border: 2px solid rgba(255,255,255,0.2);
      transform: rotateY(0deg) translateZ(7px);
    }}

    /* Flying book cover */
    .flying-cover {{
      width: 70px;
      height: 100px;
      background: linear-gradient(135deg, #6c8eff 0%, #3a6bdf 100%);
      border-radius: 4px;
      box-shadow: 0 5px 15px rgba(0,0,0,0.3);
      border: 1px solid rgba(255,255,255,0.2);
      transform: rotateY(0deg) translateZ(3px);
    }}

    .book-pages {{
      width: 16px;
      height: 200px;
      background: repeating-linear-gradient(
        #fff 0,
        #fff 2px,
        #f0f0f4 2px,
        #f0f0f4 4px
      );
      border-radius: 0 3px 3px 0;
      transform: rotateY(90deg) translateZ(63px);
      box-shadow: -4px 0 15px rgba(0,0,0,0.2);
    }}

    /* Flying book pages */
    .flying-pages {{
      width: 10px;
      height: 100px;
      background: repeating-linear-gradient(
        #fff 0,
        #fff 2px,
        #f0f0f4 2px,
        #f0f0f4 4px
      );
      border-radius: 0 2px 2px 0;
      transform: rotateY(90deg) translateZ(35px);
      box-shadow: -2px 0 8px rgba(0,0,0,0.2);
    }}

    .book-spine {{
      width: 14px;
      height: 200px;
      background: linear-gradient(135deg, #4a6bcf 0%, #2a4daf 100%);
      border-radius: 3px 0 0 3px;
      transform: rotateY(90deg) translateZ(-63px);
    }}

    /* Flying book spine */
    .flying-spine {{
      width: 8px;
      height: 100px;
      background: linear-gradient(135deg, #4a6bcf 0%, #2a4daf 100%);
      border-radius: 2px 0 0 2px;
      transform: rotateY(90deg) translateZ(-35px);
    }}

    .book-top {{
      width: 140px;
      height: 16px;
      background: linear-gradient(135deg, #8aa4ff 0%, #5a7df0 100%);
      transform: rotateX(90deg) translateZ(93px);
      border-radius: 6px 6px 0 0;
    }}

    .book-bottom {{
      width: 140px;
      height: 16px;
      background: linear-gradient(135deg, #4a6bcf 0%, #2a4daf 100%);
      transform: rotateX(-90deg) translateZ(107px);
      border-radius: 0 0 6px 6px;
    }}

    /* Flying book top/bottom */
    .flying-top {{
      width: 70px;
      height: 10px;
      background: linear-gradient(135deg, #8aa4ff 0%, #5a7df0 100%);
      transform: rotateX(90deg) translateZ(48px);
      border-radius: 4px 4px 0 0;
    }}

    .flying-bottom {{
      width: 70px;
      height: 10px;
      background: linear-gradient(135deg, #4a6bcf 0%, #2a4daf 100%);
      transform: rotateX(-90deg) translateZ(52px);
      border-radius: 0 0 4px 4px;
    }}

    /* Face */
    .book-face {{
      position: absolute;
      top: 75px;
      left: 30px;
      width: 80px;
      height: 50px;
      transform: translateZ(14px);
      display: flex;
      flex-direction: column;
      align-items: center;
    }}

    .eye {{
      width: 10px;
      height: 10px;
      background: #fff;
      border-radius: 50%;
      margin: 0 8px;
      position: relative;
      box-shadow: 0 0 8px rgba(255,255,255,0.6);
    }}

    .eye::after {{
      content: '';
      position: absolute;
      top: 2px;
      left: 3px;
      width: 4px;
      height: 4px;
      background: #000;
      border-radius: 50%;
    }}

    .eyes {{
      display: flex;
      justify-content: center;
      gap: 8px;
      margin-bottom: 6px;
    }}

    .smile {{
      width: 24px;
      height: 10px;
      border: 2px solid #fff;
      border-top: none;
      border-radius: 0 0 12px 12px;
    }}

    /* Graduation Cap */
    .grad-cap {{
      position: absolute;
      top: -32px;
      left: 38px;
      width: 64px;
      height: 20px;
      background: #1a1a2e;
      border-radius: 4px 4px 0 0;
      transform: translateZ(22px);
    }}

    .grad-cap-top {{
      position: absolute;
      top: -10px;
      left: 0;
      width: 64px;
      height: 10px;
      background: #1a1a2e;
    }}

    .grad-tassel {{
      position: absolute;
      top: 20px;
      left: 30px;
      width: 4px;
      height: 30px;
      background: #ffd700;
      transform: translateZ(22px);
      animation: swing 2s ease-in-out infinite;
    }}

    @keyframes swing {{
      0%, 100% {{ transform: translateZ(22px) rotate(0deg); }}
      50% {{ transform: translateZ(22px) rotate(15deg); }}
    }}

    /* Limbs */
    .arm {{
      position: absolute;
      width: 8px;
      height: 35px;
      background: linear-gradient(135deg, #6c8eff 0%, #3a6bdf 100%);
      border-radius: 4px;
      transform: translateZ(12px);
    }}

    .arm-left {{
      top: 90px;
      left: -8px;
      transform: rotate(-30deg) translateZ(12px);
    }}

    .arm-right {{
      top: 90px;
      right: -8px;
      transform: rotate(30deg) translateZ(12px);
    }}

    .leg {{
      position: absolute;
      width: 10px;
      height: 30px;
      background: linear-gradient(135deg, #4a6bcf 0%, #2a4daf 100%);
      border-radius: 4px;
      bottom: -30px;
      transform: translateZ(12px);
    }}

    .leg-left {{
      left: 30px;
      transform: rotate(-15deg) translateZ(12px);
    }}

    .leg-right {{
      right: 30px;
      transform: rotate(15deg) translateZ(12px);
    }}

    .loader-text {{
      margin-top: 32px;
      color: var(--text-primary);
      font-weight: 600;
      letter-spacing: 0.08em;
      text-transform: uppercase;
      font-size: 12px;
      animation: pulse 2s ease-in-out infinite;
    }}

    @keyframes pulse {{
      0%, 100% {{ opacity: 1; }}
      50% {{ opacity: 0.5; }}
    }}
  </style>
</head>
<body>

<!-- Loader Overlay -->
<div class="loader-overlay" id="loaderOverlay">
  <div class="book-stage">
    <div class="flying-book">
      <div class="book-part flying-cover"></div>
      <div class="book-part flying-pages"></div>
      <div class="book-part flying-spine"></div>
      <div class="book-part flying-top"></div>
      <div class="book-part flying-bottom"></div>
    </div>
    <div class="book-character">
      <div class="book-part book-cover"></div>
      <div class="book-part book-pages"></div>
      <div class="book-part book-spine"></div>
      <div class="book-part book-top"></div>
      <div class="book-part book-bottom"></div>
      <div class="grad-cap">
        <div class="grad-cap-top"></div>
        <div class="grad-tassel"></div>
      </div>
      <div class="arm arm-left"></div>
      <div class="arm arm-right"></div>
      <div class="leg leg-left"></div>
      <div class="leg leg-right"></div>
      <div class="book-face">
        <div class="eyes">
          <div class="eye"></div>
          <div class="eye"></div>
        </div>
        <div class="smile"></div>
      </div>
    </div>
  </div>
  <div class="loader-text">Selamat Belajar</div>
</div>

<!-- NAVBAR -->
<nav class="navbar" role="navigation" aria-label="Main navigation">
  <a class="navbar-brand" href="{base}">Learning<span>.</span></a>
  <ul class="navbar-nav">
    <li><a href="{base}" class="{nav_home}">Beranda</a></li>
    <li><a href="{base}about/" class="{nav_about}">About</a></li>
    <li><a href="{base}mylearn/" class="{nav_mylearn}">MyLearn</a></li>
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

  // ---- Loader Animation (every page load) ----
  (function() {{
    var overlay = document.getElementById('loaderOverlay');
    setTimeout(function() {{
      overlay.classList.add('loader-hidden');
    }}, 2000);
  }})();
</script>
</body>
</html>"#,
        title = title,
        base = base,
        nav_home    = if active == "home"    { "active" } else { "" },
        nav_about   = if active == "about"   { "active" } else { "" },
        nav_mylearn = if active == "mylearn" || active == "insert" || active == "bubble" || active == "pointer" || active == "statistics" { "active" } else { "" },
        body = body,
        year = year
    )
}
