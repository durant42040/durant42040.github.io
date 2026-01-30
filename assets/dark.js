(function () {
  function applyDarkMode() {
    var hour = new Date().getHours();
    var isDark = hour >= 18 || hour < 6;
    if (isDark) {
      document.documentElement.classList.add('dark-mode');
    } else {
      document.documentElement.classList.remove('dark-mode');
    }
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyDarkMode);
  } else {
    applyDarkMode();
  }
})();
