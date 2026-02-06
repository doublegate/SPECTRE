import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useSettingsStore } from '../stores/settingsStore';
import type { AppConfig, ConfigUpdate } from '../types/settings';
import { DEFAULT_CONFIG } from '../types/settings';

export function useSettings() {
  const { config, loading, error, setConfig, setLoading, setError, clearError } =
    useSettingsStore();

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    setLoading(true);
    setError(undefined);

    try {
      const data = await invoke<AppConfig>('get_config');
      setConfig(data);
    } catch (err) {
      console.error('Failed to load config:', err);
      // Fall back to default config on error
      setConfig(DEFAULT_CONFIG);
      setError(err instanceof Error ? err.message : 'Failed to load configuration');
    } finally {
      setLoading(false);
    }
  };

  const updateConfig = async (updates: ConfigUpdate) => {
    setLoading(true);
    setError(undefined);

    try {
      const updated = await invoke<AppConfig>('update_config', { updates });
      setConfig(updated);
    } catch (err) {
      console.error('Failed to update config:', err);
      setError(err instanceof Error ? err.message : 'Failed to update configuration');
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const resetConfig = async () => {
    setLoading(true);
    setError(undefined);

    try {
      const reset = await invoke<AppConfig>('reset_config');
      setConfig(reset);
    } catch (err) {
      console.error('Failed to reset config:', err);
      setError(err instanceof Error ? err.message : 'Failed to reset configuration');
      throw err;
    } finally {
      setLoading(false);
    }
  };

  return {
    config,
    loading,
    error,
    loadConfig,
    updateConfig,
    resetConfig,
    clearError,
  };
}
