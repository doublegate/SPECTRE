import { create } from 'zustand';
import type { AppConfig } from '../types/settings';

interface SettingsState {
  config: AppConfig | undefined;
  loading: boolean;
  error: string | undefined;
  setConfig: (config: AppConfig) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | undefined) => void;
  clearError: () => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  config: undefined,
  loading: false,
  error: undefined,
  setConfig: (config) => set({ config, error: undefined }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error, loading: false }),
  clearError: () => set({ error: undefined }),
}));
