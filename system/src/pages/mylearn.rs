use axum::response::Html;
use crate::pages::layout::layout;

pub async fn handler() -> Html<String> {
    let body = r#"
<style>
  .mylearn-layout {
    display: flex;
    min-height: calc(100vh - 96px);
    max-width: 980px;
    margin: 0 auto;
    padding: 32px 24px 64px;
    gap: 24px;
  }

  /* Sidebar */
  .sidebar {
    width: 240px;
    flex-shrink: 0;
  }

  .sidebar-inner {
    position: sticky;
    top: 88px;
    padding: 20px 16px;
    border-radius: var(--radius-lg);
  }

  .sidebar-title {
    font-size: 10.5px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 12px;
    padding: 0 8px;
  }

  .sidebar-nav {
    list-style: none;
  }

  .sidebar-nav li + li {
    margin-top: 2px;
  }

  .sidebar-section-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
    padding: 12px 8px 4px;
    display: block;
  }

  .sidebar-nav a {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 10px;
    border-radius: var(--radius-sm);
    text-decoration: none;
    color: var(--text-secondary);
    font-size: 13px;
    font-weight: 500;
    transition: color var(--transition), background var(--transition);
  }

  .sidebar-nav a:hover {
    color: var(--text-primary);
    background: var(--glass-hover);
  }

  .sidebar-nav a.active {
    color: var(--accent);
    background: var(--accent-soft);
  }

  .sidebar-nav a .dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: currentColor;
    flex-shrink: 0;
    opacity: 0.6;
  }

  /* Main content */
  .mylearn-content {
    flex: 1;
    min-width: 0;
  }

  .content-header {
    margin-bottom: 36px;
  }

  .content-header .label {
    margin-bottom: 14px;
  }

  .content-header h1 {
    margin-bottom: 10px;
  }

  .content-header p {
    max-width: 540px;
    font-size: 14px;
    line-height: 1.75;
  }

  .learn-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 16px;
  }

  .learn-item {
    display: block;
    text-decoration: none;
    padding: 24px 22px;
    border-radius: var(--radius-lg);
    transition: transform var(--transition), background var(--transition);
  }

  .learn-item:hover {
    transform: translateY(-3px);
    background: var(--glass-hover);
  }

  .learn-item-num {
    font-size: 11px;
    font-weight: 700;
    color: var(--accent);
    letter-spacing: 0.08em;
    text-transform: uppercase;
    margin-bottom: 10px;
  }

  .learn-item h3 {
    font-size: 15px;
    margin-bottom: 8px;
  }

  .learn-item p {
    font-size: 13px;
    line-height: 1.65;
    color: var(--text-secondary);
  }

  .learn-item-meta {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 18px;
    padding-top: 14px;
    border-top: 1px solid var(--glass-border);
  }

  .meta-badge {
    padding: 3px 10px;
    border-radius: 100px;
    background: var(--accent-soft);
    border: 1px solid var(--accent-border);
    color: var(--accent);
    font-size: 11px;
    font-weight: 600;
  }

  .meta-time {
    font-size: 12px;
    color: var(--text-muted);
  }

  @media (max-width: 680px) {
    .mylearn-layout { flex-direction: column; }
    .sidebar { width: 100%; }
    .sidebar-inner { position: static; }
  }
</style>

<div class="mylearn-layout">
  <!-- Sidebar -->
  <aside class="sidebar">
    <div class="sidebar-inner glass">
      <div class="sidebar-title">Pembelajaran</div>
      <ul class="sidebar-nav">
        <span class="sidebar-section-label">Algoritma Sorting</span>
        <li>
          <a href="/Learning/Insert-Sorting/">
            <span class="dot"></span>
            Insertion Sort
          </a>
        </li>
        <li>
          <a href="/Learning/Bubble-Sorting/">
            <span class="dot"></span>
            Bubble Sort
          </a>
        </li>
      </ul>
    </div>
  </aside>

  <!-- Main -->
  <main class="mylearn-content">
    <div class="content-header">
      <div class="label">MyLearn</div>
      <h1 style="margin-top:16px;">Daftar Pembelajaran</h1>
      <p style="margin-top:12px;">
        Kumpulan topik yang telah saya pelajari, lengkap dengan simulasi interaktif
        dan penjelasan konsep. Pilih topik dari daftar di bawah atau gunakan sidebar.
      </p>
    </div>

    <div class="learn-grid">
      <a href="/Learning/Insert-Sorting/" class="learn-item glass">
        <div class="learn-item-num">01 &mdash; Sorting</div>
        <h3>Insertion Sort</h3>
        <p>Algoritma pengurutan yang membangun array terurut satu elemen per iterasi dengan cara menyisipkan elemen pada posisi yang tepat.</p>
        <div class="learn-item-meta">
          <span class="meta-badge">Interaktif</span>
          <span class="meta-time">Simulasi tersedia</span>
        </div>
      </a>

      <a href="/Learning/Bubble-Sorting/" class="learn-item glass">
        <div class="learn-item-num">02 &mdash; Sorting</div>
        <h3>Bubble Sort</h3>
        <p>Algoritma pengurutan yang membandingkan dan menukar elemen yang berdekatan secara berulang hingga seluruh array terurut.</p>
        <div class="learn-item-meta">
          <span class="meta-badge">Interaktif</span>
          <span class="meta-time">Simulasi tersedia</span>
        </div>
      </a>
    </div>
  </main>
</div>
"#;
    Html(layout("MyLearn", "mylearn", "/Learning/", body))
}
