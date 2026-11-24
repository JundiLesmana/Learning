use axum::response::Html;
use crate::pages::layout::layout;

pub async fn handler() -> Html<String> {
    let body = r#"
<style>
  .hero {
    min-height: calc(100vh - 96px);
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    padding: 20px 24px 0 24px;
    max-width: 980px;
    margin: 0 auto;
  }

  .hero-label {
    margin-bottom: 24px;
  }

  .hero-title {
    margin-bottom: 20px;
  }

  .hero-title span {
    color: var(--accent);
  }

  .hero-desc {
    font-size: 1.05rem;
    max-width: 540px;
    line-height: 1.75;
    margin-bottom: 40px;
  }

  .hero-actions {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
  }

  .btn-primary {
    padding: 11px 24px;
    border-radius: var(--radius-md);
    background: var(--accent);
    color: #fff;
    font-size: 13.5px;
    font-weight: 600;
    text-decoration: none;
    border: none;
    cursor: pointer;
    transition: opacity var(--transition), transform var(--transition);
    display: inline-block;
    font-family: inherit;
  }

  .btn-primary:hover {
    opacity: 0.88;
    transform: translateY(-1px);
  }

  .btn-secondary {
    padding: 11px 24px;
    border-radius: var(--radius-md);
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    color: var(--text-primary);
    font-size: 13.5px;
    font-weight: 600;
    text-decoration: none;
    cursor: pointer;
    transition: background var(--transition), transform var(--transition);
    display: inline-block;
    font-family: inherit;
    backdrop-filter: var(--glass-blur);
    -webkit-backdrop-filter: var(--glass-blur);
  }

  .btn-secondary:hover {
    background: var(--glass-hover);
    transform: translateY(-1px);
  }

  .stats-row {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
    margin-top: 72px;
    padding: 0 24px;
    max-width: 980px;
    margin-left: auto;
    margin-right: auto;
  }

  .stat-card {
    padding: 28px 24px;
    text-align: center;
  }

  .stat-num {
    font-size: 2.2rem;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.03em;
    margin-bottom: 4px;
  }

  .stat-num span {
    color: var(--accent);
  }

  .stat-label {
    font-size: 12.5px;
    color: var(--text-muted);
    font-weight: 500;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .section-title-row {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    margin-bottom: 28px;
  }

  .cards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 16px;
  }

  .learn-card {
    padding: 28px 24px;
    text-decoration: none;
    display: block;
    transition: transform var(--transition), background var(--transition);
  }

  .learn-card:hover {
    transform: translateY(-3px);
    background: var(--glass-hover);
  }

  .learn-card-tag {
    font-size: 11px;
    font-weight: 600;
    color: var(--accent);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    margin-bottom: 10px;
  }

  .learn-card h3 {
    font-size: 1rem;
    margin-bottom: 8px;
  }

  .learn-card p {
    font-size: 13.5px;
    line-height: 1.65;
  }

  .learn-card-footer {
    margin-top: 20px;
    padding-top: 16px;
    border-top: 1px solid var(--glass-border);
    font-size: 12px;
    color: var(--text-muted);
    font-weight: 500;
  }

  @media (max-width: 600px) {
    .stats-row { grid-template-columns: 1fr; }
  }
</style>

<section class="hero">
  <div class="hero-label"><span class="label">Catatan Belajar</span></div>
  <h1 class="hero-title">Dokumentasi Proses<br><span>Pembelajaran</span> Saya</h1>
  <p class="hero-desc">
    Ruang dokumentasi perjalanan belajar — algoritma, struktur data, dan konsep
    pemrograman yang saya pelajari, lengkap dengan simulasi interaktif.
  </p>
  <div class="hero-actions">
    <a href="/Learning/mylearn/" class="btn-primary">Lihat Pembelajaran</a>
  </div>
</section>

<section>
  <div class="container">
    <div class="section-title-row">
      <div>
        <span class="label" style="margin-bottom:12px;display:inline-flex;">Materi Terbaru</span>
        <h2 style="margin-top:12px;">Topik Pembelajaran</h2>
      </div>
      <a href="/Learning/mylearn/" class="btn-secondary" style="font-size:12.5px;padding:8px 18px;">Lihat Semua</a>
    </div>
    <div class="cards-grid">
      <a href="/Learning/Insert-Sorting/" class="learn-card glass">
        <div class="learn-card-tag">Algoritma / Sorting</div>
        <h3>Insertion Sort</h3>
        <p>Visualisasi animasi proses pengurutan data dengan prinsip insertion sort, dilengkapi simulasi input kustom.</p>
        <div class="learn-card-footer">Interaktif &mdash; Input bebas</div>
      </a>
      <a href="/Learning/Bubble-Sorting/" class="learn-card glass">
        <div class="learn-card-tag">Algoritma / Sorting</div>
        <h3>Bubble Sort</h3>
        <p>Visualisasi animasi proses pengurutan data dengan prinsip bubble sort, dilengkapi simulasi input kustom.</p>
        <div class="learn-card-footer">Interaktif &mdash; Input bebas</div>
      </a>
    </div>
  </div>
</section>
"#;
    Html(layout("Beranda", "home", "/Learning/", body))
}
