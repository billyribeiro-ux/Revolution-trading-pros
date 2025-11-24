import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'light' | 'dark' | 'auto';

function createThemeStore() {
  const defaultTheme: Theme = 'light';
  
  // Get initial theme from localStorage or default
  const initialTheme = browser 
    ? (localStorage.getItem('theme') as Theme) || defaultTheme
    : defaultTheme;

  const { subscribe, set, update } = writable<Theme>(initialTheme);

  // Apply theme to document
  function applyTheme(theme: Theme) {
    if (!browser) return;

    const root = document.documentElement;
    
    if (theme === 'auto') {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      root.classList.toggle('dark', prefersDark);
    } else {
      root.classList.toggle('dark', theme === 'dark');
    }
  }

  // Initialize theme
  if (browser) {
    applyTheme(initialTheme);

    // Listen for system theme changes
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    mediaQuery.addEventListener('change', () => {
      const currentTheme = localStorage.getItem('theme') as Theme;
      if (currentTheme === 'auto') {
        applyTheme('auto');
      }
    });
  }

  return {
    subscribe,
    setTheme: (theme: Theme) => {
      if (browser) {
        localStorage.setItem('theme', theme);
        applyTheme(theme);
      }
      set(theme);
    },
    toggle: () => {
      update(current => {
        const newTheme: Theme = current === 'light' ? 'dark' : 'light';
        if (browser) {
          localStorage.setItem('theme', newTheme);
          applyTheme(newTheme);
        }
        return newTheme;
      });
    },
  };
}

export const themeStore = createThemeStore();
