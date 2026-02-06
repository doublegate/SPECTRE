import { create } from "zustand";

export type ThemeName = "dark" | "light" | "tactical" | "matrix" | "hacker";

interface UiState {
  theme: ThemeName;
  sidebarCollapsed: boolean;
  activeModal: string | null;

  setTheme: (theme: ThemeName) => void;
  toggleSidebar: () => void;
  setSidebarCollapsed: (collapsed: boolean) => void;
  openModal: (id: string) => void;
  closeModal: () => void;
}

export const useUiStore = create<UiState>((set) => ({
  theme: "dark",
  sidebarCollapsed: false,
  activeModal: null,

  setTheme: (theme) => {
    document.documentElement.setAttribute("data-theme", theme);
    set({ theme });
  },

  toggleSidebar: () =>
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),

  setSidebarCollapsed: (collapsed) => set({ sidebarCollapsed: collapsed }),

  openModal: (id) => set({ activeModal: id }),
  closeModal: () => set({ activeModal: null }),
}));
