(function () {
  var STORAGE_KEY = 'theme';

  function getStoredTheme() {
    return localStorage.getItem(STORAGE_KEY);
  }

  function getSystemDark() {
    return window.matchMedia('(prefers-color-scheme: dark)').matches;
  }

  function isDark() {
    var stored = getStoredTheme();
    if (stored === 'dark') return true;
    if (stored === 'light') return false;
    return getSystemDark();
  }

  function updateToggleButton() {
    var btn = document.getElementById('theme-toggle');
    if (!btn) return;
    var dark = isDark();
    btn.setAttribute('aria-label', dark ? 'Switch to light mode' : 'Switch to dark mode');
    btn.setAttribute('aria-pressed', dark ? 'true' : 'false');
  }

  function applyTheme() {
    document.documentElement.classList.toggle('dark-mode', isDark());
    updateToggleButton();
  }

  function toggleTheme() {
    localStorage.setItem(STORAGE_KEY, isDark() ? 'light' : 'dark');
    applyTheme();
  }

  function init() {
    applyTheme();

    var btn = document.getElementById('theme-toggle');
    if (btn) btn.addEventListener('click', toggleTheme);

    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function () {
      if (!getStoredTheme()) applyTheme();
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
