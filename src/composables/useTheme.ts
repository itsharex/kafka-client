import { invoke } from "@tauri-apps/api/core";
import { useColorMode } from "@vueuse/core";
import { computed, ref, watchEffect } from "vue";

export type Theme = "auto" | "light" | "dark";

export function useTheme() {
  const mode = useColorMode();
  const _theme = ref<Theme>("auto");
  invoke("plugin:theme|get_theme").then((res: any) => {
    _theme.value = res as Theme;
  });
  const setTheme = (t: Theme) => {
    invoke("plugin:theme|set_theme", { theme: t }).then(() => {
      _theme.value = t;
    });
  };
  watchEffect(() => {
    mode.value = _theme.value;
  });
  // readonly theme for external use
  const theme = computed(() => _theme.value);

  return { theme, setTheme };
}
